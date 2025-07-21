//! Intermediate Representation (IR) of the HSL language

use crate::lexer::token::TokenKind;

pub struct Program {
    pub stmts: Vec<Stmt>,
}

pub enum Op {
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub enum Stmt {
    FnCall { id: String, args: Vec<Expr> },
    Let { id: String, value: Expr },
    Assign { id: String, ops: Vec<Unop> },
}

pub struct Unop {
    pub value: Expr,
    pub op: Op,
}

pub enum Expr {
    Lit(Lit),
    ID(String),
}

pub enum Lit {
    Int(i64),
    Str(String),
    Bool(bool),
}

impl TryFrom<TokenKind> for Op {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            T![Plus] => Ok(Self::Add),
            T![Eq] => Ok(Self::Eq),
            T![Minus] => Ok(Self::Sub),
            T![Div] => Ok(Self::Div),
            T![Mul] => Ok(Self::Mul),
            T![Mod] => Ok(Self::Mod),
            _ => Err(()),
        }
    }
}
