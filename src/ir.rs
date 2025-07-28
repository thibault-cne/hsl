//! Intermediate Representation (IR) of the HSL language

use crate::lexer::token::TokenKind;

pub struct Program<'prog> {
    pub func: Vec<Fn<'prog>>,
}

pub struct Fn<'prog> {
    pub id: &'prog str,
    pub stmts: Vec<Stmt<'prog>>,
}

pub enum Op {
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub enum Stmt<'prog> {
    Let {
        id: &'prog str,
        value: Expr<'prog>,
    },
    FnCall {
        id: &'prog str,
        args: Vec<Expr<'prog>>,
    },
    Assign {
        id: &'prog str,
        ops: Vec<Unop<'prog>>,
    },
}

pub struct Unop<'prog> {
    pub value: Expr<'prog>,
    pub op: Op,
}

pub enum Expr<'prog> {
    FnCall {
        id: &'prog str,
        args: Vec<Expr<'prog>>,
    },
    Lit {
        id: &'prog str,
        lit: Lit<'prog>,
    },
    ID(&'prog str),
}

pub enum Lit<'prog> {
    Int(i64),
    Str(&'prog str),
    Bool(bool),
}

impl<'prog> Program<'prog> {
    pub fn new() -> Self {
        Self { func: Vec::new() }
    }
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
