use crate::lexer::token::Token;
use crate::parser::ast;
use crate::parser::Parser;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn expression(&mut self) -> ast::Expr {
        match self.peek() {
            T![string] | T![int] | T![bool] => ast::Expr::Literal(self.literal()),
            T![ident] => {
                let name = {
                    let ident_token = self.next().unwrap();
                    self.text(ident_token)
                };

                ast::Expr::Ident(name.to_string())
            }
            lit @ T![add] | lit @ T![sub] | lit @ T![mul] | lit @ T![div] | lit @ T![mod] => {
                self.consume(lit);
                let value = match self.literal() {
                    lit @ ast::Lit::Int(_) => lit,
                    lit => panic!(
                        "Unexpected value for operation, expected integer got: {:?}",
                        lit
                    ),
                };
                ast::Expr::Op {
                    op: ast::Op::try_from(lit).unwrap(),
                    value,
                }
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}
