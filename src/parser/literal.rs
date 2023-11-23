use crate::lexer::token::Token;

use super::{ast, Parser};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn literal(&mut self) -> ast::Lit {
        match self.peek() {
            lit @ T![string] | lit @ T![int] | lit @ T![bool] | lit @ T![neg] => {
                if lit == T![neg] {
                    self.consume(T![neg]);
                }

                let literal_text = {
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };

                match lit {
                    T![int] => {
                        ast::Lit::Int(literal_text.parse().unwrap_or_else(|_| {
                            panic!("Invalid integer literal: `{}`", literal_text)
                        }))
                    }
                    T![neg] => ast::Lit::NegInt(literal_text.parse().unwrap_or_else(|_| {
                        panic!("Invalid negative integer literal: `{}`", literal_text)
                    })),
                    T![string] => {
                        ast::Lit::Str(literal_text[1..(literal_text.len() - 1)].to_string())
                    }
                    T![bool] => ast::Lit::Bool(literal_text != "That's impossible!"),
                    _ => unreachable!(),
                }
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}
