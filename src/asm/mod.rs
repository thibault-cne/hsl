use std::io::{BufRead, Write};

use crate::parser::ast;
use crate::parser::slt::NavigableSlt;

mod macos;

pub use macos::MacOsARM;

pub trait Compiler {
    fn new() -> Self;
    fn evaluate_expression<R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &ast::Expr,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String>;
    fn evaluate_item<R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &ast::Item,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String>;
    fn evaluate_statement<R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &ast::Stmt,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String>;
}

pub struct State<R, W, C> {
    _reader: R,
    writer: W,
    _compiler: std::marker::PhantomData<C>,
}

impl<R, W, C: Compiler> State<R, W, C>
where
    R: BufRead,
    W: Write,
{
    pub fn new(_reader: R, writer: W) -> State<R, W, C> {
        State {
            _reader,
            writer,
            _compiler: std::marker::PhantomData,
        }
    }
}

pub fn evaluate<R, W, C: Compiler>(
    ast: Vec<ast::Item>,
    reader: R,
    writer: W,
    slt: &NavigableSlt<'_>,
    mut compiler: C,
) -> Result<(), String>
where
    R: BufRead,
    W: Write,
{
    let mut main = None;
    let state: &mut State<R, W, C> = &mut State::new(reader, writer);

    for node in ast {
        match &node {
            ast::Item::Main { .. } => {
                main = Some(node);
            }
        }
    }

    compiler.evaluate_item(&main.unwrap(), state, slt)
}
