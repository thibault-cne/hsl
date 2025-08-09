use slt::{Builder, SymbolLookupTable};

use crate::ir::{Extrn, Fn, InnerType, Program, Type};
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;

mod arg;
mod expression;
mod literal;
pub mod slt;

pub struct Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    arena: &'prog crate::arena::Arena<'prog>,
    input: &'input str,
    tokens: std::iter::Peekable<I>,

    id: &'input str,
    integer: usize,
    span: crate::lexer::token::Span,
    pub has_main: bool,
    pub err_cpt: usize,
}

impl<'input, 'prog> Parser<'input, 'prog, Lexer<'input>> {
    pub fn new(input: &'input str, arena: &'prog crate::arena::Arena<'prog>) -> Self {
        Parser {
            arena,
            input,
            tokens: Lexer::new(input).peekable(),
            id: "",
            integer: 0,
            span: crate::lexer::token::Span::default(),
            has_main: false,
            err_cpt: 0,
        }
    }
}

impl<'input, 'prog, I> Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    /// Get the source text of a token::Token
    pub fn text(&self, token: Token) -> &'input str {
        token.text(self.input)
    }

    pub(crate) fn peek(&mut self) -> Option<TokenKind> {
        self.tokens.peek().map(|t| t.kind)
    }

    /// Check if the next token is of a given kind
    pub(crate) fn check_next(&mut self, kind: TokenKind) -> bool {
        let Some(t_kind) = self.peek() else {
            return false;
        };

        t_kind == kind
    }

    pub(crate) fn next(&mut self) -> Option<Token> {
        if let Some(tok) = self.tokens.next() {
            self.span = tok.span;
            Some(tok)
        } else {
            None
        }
    }

    /// Move forward one token in the input and check if the kind of the
    /// token is the same as `expected`.
    ///
    /// # Panics
    /// This panics if the consumed token don't have the same kind as `expected`
    /// or if there is no more tokens to consume.
    pub(crate) fn consume(&mut self, expected: TokenKind) -> Option<()> {
        let Some(token) = self.next() else {
            error!("expected to consume `{expected}`, but there was no next token.");
            self.err_cpt += 1;
            return None;
        };

        if token.kind != expected {
            error!(
                "expected to consume `{expected}`, but found `{}` instead",
                token.kind
            );
            self.err_cpt += 1;
            return None;
        }

        match token.kind {
            T![ID] => self.id = self.text(token),
            // SAFETY: this is guarented to work thanks to the lexer
            T![IntLit] => self.integer = self.text(token).parse().unwrap(),
            _ => (),
        }

        Some(())
    }

    pub(crate) fn parse(
        &mut self,
        program: &mut Program<'prog>,
        slt_builder: &mut Builder,
        slt: &mut SymbolLookupTable<'prog>,
    ) -> Option<()> {
        while !self.check_next(T![EOF]) {
            // SAFETY: this is safe since the while loop is still looping
            match self.peek().unwrap() {
                T![OFnDecl1] => program.func.push(self.parse_function(slt_builder, slt)?),
                T![OExtrnFn] => program.extrn.push(self.parse_extrn_function(slt)?),
                _ => todo!("handle unexpected token"),
            }
        }
        self.consume(T![EOF])?;
        Some(())
    }

    fn parse_extrn_function(&mut self, slt: &mut SymbolLookupTable<'prog>) -> Option<Extrn<'prog>> {
        self.consume(T![OExtrnFn])?;

        self.consume(T![ID])?;
        let id = self.arena.strdup(self.id);

        let variadic = if self.check_next(T![Variadic]) {
            self.consume(T![Variadic])?;
            self.consume(T![IntLit])?;

            Some(self.integer)
        } else {
            None
        };

        let mut args = Vec::new();
        while !self.check_next(T![CExtrnFn]) {
            let Some(kind) = self.peek() else {
                error!("expected type token in function params");
                return None;
            };

            let ty = match kind {
                T![TyInt] => Type::Val(InnerType::Int),
                T![TyString] => Type::Val(InnerType::Str),
                T![TyBool] => Type::Val(InnerType::Bool),
                _ => {
                    error!("unexpected token for type");
                    self.err_cpt += 1;
                    return None;
                }
            };
            self.consume(kind)?;
            args.push(ty);
        }

        self.consume(T![CExtrnFn])?;

        if let Some(variadic) = variadic {
            if args.len() != variadic {
                error!(
                    "invalid amount of fixed external function arguments given for {id} please verify it"
                );
                self.err_cpt += 1;
            }
        }

        let extrn = Extrn { id, variadic, args };
        slt.add_function(&extrn, self.span);

        Some(extrn)
    }

    fn parse_function(
        &mut self,
        slt_builder: &mut Builder,
        slt: &mut SymbolLookupTable<'prog>,
    ) -> Option<Fn<'prog>> {
        slt_builder.new_region(slt);
        let child_mut = slt.last_children_mut().unwrap();

        self.consume(T![OFnDecl1])?;
        self.consume(T![ID])?;

        if self.id == "galaxy" {
            self.has_main = true;
        }

        let id = self.arena.strdup(self.id);

        self.consume(T![OFnDecl2])?;

        // Collect function parameters
        let mut args = Vec::new();
        if self.check_next(T![OFnParams]) {
            self.consume(T![OFnParams])?;

            while !self.check_next(T![CFnParams]) {
                let Some(kind) = self.peek() else {
                    error!("expected type token in function params");
                    self.err_cpt += 1;
                    return None;
                };

                let ty = match kind {
                    T![TyInt] => Type::Val(InnerType::Int),
                    T![TyString] => Type::Val(InnerType::Str),
                    T![TyBool] => Type::Val(InnerType::Bool),
                    _ => {
                        error!("unexpected token for type");
                        self.err_cpt += 1;
                        return None;
                    }
                };
                self.consume(kind)?;

                self.consume(T![ID])?;

                let id = self.arena.strdup(self.id);

                args.push((id, ty));
                child_mut.add_variable((id, ty), self.span);
            }

            self.consume(T![CFnParams])?;
        }

        // TODO: handle variadic functions
        let variadic = if self.check_next(T![Variadic]) {
            self.consume(T![Variadic])?;
            if !self.check_next(T![IntLit]) {
                todo!("handle non integer variadic");
            }
            let tok = self.next()?;
            let _variadic: usize = self.text(tok).parse().ok()?;

            todo!("handle the variadic functions")
        } else {
            None
        };

        let mut body = Vec::new();
        while !self.check_next(T![CFnDecl]) {
            body.push(self.expression(slt_builder, child_mut)?);
        }

        self.consume(T![CFnDecl])?;

        let func = Fn {
            id,
            body,
            variadic,
            args,
        };

        slt.add_function(&func, self.span);

        Some(func)
    }
}
