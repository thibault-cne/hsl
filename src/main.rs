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
        error!("unable to create a compiler instance, it may be because you have wrong file paths");
        return std::process::ExitCode::FAILURE;
    };

    info!("compiling files {}", c.flags.source_files.join(", "));
    let mut slt_builder = parser::slt::Builder::new();
    let mut slt = slt_builder.region();
    let mut err_cpt = 0;

    // We deliberatly skip this error and just log it to continue compilation and collect has much
    // errors has possible
    let _ = c.try_for_each_source_files(|c, file| {
        let Ok(content) = std::fs::read_to_string(file) else {
            error!("unable to read file `{file}` continuing compilation to collect more errors");
            return Err(());
        };
        let mut parser = parser::Parser::new(&content, &arena);
        parser.parse(c.program_mut(), &mut slt_builder, &mut slt);
        err_cpt += parser.err_cpt;

        Ok(())
    });

    if err_cpt != 0 {
        error!("unable to compile your program because of {err_cpt} errors");
        return std::process::ExitCode::FAILURE;
    }

    let nav_slt: parser::slt::NavigableSlt<'_> = (&slt).into();

    let mut cmd = command::Cmd::new(c.flags.quiet);

    let program_slt = nav_slt.childs().next().unwrap();
    let mut program_slt_childs = program_slt.childs();

    let mut codegen = codegen::build_codegen(&c);

    if codegen
        .generate_program(&c.program, &program_slt, &mut program_slt_childs, &mut cmd)
        .is_err()
    {
        error!("an error occured in codegen, please check the logs or file an issue");
        return std::process::ExitCode::FAILURE;
    }

    if c.flags.run && codegen.run_program(&mut cmd).is_err() {
        error!(
            "an error occured while running the executable, please check the logs or file an issue"
        );
        return std::process::ExitCode::FAILURE;
    }

    std::process::ExitCode::SUCCESS
}
