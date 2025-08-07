use crate::command::Cmd;
use crate::ir;
use crate::parser::slt::{ChildIterator, NavigableSlt};

#[macro_use]
pub mod error;

pub mod aarch64;

pub trait Codegen<'prog> {
    fn generate_program(
        &mut self,
        program: &'prog ir::Program,
        slt: &'prog NavigableSlt<'prog, 'prog>,
        childs: &mut ChildIterator<'prog, 'prog>,
        cmd: &mut Cmd<'prog>,
    ) -> error::Result<()>;
    fn run_program(&mut self, cmd: &mut Cmd<'prog>) -> error::Result<()>;
}

pub fn build_codegen<'prog>(c: &'prog crate::compiler::Compiler) -> impl Codegen<'prog> {
    use crate::target::Target::*;

    match c.target {
        AArch64Darwin => {
            let output_file_writer =
                std::fs::File::create(c.output_path).expect("unable to create output file");
            aarch64::Codegen::new(c, output_file_writer)
        }
    }
}
