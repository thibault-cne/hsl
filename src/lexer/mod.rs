use std::char;

use rules::{KEYWORDS, PUNCTS};
use token::{Span, Token};

#[macro_use]
pub(crate) mod token;

mod rules;

#[derive(Debug, Clone, Copy)]
pub struct ParsePoint {
    position: usize,
    line_start: usize,
    line_number: usize,
}

#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    input: &'input [u8],
    parse_point: ParsePoint,
    has_eof: bool,

    int_number: u64,
    char_lit: char,
    string: String,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Lexer<'input> {
        Lexer {
            input: input.as_bytes(),
            parse_point: ParsePoint {
                position: 0,
                line_start: 0,
                line_number: 0,
            },
            has_eof: false,

            int_number: 0,
            char_lit: ' ',
            string: String::new(),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.parse_point.position >= self.input.len()
    }

    fn peek_char(&self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            Some(self.input[self.parse_point.position] as char)
        }
    }

    fn skip_char(&mut self) {
        assert!(!self.is_eof());

        let x = self.parse_point.position;
        self.parse_point.position += 1;
        if self.input[x] as char == '\n' {
            self.parse_point.line_number += 1;
            self.parse_point.line_start = x;
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.skip_char()
            } else {
                break;
            }
        }
    }

    fn skip_prefix(&mut self, prefix: &str) -> bool {
        let saved_point = self.parse_point;
        let mut ptr = 0;

        while ptr < prefix.len() {
            let Some(x) = self.peek_char() else {
                self.parse_point = saved_point;
                return false;
            };

            if x != prefix.chars().nth(ptr).unwrap() {
                self.parse_point = saved_point;
                return false;
            }

            self.skip_char();
            ptr += 1;
        }

        true
    }

    fn skip_until(&mut self, prefix: &str) {
        while !self.is_eof() && !self.skip_prefix(prefix) {
            self.skip_char();
        }
    }

    fn parse_number(&mut self, radix: Radix) -> Option<()> {
        while let Some(x) = self.peek_char() {
            let Some(d) = x.to_digit(radix as u32) else {
                break;
            };

            let Some(r) = self.int_number.checked_mul(radix as u64) else {
                error!("invalid digit character `{x}`, reached overflow by multipying `{}` by radix `{radix}`", self.int_number);
                return None;
            };
            self.int_number = r;

            let Some(r) = self.int_number.checked_add(d as u64) else {
                error!(
                    "invalid digit character `{x}`, reached overflow by adding `{}` and `{d}`",
                    self.int_number
                );
                return None;
            };
            self.int_number = r;
            self.skip_char();
        }

        Some(())
    }

    fn parse_string(&mut self, delim: char) -> Option<()> {
        while let Some(x) = self.peek_char() {
            match x {
                '\\' => {
                    self.skip_char();
                    let x = self.peek_char()?;
                    let x = match x {
                        '0' => '\0',
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        x if x == delim => delim,
                        '\\' => '\\',
                        _ => {
                            error!("invalid escaped character `{x}`");
                            return None;
                        }
                    };

                    self.string.push(x);
                    self.skip_char();
                }
                x if x == delim => break,
                _ => {
                    self.string.push(x);
                    self.skip_char();
                }
            }
        }

        Some(())
    }

    pub fn next_token(&mut self) -> Token {
        'comment: loop {
            self.skip_whitespaces();

            // This is the comment prefix so we skip everything starting at this point
            if self.skip_prefix("<(-.-)>") {
                self.skip_until("\n");
                continue 'comment;
            }

            break 'comment;
        }

        let Some(x) = self.peek_char() else {
            self.has_eof = true;
            return Token::new(
                T![EOF],
                Span {
                    start: self.parse_point.position,
                    end: self.parse_point.position,
                    line: self.parse_point.line_number,
                },
            );
        };

        // Check if we have a punctuation
        {
            let saved_position = self.parse_point.position;
            if let Some(&kind) = PUNCTS
                .iter()
                .find(|(prefix, _)| self.skip_prefix(prefix))
                .map(|(_, kind)| kind)
            {
                return Token::new(
                    kind,
                    Span {
                        start: saved_position,
                        end: self.parse_point.position,
                        line: self.parse_point.line_number,
                    },
                );
            }
        }

        // Check if we have a keyword
        {
            let saved_position = self.parse_point.position;
            if let Some(&kind) = KEYWORDS
                .iter()
                .find(|(prefix, _)| self.skip_prefix(prefix))
                .map(|(_, kind)| kind)
            {
                return Token::new(
                    kind,
                    Span {
                        start: saved_position,
                        end: self.parse_point.position,
                        line: self.parse_point.line_number,
                    },
                );
            }
        }

        // Check if we have an identifier
        if is_identifier_start(x) {
            let saved_position = self.parse_point.position;
            while let Some(x) = self.peek_char() {
                if is_identifier(x) {
                    self.skip_char();
                } else {
                    break;
                }
            }

            return Token::new(
                T![ID],
                Span {
                    line: self.parse_point.line_number,
                    start: saved_position,
                    end: self.parse_point.position,
                },
            );
        }

        // Check if we have a number
        {
            let saved_position = self.parse_point.position;
            if self.skip_prefix("0x") {
                let Some(_) = self.parse_number(Radix::Hex) else {
                    error!(
                        "line {} position {}: invalid hexadecimal number",
                        self.parse_point.line_number,
                        self.parse_point.position - self.parse_point.line_start
                    );

                    return Token::new(
                        T![ParseError],
                        Span {
                            line: self.parse_point.line_number,
                            start: self.parse_point.position,
                            end: self.parse_point.position,
                        },
                    );
                };

                self.int_number = 0;
                return Token::new(
                    T![IntLit],
                    Span {
                        line: self.parse_point.line_number,
                        start: saved_position,
                        end: self.parse_point.position,
                    },
                );
            }

            if self.skip_prefix("0") {
                let Some(_) = self.parse_number(Radix::Oct) else {
                    error!(
                        "line {} position {}: invalid octal number",
                        self.parse_point.line_number,
                        self.parse_point.position - self.parse_point.line_start
                    );

                    return Token::new(
                        T![ParseError],
                        Span {
                            line: self.parse_point.line_number,
                            start: self.parse_point.position,
                            end: self.parse_point.position,
                        },
                    );
                };

                self.int_number = 0;
                return Token::new(
                    T![IntLit],
                    Span {
                        line: self.parse_point.line_number,
                        start: saved_position,
                        end: self.parse_point.position,
                    },
                );
            }

            if x.is_digit(Radix::Dec as u32) {
                let Some(_) = self.parse_number(Radix::Dec) else {
                    error!(
                        "line {} position {}: invalid decimal number",
                        self.parse_point.line_number,
                        self.parse_point.position - self.parse_point.line_start
                    );

                    return Token::new(
                        T![ParseError],
                        Span {
                            line: self.parse_point.line_number,
                            start: self.parse_point.position,
                            end: self.parse_point.position,
                        },
                    );
                };

                self.int_number = 0;
                return Token::new(
                    T![IntLit],
                    Span {
                        line: self.parse_point.line_number,
                        start: saved_position,
                        end: self.parse_point.position,
                    },
                );
            }
        }

        // Check if we have a string
        if x == '"' {
            let saved_position = self.parse_point.position;
            self.skip_char();
            self.string.clear();
            if self.parse_string('"').is_none() || self.is_eof() {
                return Token::new(
                    T![ParseError],
                    Span {
                        line: self.parse_point.line_number,
                        start: self.parse_point.position,
                        end: self.parse_point.position,
                    },
                );
            }
            self.skip_char();
            return Token::new(
                T![String],
                Span {
                    line: self.parse_point.line_number,
                    start: saved_position,
                    end: self.parse_point.position,
                },
            );
        }

        if x == '\'' {
            let saved_position = self.parse_point.position;
            self.skip_char();
            self.string.clear();
            if self.parse_string('\'').is_none() || self.is_eof() {
                return Token::new(
                    T![ParseError],
                    Span {
                        line: self.parse_point.line_number,
                        start: self.parse_point.position,
                        end: self.parse_point.position,
                    },
                );
            }
            self.skip_char();

            let chars: Vec<char> = self.string.chars().collect();

            // TODO: add the error diagnostic and handling
            if chars.len() != 1 {
                return Token::new(
                    T![ParseError],
                    Span {
                        line: self.parse_point.line_number,
                        start: self.parse_point.position,
                        end: self.parse_point.position,
                    },
                );
            }

            self.char_lit = chars[0];

            return Token::new(
                T![CharLit],
                Span {
                    line: self.parse_point.line_number,
                    start: saved_position,
                    end: self.parse_point.position,
                },
            );
        }

        self.skip_char();
        Token::new(
            T![ParseError],
            Span {
                line: self.parse_point.line_number,
                start: self.parse_point.position,
                end: self.parse_point.position,
            },
        )
    }

    #[cfg(test)]
    pub fn tokenize(&mut self) -> Vec<Token> {
        self.collect()
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_eof {
            None
        } else {
            Some(self.next_token())
        }
    }
}

