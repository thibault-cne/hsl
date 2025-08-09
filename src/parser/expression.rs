use crate::ir::{Arg, Expr, InnerType, Lit, Type};
use crate::lexer::token::Token;
use crate::parser::Parser;

use super::slt::{Builder, SymbolLookupTable};

impl<'input, 'prog, I> Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    pub fn expression(
        &mut self,
        _slt_builder: &mut Builder,
        slt: &mut SymbolLookupTable<'prog>,
    ) -> Option<Expr<'prog>> {
        let Some(kind) = self.peek() else {
            error!("Expected a statement and found nothing");
            self.err_cpt += 1;
            return None;
        };

        match kind {
            T![Let] => {
                self.consume(T![Let])?;
                let Some(ident) = self.next() else {
                    error!("expected identifier after `let` but found nothing");
                    self.err_cpt += 1;
                    return None;
                };

                if ident.kind != T![ID] {
                    error!(
                        "expected identifier after `let` but found {} instead",
                        ident.kind
                    );
                    self.err_cpt += 1;
                    return None;
                }

                let id = self.arena.strdup(self.text(ident));
                self.consume(T![Assign])?;
                let value = self.arg()?;

                let res = match value {
                    Arg::Lit(Lit::Str(s)) => {
                        slt.add_variable((id, Type::Val(InnerType::Str), s), ident.span)
                    }
                    Arg::Lit(Lit::Int(i)) => {
                        slt.add_variable((id, Type::Val(InnerType::Int), i), ident.span)
                    }
                    Arg::Lit(Lit::Bool(b)) => {
                        slt.add_variable((id, Type::Val(InnerType::Bool), b), ident.span)
                    }
                    _ => {
                        error!("invalid expression found");
                        self.err_cpt += 1;
                        return None;
                    }
                };

                if let Some((_, loc)) = res {
                    self.err_cpt += 1;
                    error!(
                        "variable {id} already declared, previous declaration happened on line {}",
                        loc.line
                    );
                }

                Some(Expr::Let { id, value })
            }
            T![OFnCall] => {
                self.consume(T![OFnCall])?;
                let Some(ident) = self.next() else {
                    error!("expected identifier after `fn_call` but found nothing");
                    self.err_cpt += 1;
                    return None;
                };

                if ident.kind != T![ID] {
                    error!(
                        "expected identifier after `fn_call` but found {} instead",
                        ident.kind
                    );
                    self.err_cpt += 1;
                    return None;
                }
                let id = self.arena.strdup(self.text(ident));

                let mut args = Vec::new();
                while !self.check_next(T![CFnCall]) {
                    args.push(self.arg()?);
                }

                self.consume(T![CFnCall])?;
                Some(Expr::FnCall { id, args })
            }
            //T![OAssign] => {
            //    self.consume(T![OAssign])?;
            //    let Some(ident) = self.next() else {
            //        error!("expected identifier after `assign` but found nothing");
            //        self.err_cpt += 1;
            //        return None;
            //    };

            //    if ident.kind != T![ID] {
            //        error!(
            //            "expected identifier after `assign` but found {} instead",
            //            ident.kind
            //        );
            //        self.err_cpt += 1;
            //        return None;
            //    }

            //    let id = self.arena.strdup(self.text(ident));

            //    let mut ops = Vec::new();
            //    while !self.check_next(T![CAssign]) {
            //        ops.push(self.unary_op()?);
            //    }

            //    self.consume(T![CAssign])?;
            //    Some(Stmt::Assign { id, ops })
            //}
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
            kind => {
                error!("unknown start of statement: `{kind}`");
                self.err_cpt += 1;
                None
            }
        }
    }

    //fn unary_op(&mut self) -> Option<Unop<'prog>> {
    //    let Some(kind) = self.peek() else {
    //        error!("expected an unary operator and found nothing");
    //        self.err_cpt += 1;
    //        return None;
    //    };

    //    let unop = match kind {
    //        T![Plus] => Unop {
    //            op: crate::ir::Op::Add,
    //            value: self.expression()?,
    //        },
    //        T![Minus] => Unop {
    //            op: crate::ir::Op::Sub,
    //            value: self.expression()?,
    //        },
    //        T![Div] => Unop {
    //            op: crate::ir::Op::Div,
    //            value: self.expression()?,
    //        },
    //        T![Mul] => Unop {
    //            op: crate::ir::Op::Mul,
    //            value: self.expression()?,
    //        },
    //        T![Mod] => Unop {
    //            op: crate::ir::Op::Mod,
    //            value: self.expression()?,
    //        },
    //        kind => {
    //            error!("unknown start of unary operator: `{kind}`");
    //            self.err_cpt += 1;
    //            return None;
    //        }
    //    };

    //    Some(unop)
    //}
}
