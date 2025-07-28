use std::io;

use crate::codegen;
use crate::ir::{self, Expr, Fn, Lit, Stmt};

pub struct Codegen<'prog, W> {
    // Inputs
    output_path: &'prog str,
    object_path: &'prog str,
    program_path: &'prog str,
    quiet: bool,
    run: bool,

    writer: W,

    // State of the compiler
    string_literals: Vec<(&'prog str, &'prog str)>,
    curr_var_id: &'prog str,
    fmt_str_cpt: usize,
    has_stack_room: bool,
}

impl<'prog, W: io::Write> Codegen<'prog, W> {
    pub fn new(c: &'prog crate::compiler::Compiler, writer: W) -> Self {
        Self {
            output_path: c.output_path,
            object_path: c.object_path,
            program_path: c.program_path,
            quiet: c.flags.quiet,
            run: c.flags.run,

            writer,

            string_literals: Vec::new(),
            curr_var_id: "",
            fmt_str_cpt: 0,
            has_stack_room: false,
        }
    }
}

impl<'prog, W: io::Write> codegen::Codegen<'prog> for Codegen<'prog, W> {
    fn generate_program(
        &mut self,
        program: &'prog crate::ir::Program,
        slt: &'prog crate::parser::slt::NavigableSlt<'prog>,
        childs: &mut crate::parser::slt::ChildIterator<'prog>,
        cmd: &mut crate::command::Cmd<'prog>,
    ) -> codegen::error::Result<()> {
        map_err! {
            write!(self.writer, ".global _main\n.align 2\n_main:\n");
            write!(self.writer, "    // program header\n");
            write!(self.writer, "    stp x29, lr, [sp, -0x10]!\n");
            write!(self.writer, "    mov x29, sp\n");
            write!(self.writer, "\n");
            write!(self.writer, "    // jump to the main function\n");
            write!(self.writer, "    b _galaxy\n");
            write!(self.writer, "_end:\n");
        };

        // Allocated stack is actually the smallest multiple of 16 greater than 8 times the number of variables
        let var_size = slt.variables.len() * 8;
        let stack_size = crate::math::smallest_multiple_greater_than(16, var_size as _);

        map_err! {
            write!(self.writer, "    // pop the stack\n");
            write!(self.writer, "    add sp, sp, {:#02x} // deallocating {} variables\n", stack_size, var_size / 8);
            write!(self.writer, "    ldp x29, lr, [sp], 0x10\n");
            write!(self.writer, "    mov x0, #0\n");
            write!(self.writer, "    mov x16, #1\n");
            write!(self.writer, "    svc 0\n");
            write!(self.writer, "\n");
        }

        for func in program.func.iter() {
            self.generate_fn_decl(func, slt, childs)?;
        }

        map_err! {
            write!(self.writer, "    // jump to the end of the program\n");
            write!(self.writer, "    b _end\n");
            write!(self.writer, ".data\n");
        }

        for (name, s) in self.string_literals.iter() {
            map_err! {
                write!(self.writer, "    {}:\n        .asciz \"{}\"\n", name, s);
            }
        }

        // Ensures that the writer has been flushed
        map_err! {
            self.writer.flush();
        }

        info!("generated {}", self.output_path);
        cmd_append!(cmd, "as", "-o", self.object_path, self.output_path);
        if let Err(e) = cmd.run_and_reset() {
            return Err(new_error!(from e));
        }
        cmd_append!(
            cmd,
            "cc",
            "-arch",
            "arm64",
            "-o",
            self.program_path,
            self.object_path
        );
        if let Err(e) = cmd.run_and_reset() {
            return Err(new_error!(from e));
        }

        Ok(())
    }

    fn run_program(&mut self, cmd: &mut crate::command::Cmd) -> codegen::error::Result<()> {
        todo!()
    }
}

impl<'prog, W: io::Write> Codegen<'prog, W> {
    fn generate_fn_decl(
        &mut self,
        func: &'prog Fn,
        slt: &crate::parser::slt::NavigableSlt<'prog>,
        childs: &mut crate::parser::slt::ChildIterator<'prog>,
    ) -> codegen::error::Result<()> {
        // TODO: handle the stack for function call
        map_err! {
            write!(self.writer, "_{}:\n", func.id);
        }

        let child = childs.next().unwrap();
        let mut fn_childs = child.childs();

        for stmt in func.stmts.iter() {
            self.generate_stmt(stmt, &child, &mut fn_childs)?;
        }

        Ok(())
    }

    fn generate_stmt(
        &mut self,
        stmt: &'prog Stmt,
        slt: &crate::parser::slt::NavigableSlt<'prog>,
        childs: &mut crate::parser::slt::ChildIterator<'_>,
    ) -> codegen::error::Result<()> {
        use Stmt::*;

        match stmt {
            Let { id, value } => self.generate_let_stmt(id, value, slt),
            FnCall { id, args } => self.generate_fn_call(id, args, slt),
            Assign { id, ops } => {
                todo!("implement assign stmt");
            }
        }
    }

