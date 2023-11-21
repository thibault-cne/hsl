use std::fmt;

#[macro_export]
macro_rules! T {
    [string] => { $crate::lexer::token::TokenKind::String };
    [comment] => { $crate::lexer::token::TokenKind::Comment };
    [int] => { $crate::lexer::token::TokenKind::Int };
    [bool] => { $crate::lexer::token::TokenKind::Boolean };
    [ident] => { $crate::lexer::token::TokenKind::Identifier };
    [let] => { $crate::lexer::token::TokenKind::KeywordLet };
    [init] => { $crate::lexer::token::TokenKind::KeywordInit };
    [start] => { $crate::lexer::token::TokenKind::BeginMain };
    [end] => { $crate::lexer::token::TokenKind::EndMain };
    [print] => { $crate::lexer::token::TokenKind::Print };
    [assign_start] => { $crate::lexer::token::TokenKind::AssignValueStart };
    [assign_end] => { $crate::lexer::token::TokenKind::AssignValueEnd };
    [set] => { $crate::lexer::token::TokenKind::SetValue };
    [add] => { $crate::lexer::token::TokenKind::Add };
    [sub] => { $crate::lexer::token::TokenKind::Substract };
    [mul] => { $crate::lexer::token::TokenKind::Multiply };
    [div] => { $crate::lexer::token::TokenKind::Divide };
    [mod] => { $crate::lexer::token::TokenKind::Modulus };
    [ws] => { $crate::lexer::token::TokenKind::Whitespace };
    [err] => { $crate::lexer::token::TokenKind::Error };
    [EOF] => { $crate::lexer::token::TokenKind::Eof };
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - <{}, {}>",
            self.kind, self.span.start, self.span.end
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    // Multiple characters
    String,
    Comment,
    Int,
    Boolean,
    Identifier,
    KeywordLet,
    KeywordInit,
    // Functions
    Print,
    // Assign values block
    AssignValueStart,
    AssignValueEnd,
    SetValue,
    // Operations
    Add,
    Substract,
    Multiply,
    Divide,
    Modulus,
    // Main delimiter
    BeginMain,
    EndMain,
    // Misc
    Whitespace,
    Error,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                T![string] => "String",
                T![comment] => "Comment",
                T![int] => "Int",
                T![bool] => "Boolean",
                T![ident] => "Identifier",
                T![let] => "Let",
                T![init] => "Initial value",
                T![start] => "Main start",
                T![end] => "Main end",
                T![print] => "<print>",
                T![assign_start] => "Assign value start",
                T![assign_end] => "Assign value end",
                T![set] => "Set value",
                T![add] => "Addition",
                T![sub] => "Substraction",
                T![mul] => "Multiplication",
                T![div] => "Division",
                T![mod] => "Mudulus",
                T![ws] => "<ws>",
                T![err] => "<?>",
                T![EOF] => "<EOF>",
            }
        )
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default, Debug)]
pub struct Span {
    // inclusive
    pub start: usize,
    // exclusive
    pub end: usize,
}

impl From<Span> for std::ops::Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Span {
        Span {
            start: range.start,
            end: range.end,
        }
    }
}

impl std::ops::Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[std::ops::Range::<usize>::from(index)]
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn token_kind_display() {
        assert_eq!(T![string].to_string(), "String");
        assert_eq!(T![ws].to_string(), "<ws>");
    }
}
