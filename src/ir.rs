//! Intermediate Representation (IR) of the HSL language
#![allow(dead_code)]

use crate::lexer::token::TokenKind;

pub struct Program<'prog> {
    pub func: Vec<Fn<'prog>>,
    pub extrn: Vec<Extrn<'prog>>,
}

pub struct Extrn<'prog> {
    pub id: &'prog str,

    // Tell if the function has a variadic parameter and if so the value of variadic is
    // the number of fixed parameters
    pub variadic: Option<usize>,
}

pub struct Fn<'prog> {
    pub id: &'prog str,
    pub stmts: Vec<Stmt<'prog>>,

    // Tell if the function has a variadic parameter and if so the value of variadic is
    // the number of fixed parameters
    pub variadic: Option<usize>,
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
    Lit(Lit<'prog>),
    ID(&'prog str),
}

pub enum Lit<'prog> {
    Int(i64),
    Str(&'prog str),
    Bool(bool),
}

impl<'prog> Program<'prog> {
    pub fn new() -> Self {
        Self {
            func: Vec::new(),
            extrn: Vec::new(),
        }
    }

    pub fn get_fn_variadic(&self, id: &'prog str) -> Option<usize> {
        if let Some(func) = self.func.iter().find(|f| f.id == id) {
            func.variadic
        } else if let Some(func) = self.extrn.iter().find(|f| f.id == id) {
            func.variadic
        } else {
            None
        }
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
