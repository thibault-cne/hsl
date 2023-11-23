#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    Op { op: Op, value: Lit },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    NegInt(u32),
    Int(u32),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        var_name: String,
        value: Box<Expr>,
    },
    Assignment {
        var_name: Box<Expr>,
        initial_value: Box<Expr>,
        operations: Vec<Expr>,
    },
    Print {
        value: Box<Expr>,
    },
    IfStmt {
        condition: Box<Expr>,
        body: Vec<Stmt>,
        else_stmt: Option<Vec<Stmt>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Main { body: Vec<Stmt> },
}

impl TryFrom<crate::lexer::token::TokenKind> for Op {
    type Error = ();

    fn try_from(value: crate::lexer::token::TokenKind) -> Result<Self, Self::Error> {
        match value {
            T![add] => Ok(Self::Add),
            T![sub] => Ok(Self::Sub),
            T![div] => Ok(Self::Div),
            T![mul] => Ok(Self::Mul),
            T![mod] => Ok(Self::Mod),
            _ => Err(()),
        }
    }
}
