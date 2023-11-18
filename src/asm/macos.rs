use std::io::{BufRead, Write};

use super::{Compiler, State};
use crate::parser::{ast::Node, slt::NavigableSlt};

pub struct MacOsARM {
    string_literals: Vec<(String, String)>,
}

impl Compiler for MacOsARM {
    fn new() -> Self {
        MacOsARM {
            string_literals: vec![
                ("str_format".to_string(), "%s\\n".to_string()),
                ("int_format".to_string(), "%d\\n".to_string()),
            ],
        }
    }

    fn evaluate_node<R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &crate::parser::ast::Node,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String> {
        match ast {
            Node::DeclareLiteral(name, value) => {
                // TODO: add the value to the stack
                let variable = slt.find_variable(name).unwrap();
                writeln!(state.writer, "\t// Pushing variable {} to the stack", name)
                    .map_err(|e| e.to_string())?;
                match &variable.value {
                    crate::parser::slt::Value::String(s) => {
                        load_string(&mut state.writer, name, "x8");
                        str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                        self.string_literals.push((name.to_string(), s.to_string()))
                    }
                    crate::parser::slt::Value::Integer(i) => {
                        mov(&mut state.writer, "x8", &format!("#{}", i));
                        str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                    }
                    _ => (),
                }
                self.evaluate_node(value, state, slt)
            }
            Node::Integer(_) => Ok(()),
            Node::Float(_) => Ok(()),
            Node::String(_) => Ok(()),
            Node::Identifier(_ident) => {
                // TODO: load the value in the stack

                Ok(())
            }
            Node::Main(statements) => {
                writeln!(state.writer, ".global _start\n.align 2\n_start:")
                    .map_err(|x| x.to_string())?;
                stp(&mut state.writer, "x29", "lr", "sp", Some(Index::pre(-16)));

                let child = slt.childs().next().unwrap();

                for statement in statements {
                    self.evaluate_node(statement, state, &child)?;
                }

                let stack_size = child.slt.variables.len() * 16;
                add(
                    &mut state.writer,
                    "sp",
                    "sp",
                    &format!("0x{:x}", stack_size),
                );

                ldp(&mut state.writer, "x29", "lr", "sp", Some(Index::post(16)));

                writeln!(
                    state.writer,
                    "\n\tmov     x0, #0\n\tmov     x16, #1\n\tsvc     0"
                )
                .map_err(|x| x.to_string())?;

                writeln!(state.writer, ".data").map_err(|x| x.to_string())?;
                for (name, s) in self.string_literals.iter() {
                    writeln!(state.writer, "\t{}:      .asciz  \"{}\"", name, s)
                        .map_err(|x| x.to_string())?;
                }
                Ok(())
            }
            Node::Print(node) => {
                // Validate it's a value
                match &**node {
                    Node::Float(_) | Node::Boolean(_) | Node::String(_) | Node::Integer(_) => (),
                    Node::Identifier(ident) => {
                        let variable = slt.find_variable(ident).unwrap();
                        let format_name = match variable.value {
                            crate::parser::slt::Value::String(_) => {
                                ldr(
                                    &mut state.writer,
                                    "sp",
                                    "x8",
                                    Some(Index::offset(variable.offset.abs())),
                                );
                                str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                                "str_format"
                            }
                            crate::parser::slt::Value::Integer(_) => {
                                ldr(
                                    &mut state.writer,
                                    "sp",
                                    "x8",
                                    Some(Index::offset(variable.offset.abs())),
                                );
                                str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                                "int_format"
                            }
                            _ => "any",
                        };
                        load_string(&mut state.writer, format_name, "x0");
                    }
                    _ => return Err("Return not a value".to_string()),
                };

                // TODO: just call the printf function as everything must be
                // in the stack
                writeln!(state.writer, "\tbl _printf").expect("writer error");
                add(&mut state.writer, "sp", "sp", "0x10");
                Ok(())
            }
            _ => todo!(),
        }
    }
}

fn load_string<W: Write>(writer: &mut W, name: &str, register: &str) {
    writeln!(writer, "\tadrp {}, {}@PAGE", register, name).expect("writer error");
    writeln!(writer, "\tadd {}, {}, {}@PAGEOFF", register, register, name).expect("writer error");
}

fn str<W: Write>(writer: &mut W, src: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => writeln!(writer, "\tstr {}, [{}, #{}]!", src, dst, index.offset)
                .expect("writer error"),
            Position::Post => writeln!(writer, "\tstr {}, [{}], #{}", src, dst, index.offset)
                .expect("writer error"),
            Position::Offset => writeln!(writer, "\tstr {}, [{}, #{}]", src, dst, index.offset)
                .expect("writer error"),
        },
        None => writeln!(writer, "\tstr {}, [{}]", src, dst).expect("writer error"),
    }
}

fn stp<W: Write>(writer: &mut W, src1: &str, src2: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => writeln!(
                writer,
                "\tstp {}, {}, [{}, #{}]!",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Post => writeln!(
                writer,
                "\tstp {}, {}, [{}], #{}",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Offset => writeln!(
                writer,
                "\tstp {}, {}, [{}, #{}]",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
        },
        None => writeln!(writer, "\tstp {}, {}, [{}]", src1, src2, dst).expect("writer error"),
    }
}

fn ldr<W: Write>(writer: &mut W, src: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => writeln!(writer, "\tldr {}, [{}, #{}]!", dst, src, index.offset)
                .expect("writer error"),
            Position::Post => writeln!(writer, "\tldr {}, [{}], #{}", dst, src, index.offset)
                .expect("writer error"),
            Position::Offset => writeln!(writer, "\tldr {}, [{}, #{}]", dst, src, index.offset)
                .expect("writer error"),
        },
        None => writeln!(writer, "\tldr {}, [{}]", dst, src).expect("writer error"),
    }
}

fn ldp<W: Write>(writer: &mut W, src1: &str, src2: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => writeln!(
                writer,
                "\tldp {}, {}, [{}, #{}]!",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Post => writeln!(
                writer,
                "\tldp {}, {}, [{}], #{}",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Offset => writeln!(
                writer,
                "\tldp {}, {}, [{}, #{}]",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
        },
        None => writeln!(writer, "\tldp {}, {}, [{}]", src1, src2, dst).expect("writer error"),
    }
}

fn mov<W: Write>(writer: &mut W, dst: &str, value: &str) {
    writeln!(writer, "\tmov {}, {}", dst, value).expect("writer error");
}

fn add<W: Write>(writer: &mut W, dst: &str, src1: &str, src2: &str) {
    writeln!(writer, "\tadd {}, {}, {}", dst, src1, src2).expect("writer error");
}

struct Index {
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
}

enum Position {
    Pre,
    Post,
    Offset,
}
