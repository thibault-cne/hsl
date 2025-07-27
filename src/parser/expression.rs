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
            T![OFnCall] => {
                self.consume(T![OFnCall]);
                let ident = self
                    .next()
                    .expect("Expected function identifier in fn call");
                assert_eq!(
                    ident.kind,
                    T![ID],
                    "Expected identifier after `fn call`, but found `{}`",
                    ident.kind
                );

                let mut args = Vec::new();
                while !self.check_next(T![CFnCall]) {
                    args.push(self.expression());
                }

                self.consume(T![CFnCall]);
                Expr::FnCall {
                    id: self.text(ident).to_string(),
                    args,
                }
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}
