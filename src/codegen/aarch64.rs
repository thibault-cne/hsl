use std::io;

use crate::codegen;
use crate::ir::{Expr, Lit, Stmt};

pub struct Compiler<'prog, W> {
    // Inputs
    output_path: &'static str,
    quiet: bool,
    run: bool,

    writer: W,

    // State of the compiler
    string_literals: Vec<(&'prog str, &'prog str)>,
    curr_var_id: &'prog str,
    fmt_str_cpt: usize,
}

impl<'prog, W: io::Write> Compiler<'prog, W> {
    pub fn new(output_path: &'static str, quiet: bool, run: bool, writer: W) -> Self {
        Self {
            output_path,
            quiet,
            run,

            writer,

            string_literals: Vec::new(),
            curr_var_id: "",
            fmt_str_cpt: 0,
        }
    }
}

impl<'prog, W: io::Write> codegen::Compiler<'prog> for Compiler<'prog, W> {
    fn generate_program(
        &mut self,
        program: &'prog crate::ir::Program,
        slt: &'prog crate::parser::slt::NavigableSlt<'prog>,
        cmd: &mut crate::command::Cmd,
    ) -> codegen::error::Result<()> {
        map_err! {
            write!(self.writer, ".global _main\n.align 2\n_main:\n");
            write!(self.writer, "    // program header\n");
            write!(self.writer, "    stp x29, lr, [sp, -0x10]!\n");
            write!(self.writer, "    mov x29, sp\n");
            write!(self.writer, "\n");
            write!(self.writer, "    // jump to the main function\n");
            write!(self.writer, "    b _program\n");
            write!(self.writer, "_end:\n");
        };

        // TODO: check this unwrap
        let child = slt.childs().next().unwrap();
        let mut program_childs = child.childs();
        let stack_size = child.slt.variables.len() * 16;

        map_err! {
            write!(self.writer, "    // pop the stack\n");
            write!(self.writer, "    add sp, sp, {:#02x} // deallocating {} variables\n", stack_size, stack_size / 16);
            write!(self.writer, "    ldp x29, lr, [sp], 0x10\n");
            write!(self.writer, "    mov x0, #0\n");
            write!(self.writer, "    mov x16, #1\n");
            write!(self.writer, "    svc 0\n");
            write!(self.writer, "\n");
            write!(self.writer, "_program:\n");
        }

        for stmt in program.stmts.iter() {
            self.generate_stmt(stmt, &slt.childs().next().unwrap(), &mut program_childs)?;
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

        cmd_append!(cmd, "as", "-o", "test.o", self.output_path);
        let _ = cmd.run_and_reset();
        cmd_append!(cmd, "cc", "-arch", "arm64", "-o", "test", self.output_path);
        let _ = cmd.run_and_reset();

        Ok(())
    }

    fn run_program(&mut self, cmd: &mut crate::command::Cmd) -> codegen::error::Result<()> {
        todo!()
    }
}

impl<'prog, W: io::Write> Compiler<'prog, W> {
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
            let mut arg_cpt = 0;
            for arg in args.iter() {
                self.generate_expr(&arg, slt)?;
                map_err! {
                    // Load the argument onto the stack to allow printf to unstack them and print
                    write!(self.writer, "    // load x8 onto the stack\n");
                    write!(self.writer, "    str x8, [sp, {:#02x}]\n", arg_cpt);
                    write!(self.writer, "\n");
                }
                arg_cpt += 8;
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
        let mut sb = String::new();

        args.iter().enumerate().for_each(|(i, arg)| {
            match arg {
                Expr::Lit(lit) => match lit {
                    Lit::Int(_) => sb.push_str("%d"),
                    Lit::Str(_) => sb.push_str("%s"),
                    Lit::Bool(_) => sb.push_str("%d"),
                },
                Expr::ID(id) => {
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
        map_err! {
            write!(self.writer, "    // pushing variable {} to the stack\n", id);
        };

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
                    write!(self.writer, "    ldr x8, [x9, {}]\n", var.offset);
                    write!(self.writer, "\n")
                }
            }
        }
    }

    fn generate_lit(&mut self, lit: &'prog Lit) -> codegen::error::Result<()> {
        use Lit::*;
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
        map_err! {
            write!(self.writer, "    str x8, [sp, -0x10]!\n")
        }
    }
}
