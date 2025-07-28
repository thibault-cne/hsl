use crate::ir::{Fn, Program};
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;

pub mod expression;
pub mod literal;
pub mod program;
pub mod slt;
pub mod statement;

pub struct Parser<'prog, I>
where
    I: Iterator<Item = Token>,
{
    input: &'prog str,
    tokens: std::iter::Peekable<I>,

    id: &'prog str,
    pub has_main: bool,
}

impl<'prog> Parser<'prog, Lexer<'prog>> {
    pub fn new(input: &'prog str) -> Self {
        Parser {
            input,
            tokens: Lexer::new(input).peekable(),
            id: "",
            has_main: false,
        }
    }
}

impl<'prog, I> Parser<'prog, I>
where
    I: Iterator<Item = Token>,
{
    /// Get the source text of a token::Token
    pub fn text(&self, token: Token) -> &'prog str {
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
            panic!(
                "Expected to consume `{}`, but there was no next token.",
                expected
            )
        });
        assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}` instead",
            expected, token.kind
        );

        if matches!(token.kind, T![ID]) {
            // In this case we update the id state of the parser
            self.id = self.text(token);
        }
    }

    pub(crate) fn parse(&mut self) -> Program {
        let mut func = Vec::new();
        while !self.check_next(T![EOF]) {
            func.push(self.parse_function());
        }
        self.consume(T![EOF]);

        Program { func }
    }

    fn parse_function(&mut self) -> Fn {
        self.consume(T![OFnDecl1]);
        self.consume(T![ID]);

        if self.id == "galaxy" {
            self.has_main = true;
        }

        let id = self.id.to_string();

        self.consume(T![OFnDecl2]);

        let mut stmts = Vec::new();
        while !self.check_next(T![CFnDecl]) {
            stmts.push(self.statement());
        }

        self.consume(T![CFnDecl]);

        Fn { id, stmts }
    }
}
