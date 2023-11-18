use std::io::{BufRead, Write};

use crate::parser::ast::Node;
use crate::parser::slt::NavigableSlt;

mod macos;

pub use macos::MacOsARM;

pub trait Compiler {
    fn new() -> Self;
    fn evaluate_node<'n, R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &Node,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'n>
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

pub fn evaluate<'n, R, W, C: Compiler>(
    ast: Vec<Node>,
    reader: R,
    writer: W,
    slt: &NavigableSlt<'n>,
    mut compiler: C,
) -> Result<(), String>
where
    R: BufRead,
    W: Write,
{
    let mut main = Node::Noop;
    let state: &mut State<R, W, C> = &mut State::new(reader, writer);

    for node in ast {
        match &node {
            Node::Main(_) => {
                main = node;
            }
            _ => unreachable!(), // TODO: Get actual error message
        }
    }

    compiler.evaluate_node(&main, state, slt)
}
