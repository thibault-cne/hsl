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

    fn evaluate_node<'n, R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &crate::parser::ast::Node,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'n>,
    ) -> Result<(), String> {
        match ast {
            Node::DeclareLiteral(name, value) => {
                // TODO: add the value to the stack
                let variable = slt.find_variable(name).unwrap();
                write!(
                    state.writer,
                    "\t// Pushing variable {} to the stack\n",
                    name
                )
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
                write!(state.writer, "{}", ".global _start\n.align 2\n_start:\n")
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

                write!(
                    state.writer,
                    "{}",
                    "\n\tmov     x0, #0\n\tmov     x16, #1\n\tsvc     0\n"
                )
                .map_err(|x| x.to_string())?;

                write!(state.writer, ".data\n",).map_err(|x| x.to_string())?;
                for (name, s) in self.string_literals.iter() {
                    write!(state.writer, "\t{}:      .asciz  \"{}\"\n", name, s)
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
                write!(state.writer, "\tbl _printf\n").expect("writer error");
                add(&mut state.writer, "sp", "sp", "0x10");
                Ok(())
            }
            _ => todo!(),
        }
    }
}

fn load_string<W: Write>(writer: &mut W, name: &str, register: &str) {
    write!(writer, "\tadrp {}, {}@PAGE\n", register, name).expect("writer error");
    write!(
        writer,
        "\tadd {}, {}, {}@PAGEOFF\n",
        register, register, name
    )
    .expect("writer error");
}

fn str<W: Write>(writer: &mut W, src: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => write!(writer, "\tstr {}, [{}, #{}]!\n", src, dst, index.offset)
                .expect("writer error"),
            Position::Post => write!(writer, "\tstr {}, [{}], #{}\n", src, dst, index.offset)
                .expect("writer error"),
            Position::Offset => write!(writer, "\tstr {}, [{}, #{}]\n", src, dst, index.offset)
                .expect("writer error"),
        },
        None => write!(writer, "\tstr {}, [{}]\n", src, dst).expect("writer error"),
    }
}

fn stp<W: Write>(writer: &mut W, src1: &str, src2: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => write!(
                writer,
                "\tstp {}, {}, [{}, #{}]!\n",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Post => write!(
                writer,
                "\tstp {}, {}, [{}], #{}\n",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Offset => write!(
                writer,
                "\tstp {}, {}, [{}, #{}]\n",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
        },
        None => write!(writer, "\tstp {}, {}, [{}]\n", src1, src2, dst).expect("writer error"),
    }
}

fn ldr<W: Write>(writer: &mut W, src: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => write!(writer, "\tldr {}, [{}, #{}]!\n", dst, src, index.offset)
                .expect("writer error"),
            Position::Post => write!(writer, "\tldr {}, [{}], #{}\n", dst, src, index.offset)
                .expect("writer error"),
            Position::Offset => write!(writer, "\tldr {}, [{}, #{}]\n", dst, src, index.offset)
                .expect("writer error"),
        },
        None => write!(writer, "\tldr {}, [{}]\n", dst, src).expect("writer error"),
    }
}

fn ldp<W: Write>(writer: &mut W, src1: &str, src2: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => write!(
                writer,
                "\tldp {}, {}, [{}, #{}]!\n",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Post => write!(
                writer,
                "\tldp {}, {}, [{}], #{}\n",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
            Position::Offset => write!(
                writer,
                "\tldp {}, {}, [{}, #{}]\n",
                src1, src2, dst, index.offset
            )
            .expect("writer error"),
        },
        None => write!(writer, "\tldp {}, {}, [{}]\n", src1, src2, dst).expect("writer error"),
    }
}

fn mov<W: Write>(writer: &mut W, dst: &str, value: &str) {
    write!(writer, "\tmov {}, {}\n", dst, value).expect("writer error");
}

fn add<W: Write>(writer: &mut W, dst: &str, src1: &str, src2: &str) {
    write!(writer, "\tadd {}, {}, {}\n", dst, src1, src2).expect("writer error");
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
