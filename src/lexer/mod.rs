use rules::Rule;
use token::{Span, Token};

#[macro_use]
pub(crate) mod token;

mod rules;

pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
    eof: bool,
    rules: Vec<Rule>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Lexer {
        Lexer {
            input,
            position: 0,
            eof: false,
            rules: rules::get_rules(),
        }
    }

    fn valid_token(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if next.is_whitespace() {
            (
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_whitespace())
                    .last()
                    .unwrap()
                    .0
                    + 1,
                T![ws],
            )
        } else {
            self.rules
                .iter()
                .rev()
                .filter_map(|rule| Some(((rule.matches)(input)?, rule.kind)))
                .max_by_key(|&(len, _)| len)?
        };

        let start = self.position;
        self.position += len;
        Some(Token {
            kind,
            span: Span {
                start,
                end: start + len,
            },
        })
    }

    fn invalid_token(&mut self, input: &str) -> Token {
        let start = self.position;
        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());

        debug_assert!(len <= input.len());

        self.position = start + len;
        Token {
            kind: T![err],
            span: Span {
                start,
                end: start + len,
            },
        }
    }

    pub fn next_token(&mut self, input: &str) -> Token {
        self.valid_token(input)
            .unwrap_or_else(|| self.invalid_token(input))
    }

    #[cfg(test)]
    pub fn tokenize(&mut self) -> Vec<Token> {
        self.collect()
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            if self.eof {
                return None;
            }
            self.eof = true;
            Some(Token {
                kind: T![EOF],
                span: Span {
                    start: self.position,
                    end: self.position,
                },
            })
        } else {
            Some(self.next_token(&self.input[self.position..]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_tokens {
        ($tokens:ident, [$($kind:expr),*]) => {
            let mut it = $tokens.iter();
            $(
                let token = it.next().expect("not enough tokens");
                assert_eq!(token.kind, $kind);
            )*
        };
    }

    #[test]
    fn unknown_input() {
        let input = "$$$$$$+a";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(tokens, [T![err], T![EOF]]);
    }

    #[test]
    fn unknown_input_with_whitespace() {
        let input = "   $$  $$  $$";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(
            tokens,
            [T![ws], T![err], T![ws], T![err], T![ws], T![err], T![EOF]]
        );
    }

    #[test]
    fn keywords() {
        let input = r#"
            A long time ago in a galaxy far, far away...
                I am a big deal in the resistance. finn
                Who, mesa ? 10
            May the force be with you.
        "#;
        let input = unindent::unindent(input);
        let mut lexer = Lexer::new(&input);
        let tokens: Vec<_> = lexer
            .tokenize()
            .into_iter()
            .filter(|t| t.kind != T![ws])
            .collect();
        assert_tokens!(
            tokens,
            [
                T![start],
                T![let],
                T![ident],
                T![init],
                T![int],
                T![end],
                T![EOF]
            ]
        );
    }

    #[test]
    fn keywords_with_print_call() {
        let input = r#"
            A long time ago in a galaxy far, far away...
                I am a big deal in the resistance. finn
                Who, mesa ? 10

                You're eyes can deceive you; don't trust them. finn
            May the force be with you.
        "#;
        let input = unindent::unindent(input);
        let mut lexer = Lexer::new(&input);
        let tokens: Vec<_> = lexer
            .tokenize()
            .into_iter()
            .filter(|t| t.kind != T![ws])
            .collect();
        assert_tokens!(
            tokens,
            [
                T![start],
                T![let],
                T![ident],
                T![init],
                T![int],
                T![print],
                T![ident],
                T![end],
                T![EOF]
            ]
        );
    }
}
