use crate::lexer::token::Token;
use crate::parser::ast;
use crate::parser::Parser;

use super::slt::{Builder, SymbolLookupTable};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    // pub fn item(&mut self) -> ast::Item {
    //     // TODO:
    //     todo!()
    // }
}

impl ast::Item {
    pub fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable) {
        match self {
            ast::Item::Main { body } => {
                builder.new_region(slt);
                for stm in body {
                    stm.visit(builder, slt.last_children_mut().unwrap());
                }
            }
        }
    }
}
