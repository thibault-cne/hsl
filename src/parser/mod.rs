use crate::{
    lexer::token::{Token, TokenKind},
    lexer::Lexer,
};

pub(crate) mod ast;
pub(crate) mod slt;

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: std::iter::Peekable<I>,
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str) -> Parser<TokenIter> {
        Parser {
            input,
            tokens: TokenIter::new(input).peekable(),
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

    pub(crate) fn peek(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|token| token.kind)
            .unwrap_or(T![EOF])
    }

    /// Check if the next token is of a given kind
    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
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

    pub(crate) fn parse(&mut self) -> ast::Node {
        // TODO: parse functions declaration before
        while !self.at(T![start]) {
            self.next();
        }
        self.consume(T![start]);

        let mut body = Vec::new();
        while !self.at(T![end]) {
            body.push(self.build_ast());
        }

        self.consume(T![end]);
        self.consume(T![EOF]);

        ast::Node::Main(body)
    }
}

pub struct TokenIter<'input> {
    lexer: Lexer<'input>,
}

impl<'input> TokenIter<'input> {
    pub fn new(input: &'input str) -> TokenIter {
        TokenIter {
            lexer: Lexer::new(input),
        }
    }
}

impl<'input> Iterator for TokenIter<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_token = self.lexer.next()?;
            if !matches!(next_token.kind, T![ws] | T![comment]) {
                return Some(next_token);
            }
        }
    }
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub(crate) fn build_ast(&mut self) -> ast::Node {
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
                let value = self.build_ast();
                ast::Node::DeclareLiteral(var_name, Box::new(value))
            }
            T![print] => {
                self.consume(T![print]);
                let arg = self.build_ast();

                ast::Node::Print(Box::new(arg))
            }
            lit @ T![string] | lit @ T![int] | lit @ T![float] => {
                let literal_text = {
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };

                match lit {
                    T![int] => {
                        ast::Node::Integer(literal_text.parse().unwrap_or_else(|_| {
                            panic!("Invalid integer literal: `{}`", literal_text)
                        }))
                    }
                    T![string] => {
                        ast::Node::String(literal_text[1..(literal_text.len() - 1)].to_string())
                    }
                    T![float] => ast::Node::Float(literal_text.parse().unwrap_or_else(|_| {
                        panic!("Invalid floating point literal: `{}`", literal_text)
                    })),
                    _ => unreachable!(),
                }
            }
            T![ident] => {
                let name = {
                    let ident_token = self.next().unwrap();
                    self.text(ident_token)
                };

                ast::Node::Identifier(name.to_string())
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        }
    }
}
