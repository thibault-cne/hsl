#[macro_use]
extern crate lazy_static;

#[macro_use]
mod lexer;

mod asm;
mod option;
mod parser;

use core::panic;
use std::env;

use asm::Compiler;

fn main() {
    use std::process::exit;

    let args: Vec<_> = env::args_os().skip(1).collect();
    let options = match option::Option::parse(args.iter().map(std::convert::AsRef::as_ref)) {
        option::OptionsResult::Ok(o, _) => o,
        option::OptionsResult::InvalidOptions(e) => panic!("Error while parsing args: {:?}", e),
        option::OptionsResult::Help(help) => {
            println!("{}", help);
            exit(exit::SUCCESS);
        }
    };

    let content = std::fs::read_to_string(&options.source).expect("not found");

    let mut parser = parser::Parser::new(&content);
    let program = parser.parse();

    println!("{:?}", program);

    let mut builder = parser::slt::Builder::new();
    let mut slt = builder.region();

    program.visit(&mut builder, &mut slt);

    println!("{:?}", slt);

    let file = std::fs::File::create(&options.output).expect("unable to create a new file");

    asm::evaluate(vec![program], &(&slt).into(), asm::A64Compiler::new(file))
        .expect("unable to compile");
}

mod exit {
    pub const SUCCESS: i32 = 0;
}
