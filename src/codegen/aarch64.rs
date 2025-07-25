use std::io;

use crate::codegen;

pub struct Compiler<W> {
    string_literals: Vec<(String, String)>,
    writer: W,
}

impl<W: io::Write> Compiler<W> {
    pub fn new(writer: W) -> Self {
        Self {
            string_literals: Vec::new(),
            writer,
        }
    }
}

impl<W: io::Write> codegen::Compiler for Compiler<W> {
    fn generate_program(
        &mut self,
        program: &crate::ir::Program,
        slt: &crate::parser::slt::NavigableSlt<'_>,
        cmd: &mut crate::command::Cmd,
    ) -> codegen::error::Result<()> {
        result! {
            write!(self.writer, ".global _start\n.align 2\n_start:\n");
            write!(self.writer, "    // Program header\n");
            write!(self.writer, "    stp x29, lr, [sp, -0x10]!");
            write!(self.writer, "    mov x29, sp");
        };

        Ok(())
    }

    fn run_program(&mut self, cmd: &mut crate::command::Cmd) -> codegen::error::Result<()> {
        todo!()
    }
}
