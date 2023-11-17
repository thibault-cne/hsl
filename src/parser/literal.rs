use crate::token::Token;

use super::{ast, Parser};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn literal(&mut self) -> ast::Node {
        match self.peek() {
            lit @ T![string] | lit @ T![int] | lit @ T![float] => {
                let literal_text = {
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };

                let lit = match lit {
                    T![int] => ast::Node::Integer(
                        literal_text
                            .parse()
                            .expect(&format!("Invalid integer literal: `{}`", literal_text)),
                    ),
                    T![string] => {
                        ast::Node::String(literal_text[1..(literal_text.len() - 1)].to_string())
                    }
                    T![float] => ast::Node::Float(literal_text.parse().expect(&format!(
                        "Invalid floating point literal: `{}`",
                        literal_text
                    ))),
                    _ => unreachable!(),
                };

                lit
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}