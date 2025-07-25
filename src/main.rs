#[macro_use]
mod lexer;

#[macro_use]
mod macros;

//mod asm;
mod codegen;
mod command;
mod flags;
mod ir;
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
    -v, --version       show the version of search

COMPILATION OPTIONS
    -s, --source        the source file to compile
    -o, --output        the output file to produce
    -t, --target        the targeted architecture (must be in [armv8, armv7, x86])
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
        println!("{}", USAGE);
        return std::process::ExitCode::SUCCESS;
    }

    if flags.source_files.is_empty() {
        todo!()
    }

    let Some(target) = flags.target_name.and_then(target::Target::by_name) else {
        println!("{}", USAGE);
        return std::process::ExitCode::FAILURE;
    };

    let Some(ouput_file) = flags.output_path else {
        println!("{}", USAGE);
        return std::process::ExitCode::FAILURE;
    };

    // We are sure that `flags.source_files` is not empty
    let content = std::fs::read_to_string(flags.source_files[0]).expect("unable to read file");

    let mut parser = parser::Parser::new(&content);
    let program = parser.parse();

    let mut builder = parser::slt::Builder::new();
    let mut slt = builder.region();

    program.visit(&mut builder, &mut slt);

    let mut cmd = command::Cmd::new();
    let mut compiler = codegen::build_compiler(target, ouput_file);

    // Generate the program
    // TODO: handle error
    let _ = compiler.generate_program(&program, &(&slt).into(), &mut cmd);

    // If run option unabled than run the program
    // TODO: handle error
    let _ = compiler.run_program(&mut cmd);

    std::process::ExitCode::SUCCESS
}
