use std::io;

use crate::codegen;
use crate::ir::{Arg, Expr, Fn, Lit};

pub struct Codegen<'prog, W> {
    // Inputs
    arena: &'prog crate::arena::Arena<'prog>,
    c: &'prog crate::compiler::Compiler<'prog>,

    writer: W,

    // State of the compiler
    string_literals: Vec<(&'prog str, &'prog str)>,
    curr_var_id: Option<&'prog str>,
    fmt_str_cpt: usize,
}

impl<'prog, W: io::Write> Codegen<'prog, W> {
    pub fn new(c: &'prog crate::compiler::Compiler, writer: W) -> Self {
        Self {
            arena: c.arena,
            c,

            writer,

            string_literals: Vec::new(),
            curr_var_id: None,
            fmt_str_cpt: 0,
        }
    }
}

impl<'prog, W: io::Write> codegen::Codegen<'prog> for Codegen<'prog, W> {
    fn generate_program<'a>(
        &mut self,
        program: &'prog crate::ir::Program,
        _slt: &'a crate::parser::slt::NavigableSlt<'a, 'prog>,
        childs: &mut crate::parser::slt::ChildIterator<'a, 'prog>,
        cmd: &mut crate::command::Cmd<'prog>,
    ) -> codegen::error::Result<()> {
        gen_write!(self.writer, ".global _main\n.p2align 4\n_main:\n")?;
        gen_write!(
            self.writer,
            "    // load link register and previous stack pointer onto the stack\n"
        )?;
        gen_write!(self.writer, "    stp x29, lr, [sp, -0x10]!\n")?;
        gen_write!(self.writer, "    mov x29, sp\n")?;
        gen_write!(self.writer, "\n")?;
        gen_write!(self.writer, "    // jump to the main function\n")?;
        gen_write!(self.writer, "    bl _galaxy\n")?;
        gen_write!(self.writer, "\n")?;
        gen_write!(
            self.writer,
            "    // load return address and previous stack pointer\n"
        )?;
        gen_write!(self.writer, "    ldp x29, lr, [sp], 0x10\n")?;
        gen_write!(self.writer, "    mov x0, #0\n")?;
        gen_write!(self.writer, "    ret\n")?;
        gen_write!(self.writer, "\n")?;

        for (func, slt) in program.func.iter().zip(childs) {
            let mut childs = slt.childs();
            self.generate_fn_decl(func, &slt, &mut childs)?;
        }

        gen_write!(self.writer, ".data\n")?;

        for (name, s) in self.string_literals.iter() {
            gen_write!(self.writer, "    {}:\n        .asciz \"{}\"\n", name, s)?;
        }

        // Ensures that the writer has been flushed
        map_err! {
            self.writer.flush();
        }

        info!("generated {}", self.c.output_path);
        cmd_append!(cmd, "as", "-o", self.c.object_path, self.c.output_path);
        if let Err(e) = cmd.run_and_reset() {
            return Err(new_error!(from e));
        }
        cmd_append!(
            cmd,
            "cc",
            "-arch",
            "arm64",
            "-o",
            self.c.program_path,
            self.c.object_path
        );
        if let Err(e) = cmd.run_and_reset() {
            return Err(new_error!(from e));
        }

        Ok(())
    }

    fn run_program(&mut self, _cmd: &mut crate::command::Cmd) -> codegen::error::Result<()> {
        todo!()
    }
}

