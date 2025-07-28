use crate::ir::Lit;
use crate::lexer::token::Token;

use super::Parser;

impl<'input, 'prog, I> Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    pub fn literal(&mut self) -> Lit<'prog> {
        let Some(kind) = self.peek() else {
            panic!("expected a literal but got nothing");
        };

        match kind {
            T![String] => {
                let tok = self.next().unwrap();
                let str = self.text(tok);
                let str = &str[1..(str.len() - 1)];
                Lit::Str(self.arena.strdup(str))
            }
            T![Not] => {
                self.consume(T![Not]);
                self.check_next(T![IntLit]);
                let tok = self.next().unwrap();
                let str = self.text(tok);
                Lit::Int(
                    -1 * str
                        .parse::<i64>()
                        .unwrap_or_else(|_| panic!("invalid negative integer literal: `{}`", str)),
                )
            }
            T![IntLit] => {
                let tok = self.next().unwrap();
                let str = self.text(tok);
                Lit::Int(
                    str.parse()
                        .unwrap_or_else(|_| panic!("invalid integer literal: `{}`", str)),
                )
            }
            T![True] => Lit::Bool(true),
            T![False] => Lit::Bool(false),
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}
