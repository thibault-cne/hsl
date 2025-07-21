use crate::ir::Expr;
use crate::lexer::token::Token;
use crate::parser::Parser;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn expression(&mut self) -> Expr {
        let Some(kind) = self.peek() else {
            panic!("Expected an expression and found nothing");
        };

        match kind {
            T![String] | T![IntLit] | T![True] | T![False] => Expr::Lit(self.literal()),
            T![ID] => {
                let name = {
                    let ident_token = self.next().unwrap();
                    self.text(ident_token)
                };

                Expr::ID(name.to_string())
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}
