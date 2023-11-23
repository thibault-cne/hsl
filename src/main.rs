#[macro_use]
extern crate lazy_static;

#[macro_use]
mod lexer;

mod asm;
mod parser;

use asm::Compiler;

fn main() {
    let content = std::fs::read_to_string("examples/if_then_else.hs").expect("not found");

    let mut parser = parser::Parser::new(&content);
    let program = parser.parse();

    println!("{:?}", program);

    let mut builder = parser::slt::Builder::new();
    let mut slt = builder.region();

    program.visit(&mut builder, &mut slt);

    println!("{:?}", slt);

    let file = std::fs::File::create("assembly/srcs/test.s").expect("unable to create a new file");

    asm::evaluate(vec![program], &(&slt).into(), asm::A64Compiler::new(file))
        .expect("unable to compile");
}
