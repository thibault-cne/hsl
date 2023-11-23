use std::io::Write;

use super::Compiler;
use crate::parser::{
    ast,
    slt::{ChildIterator, NavigableSlt},
};

mod asm;
mod expression;
mod statement;

pub struct A64Compiler<W: Write> {
    string_literals: Vec<(String, String)>,
    writer: W,
}

impl<W: Write> Compiler<W> for A64Compiler<W> {
    fn new(writer: W) -> Self {
        A64Compiler {
            string_literals: vec![
                ("str_format".to_string(), "%s\\n".to_string()),
                ("int_format".to_string(), "%d\\n".to_string()),
            ],
            writer,
        }
    }

    fn evaluate_expression(
        &mut self,
        ast: &ast::Expr,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String> {
        self.expression(ast, slt)
    }

    fn evaluate_statement(
        &mut self,
        ast: &ast::Stmt,
        slt: &NavigableSlt<'_>,
        childs: &mut ChildIterator<'_>,
    ) -> Result<(), String> {
        self.statement(ast, slt, childs)
    }

    fn evaluate_item(
        &mut self,
        ast: &ast::Item,
        _slt: &NavigableSlt<'_>,
        childs: &mut ChildIterator<'_>,
    ) -> Result<(), String> {
        match ast {
            ast::Item::Main { body } => {
                writeln!(self.writer, ".global _start\n.align 2\n_start:")
                    .map_err(|x| x.to_string())?;
                self.comment("Program header");
                self.stp("x29", "lr", "sp", Some(Index::pre(-16)));
                self.mov("x29", "sp");
                self.skip_line();
                self.comment("Jump to main function");
                self.b("main", "");

                self.label("start_end");

                let child = childs.next().unwrap();
                let mut main_childs = child.childs();
                let stack_size = child.slt.variables.len() * 16;

                self.comment("Pop the stack");
                self.add("sp", "sp", &format!("{:#02x}", stack_size));
                self.ldp("x29", "lr", "sp", Some(Index::post(16)));

                writeln!(
                    self.writer,
                    "\n\tmov     x0, #0\n\tmov     x16, #1\n\tsvc     0"
                )
                .map_err(|x| x.to_string())?;
                self.skip_line();
                self.label("main");

                for stmt in body {
                    self.evaluate_statement(stmt, &child, &mut main_childs)?;
                }

                self.comment("Jump to end of program");
                self.b("start_end", "");

                writeln!(self.writer, ".data").map_err(|x| x.to_string())?;
                for (name, s) in self.string_literals.iter() {
                    writeln!(self.writer, "\t{}:      .asciz  \"{}\"", name, s)
                        .map_err(|x| x.to_string())?;
                }
                Ok(())
            }
        }
    }
}

pub struct Index {
    offset: i32,
    position: Position,
}

impl Index {
    fn pre(offset: i32) -> Index {
        Index {
            offset,
            position: Position::Pre,
        }
    }

    fn offset(offset: i32) -> Index {
        Index {
            offset,
            position: Position::Offset,
        }
    }

    fn post(offset: i32) -> Index {
        Index {
            offset,
            position: Position::Post,
        }
    }

    fn format_offset(&self) -> String {
        if self.offset < 0 {
            format!("-{:#02x}", self.offset.abs())
        } else {
            format!("{:#02x}", self.offset.abs())
        }
    }
}

enum Position {
    Pre,
    Post,
    Offset,
}
