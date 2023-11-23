use std::io::Write;

use super::Compiler;
use crate::parser::{ast, slt::NavigableSlt};

mod asm;

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

    fn evaluate_statement(
        &mut self,
        ast: &ast::Stmt,
        slt: &NavigableSlt<'_>,
    ) -> Result<(), String> {
        match ast {
            ast::Stmt::Let { var_name, value } => {
                // TODO: add the value to the stack
                let variable = slt.find_variable(var_name).unwrap();
                writeln!(
                    self.writer,
                    "\t// Pushing variable {} to the stack",
                    var_name
                )
                .map_err(|e| e.to_string())?;
                match &variable.value {
                    crate::parser::slt::Value::String(s) => {
                        self.load_string(var_name, "x8");
                        self.str("x8", "sp", Some(Index::pre(-16)));
                        self.string_literals
                            .push((var_name.to_string(), s.to_string()))
                    }
                    crate::parser::slt::Value::Integer(i) => {
                        self.mov("x8", &format!("#{}", i));
                        self.str("x8", "sp", Some(Index::pre(-16)));
                    }
                    crate::parser::slt::Value::Boolean(bool) => {
                        self.mov("x8", &format!("#{}", *bool as u8));
                        self.str("x8", "sp", Some(Index::pre(-16)));
                    }
                }
                self.evaluate_expression(value, slt)
            }
            ast::Stmt::Print { value } => {
                // Validate it's a value
                match &**value {
                    ast::Expr::Literal(lit) => match lit {
                        ast::Lit::Str(str) => {
                            let lit_name = format!("_lit_{}", self.string_literals.len());
                            self.string_literals
                                .push((lit_name.clone(), str.to_string()));
                            self.load_string(&lit_name, "x8");
                            self.load_string("str_format", "x0");
                            self.str("x8", "sp", Some(Index::pre(-16)));
                        }
                        ast::Lit::Int(int) => {
                            self.mov("x8", &format!("{:#02x}", int));
                            self.str("x8", "sp", Some(Index::pre(-16)));
                            self.load_string("int_format", "x0");
                        }
                        _ => unreachable!(),
                    },
                    ast::Expr::Ident(ident) => {
                        let variable = slt.find_variable(ident).unwrap();
                        let format_name = match variable.value {
                            crate::parser::slt::Value::String(_) => {
                                self.ldr("x29", "x8", Some(Index::offset(variable.offset)));
                                self.str("x8", "sp", Some(Index::pre(-16)));
                                "str_format"
                            }
                            crate::parser::slt::Value::Integer(_) => {
                                self.ldr("x29", "x8", Some(Index::offset(variable.offset)));
                                self.str("x8", "sp", Some(Index::pre(-16)));
                                "int_format"
                            }
                            _ => "any",
                        };
                        self.load_string(format_name, "x0");
                    }
                    _ => return Err("Return not a value".to_string()),
                };

                writeln!(self.writer, "\tbl _printf").expect("writer error");
                self.add("sp", "sp", "0x10");
                Ok(())
            }
            ast::Stmt::Assignment {
                var_name,
                initial_value,
                operations,
            } => {
                writeln!(self.writer, "\n\t// Start operations").expect("writer error");
                match &**initial_value {
                    ast::Expr::Literal(lit) => {
                        if let ast::Lit::Int(init) = lit {
                            self.mov("x8", &format!("{:#02x}", init));
                        }
                    }
                    ast::Expr::Ident(ident) => {
                        // TODO: handle string and float and boolean
                        let variable = slt.find_variable(ident).unwrap();
                        self.ldr("x29", "x8", Some(Index::offset(variable.offset)));
                    }
                    _ => unreachable!(),
                }
                for op in operations {
                    self.evaluate_expression(op, slt)?;
                }

                let var_name = match &**var_name {
                    ast::Expr::Ident(ident) => ident,
                    _ => unreachable!(),
                };
                let variable = slt.find_variable(var_name).unwrap();

                self.str("x8", "x29", Some(Index::offset(variable.offset)));
                writeln!(self.writer, "\n\t//End operations\n").expect("writer error");

                Ok(())
            }
        }
    }

    fn evaluate_item(&mut self, ast: &ast::Item, slt: &NavigableSlt<'_>) -> Result<(), String> {
        match ast {
            ast::Item::Main { body } => {
                writeln!(self.writer, ".global _start\n.align 2\n_start:")
                    .map_err(|x| x.to_string())?;
                self.stp("x29", "lr", "sp", Some(Index::pre(-16)));
                self.mov("x29", "sp");

                let child = slt.childs().next().unwrap();

                for stmt in body {
                    self.evaluate_statement(stmt, &child)?;
                }

                let stack_size = child.slt.variables.len() * 16;
                self.add("sp", "sp", &format!("{:#02x}", stack_size));

                self.ldp("x29", "lr", "sp", Some(Index::post(16)));

                writeln!(
                    self.writer,
                    "\n\tmov     x0, #0\n\tmov     x16, #1\n\tsvc     0"
                )
                .map_err(|x| x.to_string())?;

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
