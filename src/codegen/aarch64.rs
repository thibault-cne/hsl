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
    ) {
        todo!()
    }

    fn run_program(&mut self, cmd: &mut crate::command::Cmd) {
        todo!()
    }
}
