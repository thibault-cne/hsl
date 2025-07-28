#![allow(static_mut_refs)]

#[macro_use]
mod macros;

#[macro_use]
mod log;

#[macro_use]
mod lexer;

#[macro_use]
mod command;

mod arena;
mod codegen;
mod compiler;
mod flags;
mod fs;
mod ir;
mod math;
mod parser;
mod target;

use codegen::Codegen;
use parser::slt::Visitor;

/// The help string.
/// This string is printed when the user asks for help.
static USAGE: &str = "Usage: 
    hsl [options]

META OPTIONS
    -h, --help          show this!

COMPILATION OPTIONS
    -s, --source        the source file to compile
    -o, --output        the output file to produce
    -t, --target        the targeted architecture (must be in [armv8])
";

fn main() -> std::process::ExitCode {
    // Create the arena allocator to store all compiler variables
    let arena = arena::Arena::new();

    let default_target = if cfg!(target_arch = "aarch64") && cfg!(target_os = "macos") {
        Some(target::Target::AArch64Darwin)
    } else {
        None
    };

    let Some(mut c) = compiler::Compiler::new(&arena, default_target.map(|d| d.name())) else {
        return std::process::ExitCode::FAILURE;
    };

    // We are sure that `flags.source_files` is not empty
    // TODO: handle multiple files
    info!("compiling files {}", c.flags.source_files.join(", "));

    {
        let content =
            std::fs::read_to_string(c.flags.source_files[0]).expect("unable to read file");
        let mut parser = parser::Parser::new(&content, &arena);
        parser.parse(c.program_mut());
    }

    let mut builder = parser::slt::Builder::new();
    let mut slt = builder.region();

    c.program.visit(&mut builder, &mut slt);
    let nav_slt: parser::slt::NavigableSlt<'_> = (&slt).into();

    let mut cmd = command::Cmd::new(c.flags.quiet);

    let program_slt = nav_slt.childs().next().unwrap();
    let mut program_slt_childs = program_slt.childs();

    let mut codegen = codegen::build_codegen(&c);

    // Generate the program
    // TODO: handle error
    if codegen
        .generate_program(&c.program, &program_slt, &mut program_slt_childs, &mut cmd)
        .is_err()
    {
        return std::process::ExitCode::FAILURE;
    }

    // If run option unabled than run the program
    // TODO: handle error
    if c.flags.run && codegen.run_program(&mut cmd).is_err() {
        return std::process::ExitCode::FAILURE;
    }

    std::process::ExitCode::SUCCESS
}
