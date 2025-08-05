use crate::ir::{Expr, Lit, Stmt, Unop};
use crate::lexer::token::Token;
use crate::parser::Parser;

use super::slt::{Builder, SymbolLookupTable};

impl<'input, 'prog, I> Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    pub fn statement(
        &mut self,
        _slt_builder: &mut Builder,
        slt: &mut SymbolLookupTable<'prog>,
    ) -> Stmt<'prog> {
        let Some(kind) = self.peek() else {
            panic!("Expected a statement and found nothing");
        };

        match kind {
            T![Let] => {
                self.consume(T![Let]);
                let ident = self.next().expect("Expected an identifier after `let`");
                assert_eq!(
                    ident.kind,
                    T![ID],
                    "Expected identifier after `let`, but found `{}`",
                    ident.kind
                );
                let id = self.arena.strdup(self.text(ident));
                self.consume(T![Assign]);
                let value = self.expression();

                let res = match value {
                    Expr::Lit(Lit::Str(s)) => slt.add_variable(
                        (
                            id,
                            crate::parser::slt::Type::Val(crate::parser::slt::InnerType::Str),
                            s,
                        ),
                        ident.span,
                    ),
                    Expr::Lit(Lit::Int(i)) => slt.add_variable(
                        (
                            id,
                            crate::parser::slt::Type::Val(crate::parser::slt::InnerType::Int),
                            i,
                        ),
                        ident.span,
                    ),
                    Expr::Lit(Lit::Bool(b)) => slt.add_variable(
                        (
                            id,
                            crate::parser::slt::Type::Val(crate::parser::slt::InnerType::Bool),
                            b,
                        ),
                        ident.span,
                    ),
                    _ => unreachable!(),
                };

                if let Some((_, loc)) = res {
                    self.err_cpt += 1;
                    error!(
                        "variable {id} already declared, previous declaration happened on line {}",
                        loc.line
                    );
                }

                Stmt::Let { id, value }
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
                let id = self.arena.strdup(self.text(ident));

                let mut args = Vec::new();
                while !self.check_next(T![CFnCall]) {
                    args.push(self.expression());
                }

                self.consume(T![CFnCall]);
                Stmt::FnCall { id, args }
            }
            T![OAssign] => {
                self.consume(T![OAssign]);
                let ident = self.next().expect("Expected an identifier after `assign`");
                assert_eq!(
                    ident.kind,
                    T![ID],
                    "Expected identifier after `assign`, but found `{}`",
                    ident.kind
                );
                let id = self.arena.strdup(self.text(ident));

                let mut ops = Vec::new();
                while !self.check_next(T![CAssign]) {
                    ops.push(self.unary_op());
                }

                self.consume(T![CAssign]);
                Stmt::Assign { id, ops }
            }
            //T![if] => {
            //    self.consume(T![if]);

            //    let condition = Box::new(self.expression());
            //    let mut body = Vec::new();

            //    while !self.at(T![else]) && !self.at(T![if_end]) {
            //        body.push(self.statement());
            //    }

            //    let else_stmt = if self.at(T![else]) {
            //        self.consume(T![else]);
            //        let mut else_body = Vec::new();

            //        while !self.at(T![if_end]) {
            //            else_body.push(self.statement());
            //        }

            //        Some(else_body)
            //    } else {
            //        None
            //    };

            //    self.consume(T![if_end]);
            //    ast::Stmt::IfStmt {
            //        condition,
            //        body,
            //        else_stmt,
            //    }
            //}
            kind => panic!("Unknown start of expression: `{kind}`"),
        }
    }

    fn unary_op(&mut self) -> Unop<'prog> {
        let Some(kind) = self.peek() else {
            panic!("Expected an unary operator and found nothing");
        };

        match kind {
            T![Plus] => Unop {
                op: crate::ir::Op::Add,
                value: self.expression(),
            },
            T![Minus] => Unop {
                op: crate::ir::Op::Sub,
                value: self.expression(),
            },
            T![Div] => Unop {
                op: crate::ir::Op::Div,
                value: self.expression(),
            },
            T![Mul] => Unop {
                op: crate::ir::Op::Mul,
                value: self.expression(),
            },
            T![Mod] => Unop {
                op: crate::ir::Op::Mod,
                value: self.expression(),
            },
            kind => panic!("Unknown start of unary operator: `{kind}`"),
        }
    }
}
