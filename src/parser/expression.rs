use crate::ir::Expr;
use crate::lexer::token::Token;
use crate::parser::Parser;

impl<'input, 'prog, I> Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    pub fn expression(&mut self) -> Option<Expr<'prog>> {
        let Some(kind) = self.peek() else {
            error!("Expected an expression and found nothing");
            self.err_cpt += 1;
            return None;
        };

        match kind {
            T![String] | T![Not] | T![IntLit] | T![True] | T![False] => {
                Some(Expr::Lit(self.literal()?))
            }
            T![ID] => {
                // Consumes the token and retrieve the id in the parser state
                self.consume(T![ID])?;
                Some(Expr::ID(self.arena.strdup(self.id)))
            }
            T![OFnCall] => {
                self.consume(T![OFnCall])?;
                self.consume(T![ID])?;
                let id = self.arena.strdup(self.id);

                let mut args = Vec::new();
                while !self.check_next(T![CFnCall]) {
                    args.push(self.expression()?);
                }

                self.consume(T![CFnCall])?;
                Some(Expr::FnCall { id, args })
            }
            kind => {
                error!("unknown start of expression: `{kind}`");
                self.err_cpt += 1;
                None
            }
        }
    }
}
