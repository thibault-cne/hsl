use crate::ir::Lit;
use crate::lexer::token::Token;

use super::Parser;

impl<'input, 'prog, I> Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    pub fn literal(&mut self) -> Option<Lit<'prog>> {
        let Some(kind) = self.peek() else {
            error!("expected a literal but got nothing");
            self.err_cpt += 1;
            return None;
        };

        match kind {
            T![String] => {
                // SAFETY: this is safe because we peeked the token before
                let tok = self.next().unwrap();
                let str = self.text(tok);
                let str = &str[1..(str.len() - 1)];
                Some(Lit::Str(self.arena.strdup(str)))
            }
            T![Not] => {
                self.consume(T![Not])?;

                if !self.check_next(T![IntLit]) {
                    error!("expected `IntLit` after `Not`");
                    self.err_cpt += 1;
                    return None;
                }

                // SAFETY: this is safe because we checked the token before
                let tok = self.next().unwrap();
                let str = self.text(tok);
                // SAFETY: this is safe of the lexer
                Some(Lit::Int(-str.parse::<i64>().unwrap()))
            }
            T![IntLit] => {
                // SAFETY: this is safe because we peeked the token before
                let tok = self.next().unwrap();
                let str = self.text(tok);
                // SAFETY: this is safe of the lexer
                Some(Lit::Int(str.parse().unwrap()))
            }
            T![True] => Some(Lit::Bool(true)),
            T![False] => Some(Lit::Bool(false)),
            kind => {
                error!("unknown start of expression: `{kind}`");
                self.err_cpt += 1;
                None
            }
        }
    }
}
