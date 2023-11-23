use std::io::Write;

use super::A64Compiler;
use crate::parser::{ast, slt::NavigableSlt};

impl<W: Write> A64Compiler<W> {
    pub fn expression(&mut self, ast: &ast::Expr, _slt: &NavigableSlt<'_>) -> Result<(), String> {
        match ast {
            ast::Expr::Literal(_) => Ok(()),
            ast::Expr::Ident(_) => Ok(()),
            ast::Expr::Op { op, value } => {
                // TODO: handle string, float and bool
                let value = match value {
                    ast::Lit::Int(int) => int,
                    _ => unreachable!(),
                };
                match op {
                    ast::Op::Add => self.add("x8", "x8", &format!("{:#02x}", value)),
                    ast::Op::Sub => self.sub("x8", "x8", &format!("{:#02x}", value)),
                    ast::Op::Mul => {
                        self.mov("x9", &format!("{:#02x}", value));
                        self.mul("x8", "x8", "x9");
                    }
                    ast::Op::Div => {
                        self.mov("x9", &format!("{:#02x}", value));
                        self.sdiv("x8", "x8", "x9");
                    }
                    ast::Op::Mod => {
                        self.mov("x9", &format!("{:#02x}", value));
                        self.udiv("x10", "x8", "x9");
                        self.msub("x8", "x10", "x9", "x8");
                    }
                }

                Ok(())
            }
        }
    }
}
