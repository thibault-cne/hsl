use slt::{Builder, SymbolLookupTable};

use crate::ir::{Extrn, Fn, Program};
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;

pub mod expression;
pub mod literal;
pub mod slt;
pub mod statement;

pub struct Parser<'input, 'prog, I>
where
    I: Iterator<Item = Token>,
{
    arena: &'prog crate::arena::Arena<'prog>,
    input: &'input str,
    tokens: std::iter::Peekable<I>,

    id: &'input str,
    integer: usize,
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
        self.tokens.next()
    }

    /// Move forward one token in the input and check if the kind of the
    /// token is the same as `expected`.
    ///
    /// # Panics
    /// This panics if the consumed token don't have the same kind as `expected`
    /// or if there is no more tokens to consume.
    pub(crate) fn consume(&mut self, expected: TokenKind) {
        let token = self.next().unwrap_or_else(|| {
            panic!("Expected to consume `{expected}`, but there was no next token.")
        });
        assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}` instead",
            expected, token.kind
        );

        match token.kind {
            T![ID] => self.id = self.text(token),
            T![IntLit] => self.integer = self.text(token).parse().unwrap(),
            _ => (),
        }
    }

    pub(crate) fn parse(
        &mut self,
        program: &mut Program<'prog>,
        slt_builder: &mut Builder,
        slt: &mut SymbolLookupTable<'prog>,
    ) {
        slt_builder.new_region(slt);
        while !self.check_next(T![EOF]) {
            // SAFETY: this is safe since the while loop is still looping
            match self.peek().unwrap() {
                T![OFnDecl1] => program
                    .func
                    .push(self.parse_function(slt_builder, slt.last_children_mut().unwrap())),
                T![OExtrnFn] => self.parse_extrn_functions(program),
                _ => todo!("handle unexpected token"),
            }
        }
        self.consume(T![EOF]);
    }

    fn parse_extrn_functions(&mut self, program: &mut Program<'prog>) {
        self.consume(T![OExtrnFn]);

        while !self.check_next(T![CExtrnFn]) {
            self.consume(T![ID]);
            let id = self.arena.strdup(self.id);

            let variadic = if self.check_next(T![Variadic]) {
                self.consume(T![Variadic]);
                self.consume(T![IntLit]);

                Some(self.integer)
            } else {
                None
            };

            program.extrn.push(Extrn { id, variadic });
        }

        self.consume(T![CExtrnFn]);
    }

    fn parse_function(
        &mut self,
        slt_builder: &mut Builder,
        slt: &mut SymbolLookupTable<'prog>,
    ) -> Fn<'prog> {
        self.consume(T![OFnDecl1]);
        self.consume(T![ID]);

        if self.id == "galaxy" {
            self.has_main = true;
        }

        let id = self.arena.strdup(self.id);

        self.consume(T![OFnDecl2]);
        let variadic = if self.check_next(T![Variadic]) {
            self.consume(T![Variadic]);
            if !self.check_next(T![IntLit]) {
                todo!("handle non integer variadic");
            }
            let tok = self.next().unwrap();
            let _variadic: usize = self.text(tok).parse().unwrap();

            todo!("handle the variadic functions")
        } else {
            None
        };

        slt_builder.new_region(slt);
        let child_mut = slt.last_children_mut().unwrap();
        let mut stmts = Vec::new();
        while !self.check_next(T![CFnDecl]) {
            stmts.push(self.statement(slt_builder, child_mut));
        }

        self.consume(T![CFnDecl]);

        Fn {
            id,
            stmts,
            variadic,
        }
    }
}
