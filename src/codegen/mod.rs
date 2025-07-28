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
        slt: &'prog NavigableSlt<'prog>,
        childs: &mut ChildIterator<'prog>,
        cmd: &mut Cmd<'prog>,
    ) -> error::Result<()>;
    fn run_program(&mut self, cmd: &mut Cmd<'prog>) -> error::Result<()>;
}

pub fn build_codegen<'prog>(
    target: crate::target::Target,
    output_path: &'prog str,
    o_path: &'prog str,
    b_path: &'prog str,
    quiet: bool,
    run: bool,
) -> impl Codegen<'prog> {
    use crate::target::Target::*;

    match target {
        AArch64Darwin => {
            let output_file_writer =
                std::fs::File::create(output_path).expect("unable to create output file");
            aarch64::Codegen::new(output_path, o_path, b_path, quiet, run, output_file_writer)
        }
    }
}
