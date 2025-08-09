use crate::ir::Arg;
use crate::lexer::token::Token;
use crate::parser::Parser;

impl<'input, 'prog, I> Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    pub fn arg(&mut self) -> Option<Arg<'prog>> {
        let Some(kind) = self.peek() else {
            error!("Expected an expression and found nothing");
            self.err_cpt += 1;
            return None;
        };

        match kind {
            T![String] | T![Not] | T![IntLit] | T![True] | T![False] => {
                Some(Arg::Lit(self.literal()?))
            }
            T![ID] => {
                // Consumes the token and retrieve the id in the parser state
                self.consume(T![ID])?;
                Some(Arg::Id(self.arena.strdup(self.id)))
            }
            kind => {
                error!("unknown start of expression: `{kind}`");
                self.err_cpt += 1;
                None
            }
        }
    }
}
