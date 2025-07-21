use std::fmt;

#[macro_export]
macro_rules! T {
    [EOF] => { $crate::lexer::token::TokenKind::EOF };
    [ParseError] => { $crate::lexer::token::TokenKind::ParseError };
    [ID] => { $crate::lexer::token::TokenKind::ID };
    [String] => { $crate::lexer::token::TokenKind::String };
    [CharLit] => { $crate::lexer::token::TokenKind::CharLit };
    [IntLit] => { $crate::lexer::token::TokenKind::IntLit };
    [True] => { $crate::lexer::token::TokenKind::True };
    [False] => { $crate::lexer::token::TokenKind::False };
    [OParen] => { $crate::lexer::token::TokenKind::OParen };
    [CParen] => { $crate::lexer::token::TokenKind::CParen };
    [Dot] => { $crate::lexer::token::TokenKind::Dot };
    [Dash] => { $crate::lexer::token::TokenKind::Dash };
    [Not] => { $crate::lexer::token::TokenKind::Not };
    [Mul] => { $crate::lexer::token::TokenKind::Mul };
    [Div] => { $crate::lexer::token::TokenKind::Div };
    [Mod] => { $crate::lexer::token::TokenKind::Mod };
    [Plus] => { $crate::lexer::token::TokenKind::Plus };
    [PlusPlus] => { $crate::lexer::token::TokenKind::PlusPlus };
    [Minus] => { $crate::lexer::token::TokenKind::Minus };
    [MinusMinus] => { $crate::lexer::token::TokenKind::MinusMinus };
    [Eq] => { $crate::lexer::token::TokenKind::Eq };
    [Less] => { $crate::lexer::token::TokenKind::Less };
    [LessEq] => { $crate::lexer::token::TokenKind::LessEq };
    [Greater] => { $crate::lexer::token::TokenKind::Greater };
    [GreaterEq] => { $crate::lexer::token::TokenKind::GreaterEq };
    [Comma] => { $crate::lexer::token::TokenKind::Comma };
    [OAssign] => { $crate::lexer::token::TokenKind::OAssign };
    [Assign] => { $crate::lexer::token::TokenKind::Assign };
    [CAssign] => { $crate::lexer::token::TokenKind::CAssign };
    [If] => { $crate::lexer::token::TokenKind::If };
    [IfEnd] => { $crate::lexer::token::TokenKind::IfEnd};
    [Else] => { $crate::lexer::token::TokenKind::Else };
    [Let] => { $crate::lexer::token::TokenKind::Let };
    [OProgram] => { $crate::lexer::token::TokenKind::OProgram };
    [CProgram] => { $crate::lexer::token::TokenKind::CProgram };
    [OFnCall] => { $crate::lexer::token::TokenKind::OFnCall };
    [CFnCall] => { $crate::lexer::token::TokenKind::CFnCall };
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Self { kind, span }
    }

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
    // Terminal
    EOF,
    ParseError,

    // Values
    ID,
    String,
    CharLit,
    IntLit,
    True,
    False,

    // Puncts
    OParen,
    CParen,
    Dot,
    Dash,
    Not,
    Mul,
    Div,
    Mod,
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Eq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Comma,

    // Keywords
    OAssign,
    Assign,
    CAssign,
    If,
    IfEnd,
    Else,
    Let,
    OProgram,
    CProgram,
    OFnCall,
    CFnCall,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                T![EOF] => "<eof>",
                T![ParseError] => "<?>",
                T![ID] => "Identifier",
                T![String] => "String",
                T![CharLit] => "Char literal",
                T![IntLit] => "Integer literal",
                T![True] => "True",
                T![False] => "False",
                T![OParen] => "Open parenthesis",
                T![CParen] => "Close parenthesis",
                T![Dot] => "Dot",
                T![Dash] => "Dash",
                T![Not] => "Not",
                T![Mul] => "Mul",
                T![Div] => "Div",
                T![Mod] => "Mod",
                T![Plus] => "Plus",
                T![PlusPlus] => "PlusPlus",
                T![Minus] => "Minus",
                T![MinusMinus] => "MinusMinus",
                T![Eq] => "Eq",
                T![Less] => "Less",
                T![LessEq] => "LessEq",
                T![Greater] => "Greater",
                T![GreaterEq] => "GreaterEq",
                T![Comma] => "Comma",
                T![OAssign] => "Open assign",
                T![Assign] => "Assign",
                T![CAssign] => "Close assign",
                T![If] => "If",
                T![IfEnd] => "IfEnd",
                T![Else] => "Else",
                T![Let] => "Let",
                T![OProgram] => "Opening program",
                T![CProgram] => "Closing program",
                T![OFnCall] => "Opening function call",
                T![CFnCall] => "Closing function call",
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
        assert_eq!(T![String].to_string(), "String");
        assert_eq!(T![EOF].to_string(), "<eof>");
    }
}
