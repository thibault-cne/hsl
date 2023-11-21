use crate::lexer::token::Token;
use crate::parser::ast;
use crate::parser::Parser;

use super::slt::{Builder, SymbolLookupTable};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn statement(&mut self) -> ast::Stmt {
        match self.peek() {
            T![let] => {
                self.consume(T![let]);
                let ident = self.next().expect("Expected an identifier after `let`");
                assert_eq!(
                    ident.kind,
                    T![ident],
                    "Expected identifier after `let`, but found `{}`",
                    ident.kind
                );
                let var_name = self.text(ident).to_string();
                self.consume(T![init]);
                let value = self.expression();
                ast::Stmt::Let {
                    var_name,
                    value: Box::new(value),
                }
            }
            T![print] => {
                self.consume(T![print]);
                let arg = self.expression();

                ast::Stmt::Print {
                    value: Box::new(arg),
                }
            }
            T![assign_start] => {
                self.consume(T![assign_start]);
                // TODO: check that the var_name is an ident
                let var_name = Box::new(self.expression());
                self.consume(T![set]);
                let initial_value = Box::new(self.expression());

                let mut operations = Vec::new();
                while !self.at(T![assign_end]) {
                    operations.push(self.expression());
                }

                self.consume(T![assign_end]);
                ast::Stmt::Assignment {
                    var_name,
                    initial_value,
                    operations,
                }
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}

impl ast::Stmt {
    pub fn visit(&self, _builder: &mut Builder, slt: &mut SymbolLookupTable) {
        match self {
            ast::Stmt::Let { var_name, value } => {
                if let ast::Expr::Literal(lit) = &**value {
                    match lit {
                        ast::Lit::Str(s) => slt.add_string(var_name, s.to_string()),
                        ast::Lit::Int(i) => slt.add_integer(var_name, *i),
                        ast::Lit::Bool(b) => slt.add_boolean(var_name, *b),
                    }
                }
            }
            ast::Stmt::Print { .. } => {}
            _ => (),
        }
    }
}
