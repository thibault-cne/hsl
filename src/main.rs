#![allow(static_mut_refs)]

#[macro_use]
mod lexer;

#[macro_use]
mod macros;

#[macro_use]
mod log;

#[macro_use]
mod command;

mod codegen;
mod flags;
mod fs;
mod ir;
mod math;
mod parser;
mod target;

use codegen::Compiler;
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
    let default_target = if cfg!(target_arch = "aarch64") && cfg!(target_os = "macos") {
        Some(target::Target::AArch64Darwin)
    } else {
        None
    };

    // Parse flags passed to the compiler
    let flags = flags::Flags::parse(default_target.map(|d| d.name()));

    if flags.help {
        eprint!("{}", USAGE);
        return std::process::ExitCode::SUCCESS;
    }

    if flags.source_files.is_empty() {
        todo!()
    }

    let Some(target) = flags.target_name.and_then(target::Target::by_name) else {
        eprint!("{}", USAGE);
        return std::process::ExitCode::FAILURE;
    };

    let program_path = if let Some(program_path) = flags.output_path {
        program_path
    } else {
        // SAFETY: this is safe because `flags.source_files` is not empty
        fs::strip_extension(flags.source_files[0])
    };

    // We are sure that `flags.source_files` is not empty
    // TODO: handle multiple files
    info!("compiling files {}", flags.source_files.join(", "));
    let content = std::fs::read_to_string(flags.source_files[0]).expect("unable to read file");

    let mut parser = parser::Parser::new(&content);
    let program = parser.parse();

    let mut builder = parser::slt::Builder::new();
    let mut slt = builder.region();

    program.visit(&mut builder, &mut slt);
    let nav_slt: parser::slt::NavigableSlt<'_> = (&slt).into();

    let mut cmd = command::Cmd::new(flags.quiet);
    let files = fs::Files::new(program_path);

    let Some(output_path) = files.output_path.to_str() else {
        error!("couldn't format output path");
        return std::process::ExitCode::FAILURE;
    };

    let Some(object_path) = files.object_path.to_str() else {
        error!("couldn't format object path");
        return std::process::ExitCode::FAILURE;
    };

    let mut compiler = codegen::build_compiler(
        target,
        &output_path,
        &object_path,
        files.build_path,
        flags.quiet,
        flags.run,
    );

    // Generate the program
    // TODO: handle error
    if compiler
        .generate_program(&program, &nav_slt, &mut cmd)
        .is_err()
    {
        return std::process::ExitCode::FAILURE;
    }

    // If run option unabled than run the program
    // TODO: handle error
    if flags.run && compiler.run_program(&mut cmd).is_err() {
        return std::process::ExitCode::FAILURE;
    }

    std::process::ExitCode::SUCCESS
}
