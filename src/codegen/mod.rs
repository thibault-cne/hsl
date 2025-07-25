use crate::command::Cmd;
use crate::ir;
use crate::parser::slt::NavigableSlt;

#[macro_use]
pub mod error;

pub mod aarch64;

pub trait Compiler<'prog> {
    fn generate_program(
        &mut self,
        program: &'prog ir::Program,
        slt: &'prog NavigableSlt<'_>,
        cmd: &mut Cmd,
    ) -> error::Result<()>;
    fn run_program(&mut self, cmd: &mut Cmd) -> error::Result<()>;
}

pub fn build_compiler<'prog>(
    target: crate::target::Target,
    output_file: &'static str,
) -> impl Compiler<'prog> {
    use crate::target::Target::*;

    match target {
        AArch64Darwin => {
            let file = std::fs::File::create(output_file).expect("unable to create output file");
            aarch64::Compiler::new(file)
        }
    }
}
