use std::io::{BufRead, Write};

use super::{Compiler, State};
use crate::parser::{ast, slt::NavigableSlt};

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

    fn evaluate_expression<R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &ast::Expr,
        state: &mut State<R, W, C>,
        _slt: &NavigableSlt<'_>,
    ) -> Result<(), String> {
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
                    ast::Op::Add => add(&mut state.writer, "x8", "x8", &format!("{:#02x}", value)),
                    ast::Op::Sub => sub(&mut state.writer, "x8", "x8", &format!("{:#02x}", value)),
                    ast::Op::Mul => {
                        mov(&mut state.writer, "x9", &format!("{:#02x}", value));
                        mul(&mut state.writer, "x8", "x8", "x9");
                    }
                    ast::Op::Div => {
                        mov(&mut state.writer, "x9", &format!("{:#02x}", value));
                        sdiv(&mut state.writer, "x8", "x8", "x9");
                    }
                    ast::Op::Mod => {
                        mov(&mut state.writer, "x9", &format!("{:#02x}", value));
                        udiv(&mut state.writer, "x10", "x8", "x9");
                        msub(&mut state.writer, "x8", "x10", "x9", "x8");
                    }
                }

                Ok(())
            }
        }
    }

    fn evaluate_statement<R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &ast::Stmt,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String> {
        match ast {
            ast::Stmt::Let { var_name, value } => {
                // TODO: add the value to the stack
                let variable = slt.find_variable(var_name).unwrap();
                writeln!(
                    state.writer,
                    "\t// Pushing variable {} to the stack",
                    var_name
                )
                .map_err(|e| e.to_string())?;
                match &variable.value {
                    crate::parser::slt::Value::String(s) => {
                        load_string(&mut state.writer, var_name, "x8");
                        str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                        self.string_literals
                            .push((var_name.to_string(), s.to_string()))
                    }
                    crate::parser::slt::Value::Integer(i) => {
                        mov(&mut state.writer, "x8", &format!("#{}", i));
                        str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                    }
                    crate::parser::slt::Value::Boolean(bool) => {
                        mov(&mut state.writer, "x8", &format!("#{}", *bool as u8));
                        str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                    }
                }
                self.evaluate_expression(value, state, slt)
            }
            ast::Stmt::Print { value } => {
                // Validate it's a value
                match &**value {
                    ast::Expr::Literal(_) => (),
                    ast::Expr::Ident(ident) => {
                        let variable = slt.find_variable(ident).unwrap();
                        let format_name = match variable.value {
                            crate::parser::slt::Value::String(_) => {
                                ldr(
                                    &mut state.writer,
                                    "x29",
                                    "x8",
                                    Some(Index::offset(variable.offset)),
                                );
                                str(&mut state.writer, "x8", "sp", Some(Index::pre(-16)));
                                "str_format"
                            }
                            crate::parser::slt::Value::Integer(_) => {
                                ldr(
                                    &mut state.writer,
                                    "x29",
                                    "x8",
                                    Some(Index::offset(variable.offset)),
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
            ast::Stmt::Assignment {
                var_name,
                initial_value,
                operations,
            } => {
                writeln!(state.writer, "\n\t// Start operations").expect("writer error");
                match &**initial_value {
                    ast::Expr::Literal(lit) => {
                        if let ast::Lit::Int(init) = lit {
                            mov(&mut state.writer, "x8", &format!("{:#02x}", init));
                        }
                    }
                    ast::Expr::Ident(ident) => {
                        // TODO: handle string and float and boolean
                        let variable = slt.find_variable(ident).unwrap();
                        ldr(
                            &mut state.writer,
                            "x29",
                            "x8",
                            Some(Index::offset(variable.offset)),
                        );
                    }
                    _ => unreachable!(),
                }
                for op in operations {
                    self.evaluate_expression(op, state, slt)?;
                }

                let var_name = match &**var_name {
                    ast::Expr::Ident(ident) => ident,
                    _ => unreachable!(),
                };
                let variable = slt.find_variable(var_name).unwrap();

                str(
                    &mut state.writer,
                    "x8",
                    "x29",
                    Some(Index::offset(variable.offset)),
                );
                writeln!(state.writer, "\n\t//End operations\n").expect("writer error");

                Ok(())
            }
        }
    }

    fn evaluate_item<R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &ast::Item,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String> {
        match ast {
            ast::Item::Main { body } => {
                writeln!(state.writer, ".global _start\n.align 2\n_start:")
                    .map_err(|x| x.to_string())?;
                stp(&mut state.writer, "x29", "lr", "sp", Some(Index::pre(-16)));
                mov(&mut state.writer, "x29", "sp");

                let child = slt.childs().next().unwrap();

                for stmt in body {
                    self.evaluate_statement(stmt, state, &child)?;
                }

                let stack_size = child.slt.variables.len() * 16;
                add(
                    &mut state.writer,
                    "sp",
                    "sp",
                    &format!("{:#02x}", stack_size),
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
            Position::Pre => writeln!(
                writer,
                "\tstr {}, [{}, {}]!",
                src,
                dst,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Post => writeln!(
                writer,
                "\tstr {}, [{}], {}",
                src,
                dst,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Offset => writeln!(
                writer,
                "\tstr {}, [{}, {}]",
                src,
                dst,
                index.format_offset()
            )
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
                "\tstp {}, {}, [{}, {}]!",
                src1,
                src2,
                dst,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Post => writeln!(
                writer,
                "\tstp {}, {}, [{}], {}",
                src1,
                src2,
                dst,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Offset => writeln!(
                writer,
                "\tstp {}, {}, [{}, {}]",
                src1,
                src2,
                dst,
                index.format_offset()
            )
            .expect("writer error"),
        },
        None => writeln!(writer, "\tstp {}, {}, [{}]", src1, src2, dst).expect("writer error"),
    }
}

fn ldr<W: Write>(writer: &mut W, src: &str, dst: &str, index: Option<Index>) {
    match index {
        Some(index) => match index.position {
            Position::Pre => writeln!(
                writer,
                "\tldr {}, [{}, {}]!",
                dst,
                src,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Post => writeln!(
                writer,
                "\tldr {}, [{}], {}",
                dst,
                src,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Offset => writeln!(
                writer,
                "\tldr {}, [{}, {}]",
                dst,
                src,
                index.format_offset()
            )
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
                "\tldp {}, {}, [{}, {}]!",
                src1,
                src2,
                dst,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Post => writeln!(
                writer,
                "\tldp {}, {}, [{}], {}",
                src1,
                src2,
                dst,
                index.format_offset()
            )
            .expect("writer error"),
            Position::Offset => writeln!(
                writer,
                "\tldp {}, {}, [{}, {}]",
                src1,
                src2,
                dst,
                index.format_offset()
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

fn sub<W: Write>(writer: &mut W, dst: &str, src1: &str, src2: &str) {
    writeln!(writer, "\tsub {}, {}, {}", dst, src1, src2).expect("writer error");
}

fn mul<W: Write>(writer: &mut W, dst: &str, src1: &str, src2: &str) {
    writeln!(writer, "\tmul {}, {}, {}", dst, src1, src2).expect("writer error");
}

fn sdiv<W: Write>(writer: &mut W, dst: &str, src1: &str, src2: &str) {
    writeln!(writer, "\tsdiv {}, {}, {}", dst, src1, src2).expect("writer error");
}

fn udiv<W: Write>(writer: &mut W, dst: &str, src1: &str, src2: &str) {
    writeln!(writer, "\tudiv {}, {}, {}", dst, src1, src2).expect("writer error");
}

fn msub<W: Write>(writer: &mut W, dst: &str, src1: &str, src2: &str, src3: &str) {
    writeln!(writer, "\tmsub {}, {}, {}, {}", dst, src1, src2, src3).expect("writer error");
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