fn is_identifier(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Radix {
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

impl std::fmt::Display for Radix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
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
        let input = "$+a";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(tokens, [T![ParseError], T![ParseError], T![ID], T![EOF]]);
    }

    #[test]
    fn unknown_input_with_whitespace() {
        let input = "   $$  $$  $$";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(
            tokens,
            [
                T![ParseError],
                T![ParseError],
                T![ParseError],
                T![ParseError],
                T![ParseError],
                T![ParseError],
                T![EOF]
            ]
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
        let mut lexer = Lexer::new(&input);
        let tokens: Vec<_> = lexer.tokenize();
        assert_tokens!(
            tokens,
            [
                T![OFnDecl1],
                T![ID],
                T![OFnDecl2],
                T![Let],
                T![ID],
                T![Assign],
                T![IntLit],
                T![CFnDecl],
                T![EOF]
            ]
        );
    }

    #[test]
    fn keywords_with_print_call_integer() {
        let input = r#"
            A long time ago in a galaxy far, far away...
                I am a big deal in the resistance. finn
                Who, mesa ? 10

                Execute order print
                    finn
                Order executed
            May the force be with you.
        "#;
        let mut lexer = Lexer::new(&input);
        let tokens: Vec<_> = lexer.tokenize();
        assert_tokens!(
            tokens,
            [
                T![OFnDecl1],
                T![ID],
                T![OFnDecl2],
                T![Let],
                T![ID],
                T![Assign],
                T![IntLit],
                T![OFnCall],
                T![ID],
                T![ID],
                T![CFnCall],
                T![CFnDecl],
                T![EOF]
            ]
        );
    }

    #[test]
    fn keywords_with_print_call_string() {
        let input = r#"
            A long time ago in a galaxy far, far away...
                I am a big deal in the resistance. finn
                Who, mesa ? "Finn"

                Execute order print
                    finn
                Order executed
            May the force be with you.
        "#;
        let mut lexer = Lexer::new(&input);
        let tokens: Vec<_> = lexer.tokenize();
        assert_tokens!(
            tokens,
            [
                T![OFnDecl1],
                T![ID],
                T![OFnDecl2],
                T![Let],
                T![ID],
                T![Assign],
                T![String],
                T![OFnCall],
                T![ID],
                T![ID],
                T![CFnCall],
                T![CFnDecl],
                T![EOF]
            ]
        );
    }
}