    fn generate_fn_call(
        &mut self,
        id: &'prog str,
        args: &'prog [Expr],
        slt: &crate::parser::slt::NavigableSlt<'prog>,
    ) -> codegen::error::Result<()> {
        // Allocate stack space for arguments on the stack
        let needed_space = (args.len() / 16 + 1) * 16;
        map_err! {
            write!(self.writer, "    // allocate needed stack space for print arguments\n");
            write!(self.writer, "    str x8, [sp, -{:#02x}]!\n", needed_space);
        }

        if id == "print" {
            let mut arg_offset = 0;
            for arg in args.iter() {
                self.generate_expr(&arg, slt)?;
                map_err! {
                    // Load the argument onto the stack to allow printf to unstack them and print
                    write!(self.writer, "    // load x8 onto the stack\n");
                    write!(self.writer, "    str x8, [sp, {:#02x}]\n", arg_offset);
                    write!(self.writer, "\n");
                }
                arg_offset += 8;
            }

            // Generate the format string
            self.generate_fmt_str(&args, slt)?;

            map_err! {
                // Call prinf
                write!(self.writer, "    bl _printf\n");
                // Unstack the argument
                write!(self.writer, "    add sp, sp, 0x10\n");
                write!(self.writer, "\n");
            }

            return Ok(());
        }

        todo!("implement fn call stmt");
    }

    /// Generate fmt str for a printf call and store it in the str literals
    /// it also loads the fmt str to x0 register
    fn generate_fmt_str(
        &mut self,
        args: &'prog [Expr],
        slt: &crate::parser::slt::NavigableSlt<'prog>,
    ) -> codegen::error::Result<()> {
        use ir::Lit::*;
        use Expr::*;

        let mut sb = String::new();

        args.iter().enumerate().for_each(|(i, arg)| {
            match arg {
                FnCall { .. } => todo!("handle fn return type"),
                Lit(lit) => match lit {
                    Int(_) => sb.push_str("%d"),
                    Str(_) => sb.push_str("%s"),
                    Bool(_) => sb.push_str("%d"),
                },
                ID(id) => {
                    // TODO: handle this unwrap
                    let var = slt.find_variable(id).unwrap();

                    use crate::parser::slt::Value;
                    match var.value {
                        Value::Int(_) => sb.push_str("%d"),
                        Value::Str(_) => sb.push_str("%s"),
                        Value::Bool(_) => sb.push_str("%d"),
                    }
                }
            }

            if i != args.len() - 1 {
                sb.push(' ');
            } else {
                sb.push_str("\\n");
            }
        });

        let format_name = format!("__fmt_str_{}", self.fmt_str_cpt);

        map_err! {
            write!(self.writer, "    adrp x0, {}@PAGE\n", format_name);
            write!(self.writer, "    add x0, x0, {}@PAGEOFF\n", format_name);
        }

        self.string_literals.push((format_name.leak(), sb.leak()));
        self.fmt_str_cpt += 1;
        Ok(())
    }

    fn generate_let_stmt(
        &mut self,
        id: &'prog str,
        value: &'prog Expr,
        slt: &crate::parser::slt::NavigableSlt<'prog>,
    ) -> codegen::error::Result<()> {
        self.curr_var_id = id;
        self.generate_expr(value, slt)
    }

    fn generate_expr(
        &mut self,
        expr: &'prog Expr,
        slt: &crate::parser::slt::NavigableSlt<'prog>,
    ) -> codegen::error::Result<()> {
        use Expr::*;
        match expr {
            FnCall { id, args } => self.generate_fn_call(id, args, slt),
            Lit(lit) => self.generate_lit(lit),
            ID(id) => {
                // TODO: handle this unwrap
                let var = slt.find_variable(id).unwrap();
                let diff = slt.slt.scope - var.scope;
                map_err! {
                    write!(self.writer, "    // load var {} into x8\n", id);
                    write!(self.writer, "    mov x9, x29\n");
                };

                for _ in 0..diff {
                    map_err! {
                        write!(self.writer, "    ldr x9, [x9]\n");
                    };
                }
                map_err! {
                    write!(self.writer, "    ldr x8, [x9, -{:#02x}]\n", var.offset * 8);
                    write!(self.writer, "\n")
                }
            }
        }
    }

    fn generate_lit(&mut self, lit: &'prog Lit) -> codegen::error::Result<()> {
        use Lit::*;

        map_err! {
            write!(self.writer, "    // pushing variable {} to x8\n", self.curr_var_id);
        }

        match lit {
            Int(val) => {
                map_err! {
                    write!(self.writer, "    mov x8, #{}\n", val);
                }
            }
            Str(s) => {
                self.string_literals.push((self.curr_var_id, s));
                map_err! {
                    write!(self.writer, "    adrp x8, {}@PAGE\n", self.curr_var_id);
                    write!(self.writer, "    add x8, x8, {}@PAGEOFF\n", self.curr_var_id);
                }
            }
            Bool(b) => {
                map_err! {
                    write!(self.writer, "    mov x8, #{}\n", *b as u8);
                }
            }
        }

        self.write_newline()?;

        if !self.has_stack_room {
            self.has_stack_room = true;
            map_err! {
                // Allocate two spaces and ensures that the stack is 16 aligned
                write!(self.writer, "    // allocate two spaces to the stack and ensures it stays 16 aligned\n");
                write!(self.writer, "    add sp, sp, -0x10\n");
                write!(self.writer, "    // pushing x8 (variable {} to the stack)\n", self.curr_var_id);
                write!(self.writer, "    str x8, [sp, 0x8]\n");
            }
        } else {
            self.has_stack_room = false;
            map_err! {
                write!(self.writer, "    // pushing x8 (variable {} to the stack)\n", self.curr_var_id);
                write!(self.writer, "    str x8, [sp, 0x0]\n");
            }
        }

        self.write_newline()
    }

    fn write_newline(&mut self) -> codegen::error::Result<()> {
        map_err! {
            write!(self.writer, "\n")
        }
    }
}
