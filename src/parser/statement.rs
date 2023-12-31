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
                let value = Box::new(self.expression());
                ast::Stmt::Let { var_name, value }
            }
            T![print] => {
                self.consume(T![print]);
                let value = Box::new(self.expression());

                ast::Stmt::Print { value }
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
            T![if] => {
                self.consume(T![if]);

                let condition = Box::new(self.expression());
                let mut body = Vec::new();

                while !self.at(T![else]) && !self.at(T![if_end]) {
                    body.push(self.statement());
                }

                let else_stmt = if self.at(T![else]) {
                    self.consume(T![else]);
                    let mut else_body = Vec::new();

                    while !self.at(T![if_end]) {
                        else_body.push(self.statement());
                    }

                    Some(else_body)
                } else {
                    None
                };

                self.consume(T![if_end]);
                ast::Stmt::IfStmt {
                    condition,
                    body,
                    else_stmt,
                }
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}

impl ast::Stmt {
    pub fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable) {
        match self {
            ast::Stmt::Let { var_name, value } => {
                if let ast::Expr::Literal(lit) = &**value {
                    match lit {
                        ast::Lit::Str(s) => slt.add_string(var_name, s.to_string()),
                        ast::Lit::Int(i) => slt.add_integer(var_name, *i),
                        ast::Lit::NegInt(i) => slt.add_negative_integer(var_name, *i),
                        ast::Lit::Bool(b) => slt.add_boolean(var_name, *b),
                    }
                }
            }
            ast::Stmt::Print { .. } => {}
            ast::Stmt::IfStmt {
                body, else_stmt, ..
            } => {
                builder.new_region(slt);
                for stmt in body {
                    stmt.visit(builder, slt.last_children_mut().unwrap());
                }
                if let Some(else_stmt) = else_stmt {
                    builder.new_region(slt);
                    for stmt in else_stmt {
                        stmt.visit(builder, slt.last_children_mut().unwrap());
                    }
                }
            }
            _ => (),
        }
    }
}
