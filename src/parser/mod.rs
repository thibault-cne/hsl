use crate::ir::Program;
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;

pub mod expression;
pub mod literal;
pub mod program;
pub mod slt;
pub mod statement;

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: std::iter::Peekable<I>,
}

impl<'input> Parser<'input, Lexer<'input>> {
    pub fn new(input: &'input str) -> Self {
        Parser {
            input,
            tokens: Lexer::new(input).peekable(),
        }
    }
}

impl<'input, I> Parser<'input, I>
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
    }

    pub(crate) fn parse(&mut self) -> Program {
        // TODO: parse functions declaration before
        self.consume(T![OProgram]);

        let mut stmts = Vec::new();
        while !self.check_next(T![CProgram]) {
            stmts.push(self.statement());
        }

        self.consume(T![CProgram]);
        self.consume(T![EOF]);

        Program { stmts }
    }
}