impl<'prog, W: io::Write> Codegen<'prog, W> {
    fn generate_fn_decl<'a>(
        &mut self,
        func: &'prog Fn,
        slt: &'a crate::parser::slt::NavigableSlt<'a, 'prog>,
        childs: &mut crate::parser::slt::ChildIterator<'a, 'prog>,
    ) -> codegen::error::Result<()> {
        // TODO: handle the stack for function call
        gen_write!(self.writer, ".global _{}\n.p2align 4\n", func.id)?;
        gen_write!(self.writer, "_{}:\n", func.id)?;
        gen_write!(
            self.writer,
            "    // load link register and previous stack pointer onto the stack\n"
        )?;
        gen_write!(self.writer, "    stp x29, lr, [sp, -0x10]!\n")?;
        gen_write!(self.writer, "    mov x29, sp\n")?;
        gen_write!(self.writer, "\n")?;

        let allocated_stack_size = crate::math::align_bytes(slt.variables.len() * 8, 16);

        if !slt.variables.is_empty() {
            // Allocate variables on the stack and store them
            let reg_args = func.variadic.unwrap_or(if func.args.len() > 7 {
                7
            } else {
                func.args.len()
            });
            let stack_args = func.args.len() - reg_args;

            let allocated_args_space = crate::math::align_bytes(func.args.len() * 8, 16);
            let mut arg_index = 0;

            gen_write!(self.writer, "    // core {} function\n", func.id)?;
            gen_write!(
                self.writer,
                "    // allocate needed stack space for {} arguments\n",
                func.id
            )?;
            gen_write!(
                self.writer,
                "    add sp, sp, -{allocated_stack_size:#02x}\n"
            )?;

            self.write_newline()?;

            gen_write!(
                self.writer,
                "    // load {} arguments onto the stack\n",
                func.id
            )?;

            for i in 0..reg_args {
                arg_index += 1;
                gen_write!(
                    self.writer,
                    "    str x{i}, [x29, -{:#02x}]\n",
                    arg_index * 8
                )?;
            }

            for i in 0..stack_args {
                let var_offset = allocated_args_space + 2 + i;
                arg_index += 1;

                gen_write!(self.writer, "    ldr x8, [x29, -{var_offset:#02x}]\n")?;
                gen_write!(self.writer, "    str x8, [x29, -{:#02x}]\n", arg_index * 8)?;
            }

            self.write_newline()?;
        }

        for expr in func.body.iter() {
            self.generate_expr(expr, slt, childs)?;
        }

        if !slt.variables.is_empty() {
            gen_write!(
                self.writer,
                "    // pop the stack (deallocating {} variables)\n",
                slt.variables.len()
            )?;
            gen_write!(self.writer, "    add sp, sp, {allocated_stack_size:#02x}\n")?;
            gen_write!(self.writer, "\n")?;
        }

        gen_write!(
            self.writer,
            "    // load return address and previous stack pointer\n"
        )?;
        gen_write!(self.writer, "    ldp x29, lr, [sp], 0x10\n")?;
        gen_write!(self.writer, "    ret\n")?;
        gen_write!(self.writer, "\n")
    }

    fn generate_expr<'a>(
        &mut self,
        stmt: &'prog Expr,
        slt: &'a crate::parser::slt::NavigableSlt<'a, 'prog>,
        _childs: &mut crate::parser::slt::ChildIterator<'a, 'prog>,
    ) -> codegen::error::Result<()> {
        use Expr::*;

        match stmt {
            Let { id, value } => self.generate_let(id, value, slt),
            FnCall { id, args } => self.generate_fn_call(id, args, slt),
        }
    }

    fn generate_fn_call<'a>(
        &mut self,
        id: &'prog str,
        args: &'prog [Arg],
        slt: &'a crate::parser::slt::NavigableSlt<'a, 'prog>,
    ) -> codegen::error::Result<()> {
        let variadic = self.c.program.get_fn_variadic(id);

        // This is for macosX variadic are passes onto the stack and other arguments are passed
        // using registers x0 to x7
        let reg_args = variadic.unwrap_or(if args.len() > 7 { 7 } else { args.len() });
        let stack_args = args.len() - reg_args;

        let allocated_space = crate::math::align_bytes(stack_args * 8, 16);

        gen_write!(self.writer, "    // calling {id} function\n")?;

        if reg_args > 0 {
            gen_write!(
                self.writer,
                "    // load {} arguments onto the associated registers\n",
                id
            )?;
            gen_write!(self.writer, "\n")?;

            for (i, arg) in args.iter().enumerate().take(reg_args) {
                self.generate_arg(arg, slt)?;

                // Load the argument onto the associated register
                gen_write!(self.writer, "    // load fn arguments onto x{i}\n")?;
                gen_write!(self.writer, "    mov x{i}, x8\n")?;
                gen_write!(self.writer, "\n")?;
            }
        }

        if stack_args > 0 {
            gen_write!(
                self.writer,
                "    // allocate needed stack space for {id} arguments\n"
            )?;
            gen_write!(self.writer, "    str x8, [sp, -{allocated_space:#02x}]!\n")?;
            gen_write!(self.writer, "\n")?;

            let mut arg_offset = 0;
            for i in 0..stack_args {
                self.generate_arg(&args[reg_args + i], slt)?;

                // Load the argument onto the stack for fn call
                gen_write!(self.writer, "    // load x8 onto the stack\n")?;
                gen_write!(self.writer, "    str x8, [sp, {arg_offset:#02x}]\n")?;
                gen_write!(self.writer, "\n")?;

                arg_offset += 8;
            }
        }

        gen_write!(self.writer, "    // jump to the function\n")?;
        gen_write!(self.writer, "    bl _{id}\n")?;
        gen_write!(self.writer, "\n")?;

        if stack_args > 0 {
            gen_write!(
                self.writer,
                "    // pop from the stack the {id} function arguments\n"
            )?;
            gen_write!(self.writer, "    add sp, sp, {allocated_space:#02x}\n")?;
            gen_write!(self.writer, "\n")?;
        }

        Ok(())
    }

    fn generate_let<'a>(
        &mut self,
        id: &'prog str,
        value: &'prog Arg,
        slt: &crate::parser::slt::NavigableSlt<'a, 'prog>,
    ) -> codegen::error::Result<()> {
        self.curr_var_id = Some(id);
        // Value is loaded inside the x8 register we need to store it on the stack
        self.generate_arg(value, slt)?;
        // SAFETY: this is safe because of the parser (the variable has been pushed to the slt)
        let var = slt.get_variable(id).unwrap();

        gen_write!(
            self.writer,
            "    // pushing x8 (variable {} to the stack)\n",
            // SAFETY: this is safe because we just populated self.curr_var_id
            self.curr_var_id.unwrap()
        )?;
        gen_write!(self.writer, "    str x8, [x29, -{:#02x}]\n", var.offset * 8)?;

        self.curr_var_id = None;
        self.write_newline()
    }

    fn generate_arg<'a>(
        &mut self,
        expr: &'prog Arg,
        slt: &crate::parser::slt::NavigableSlt<'a, 'prog>,
    ) -> codegen::error::Result<()> {
        use Arg::*;

        match expr {
            Lit(lit) => self.generate_lit(lit),
            Id(id) => {
                // SAFETY: this is safe because of the semantic controls
                let var = slt.find_variable(id).unwrap();
                let diff = slt.scope - var.scope;

                gen_write!(self.writer, "    mov x9, x29\n")?;

                if diff > 0 {
                    gen_write!(
                        self.writer,
                        "    // climb up the stack calls to get to {} scope\n",
                        id
                    )?;
                }

                for _ in 0..diff {
                    gen_write!(self.writer, "    ldr x9, [x29]\n")?;
                }

                self.write_newline()?;

                gen_write!(self.writer, "    // load var {} into x8\n", id)?;
                gen_write!(self.writer, "    ldr x8, [x9, -{:#02x}]\n", var.offset * 8)?;
                gen_write!(self.writer, "\n")
            }
        }
    }

    fn generate_lit(&mut self, lit: &'prog Lit) -> codegen::error::Result<()> {
        use Lit::*;

        let curr_id = self.curr_var_id.unwrap_or_else(|| {
            self.fmt_str_cpt += 1;
            self.arena
                .strdup(format!("__lit_{}", self.fmt_str_cpt).as_str())
        });

        match lit {
            Int(val) => {
                gen_write!(self.writer, "    // pushing variable {} to x8\n", curr_id)?;
                gen_write!(self.writer, "    mov x8, #{}\n", val)?;
            }
            Str(s) => {
                // Check if a similar string literal has already been pushed so we avoid duping items
                let lit_str_id = if let Some(name) = self
                    .string_literals
                    .iter()
                    .find(|(_, value)| value == s)
                    .map(|(value, _)| *value)
                {
                    name
                } else {
                    self.string_literals.push((curr_id, s));
                    curr_id
                };

                gen_write!(
                    self.writer,
                    "    // pushing variable {} to x8\n",
                    lit_str_id
                )?;
                gen_write!(self.writer, "    adrp x8, {}@PAGE\n", lit_str_id)?;
                gen_write!(self.writer, "    add x8, x8, {}@PAGEOFF\n", lit_str_id)?;
            }
            Bool(b) => {
                gen_write!(self.writer, "    // pushing variable {} to x8\n", curr_id)?;
                gen_write!(self.writer, "    mov x8, #{}\n", *b as u8)?;
            }
        }

        self.write_newline()
    }

    fn write_newline(&mut self) -> codegen::error::Result<()> {
        gen_write!(self.writer, "\n")
    }
}
