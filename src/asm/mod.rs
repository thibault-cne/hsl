use std::io::Write;

use crate::parser::ast;
use crate::parser::slt::{ChildIterator, NavigableSlt};

mod a64;

pub use a64::A64Compiler;

pub trait Compiler<W: Write> {
    fn new(writer: W) -> Self;
    fn evaluate_expression(
        &mut self,
        ast: &ast::Expr,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String>;
    fn evaluate_item(
        &mut self,
        ast: &ast::Item,
        slt: &NavigableSlt<'_>,
        childs: &mut ChildIterator<'_>,
    ) -> Result<(), String>;
    fn evaluate_statement(
        &mut self,
        ast: &ast::Stmt,
        slt: &NavigableSlt<'_>,
        childs: &mut ChildIterator<'_>,
    ) -> Result<(), String>;
}

pub fn evaluate<W: Write, C: Compiler<W>>(
    ast: Vec<ast::Item>,
    slt: &NavigableSlt<'_>,
    mut compiler: C,
) -> Result<(), String> {
    let mut main = None;

    for node in ast {
        match &node {
            ast::Item::Main { .. } => {
                main = Some(node);
            }
        }
    }

    compiler.evaluate_item(&main.unwrap(), slt, &mut slt.childs())
}
