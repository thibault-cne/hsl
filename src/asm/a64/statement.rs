use std::io::Write;

use super::{A64Compiler, Index};
use crate::parser::{
    ast,
    slt::{ChildIterator, NavigableSlt},
};

impl<W: Write> A64Compiler<W> {
    pub fn statement(
        &mut self,
        ast: &ast::Stmt,
        slt: &NavigableSlt<'_>,
        childs: &mut ChildIterator<'_>,
    ) -> Result<(), String> {
        match ast {
            ast::Stmt::Let { var_name, value } => {
                // TODO: add the value to the stack
                let variable = slt.find_variable(var_name).unwrap();
                self.comment(&format!("Pushing variable {} to the stack", var_name));
                match &variable.value {
                    crate::parser::slt::Value::Str(s) => {
                        self.load_string(var_name, "x8");
                        self.str("x8", "sp", Some(Index::pre(-16)));
                        self.string_literals
                            .push((var_name.to_string(), s.to_string()))
                    }
                    crate::parser::slt::Value::Int(i) => {
                        self.mov("x8", &format!("{:#02x}", i));
                        self.str("x8", "sp", Some(Index::pre(-16)));
                    }
                    crate::parser::slt::Value::NegInt(i) => {
                        self.mov("x8", &format!("-{:#02x}", i));
                        self.str("x8", "sp", Some(Index::pre(-16)));
                    }
                    crate::parser::slt::Value::Bool(bool) => {
                        self.mov("x8", &format!("#{}", *bool as u8));
                        self.str("x8", "sp", Some(Index::pre(-16)));
                    }
                }
                self.expression(value, slt)
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
                        self.load_variable(variable, slt.slt.scope);
                        self.str("x8", "sp", Some(Index::pre(-16)));
                        let format_name = match variable.value {
                            crate::parser::slt::Value::Str(_) => "str_format",
                            crate::parser::slt::Value::Int(_)
                            | crate::parser::slt::Value::NegInt(_) => "int_format",
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
                self.comment("Start operations");
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
                    self.expression(op, slt)?;
                }

                let var_name = match &**var_name {
                    ast::Expr::Ident(ident) => ident,
                    _ => unreachable!(),
                };
                let variable = slt.find_variable(var_name).unwrap();

                self.str("x8", "x29", Some(Index::offset(variable.offset)));
                self.comment("End operations");

                Ok(())
            }
            ast::Stmt::IfStmt {
                condition,
                body,
                else_stmt,
            } => match &**condition {
                ast::Expr::Ident(var_name) => {
                    let variable = slt.find_variable(var_name).unwrap();
                    let then_slt = childs.next().unwrap();
                    let mut then_childs = then_slt.childs();
                    let end_label = format!("if_end_{}", then_slt.slt.region);
                    let else_label = format!("if_else_{}", then_slt.slt.region);

                    // Load variable from a different scope in the x8 reg
                    self.comment(&format!("Load variable {}", var_name));
                    self.load_variable(variable, slt.slt.scope);
                    self.cmp("x8");
                    self.b(&else_label, "ne");

                    self.skip_line();
                    self.comment("Start of then block");
                    self.stp("x29", "lr", "sp", Some(Index::pre(-16)));
                    self.mov("x29", "sp");

                    // Then core
                    for stmt in body {
                        self.statement(stmt, &then_slt, &mut then_childs)?;
                    }

                    self.skip_line();
                    self.comment("Unstack then block");
                    let stack_size = then_slt.slt.variables.len() * 16;
                    self.add("sp", "sp", &format!("{:#02x}", stack_size));
                    self.ldp("x29", "lr", "sp", Some(Index::post(16)));
                    self.b(&end_label, "");

                    self.skip_line();
                    self.label(&else_label);

                    if let Some(else_stmt) = else_stmt {
                        let else_slt = childs.next().unwrap();
                        let mut else_childs = else_slt.childs();

                        self.comment("Start of else block");
                        self.stp("x29", "lr", "sp", Some(Index::pre(-16)));
                        self.mov("x29", "sp");

                        // Then core
                        for stmt in else_stmt {
                            self.statement(stmt, &else_slt, &mut else_childs)?;
                        }

                        self.skip_line();
                        self.comment("Unstack else block");
                        let stack_size = else_slt.slt.variables.len() * 16;
                        self.add("sp", "sp", &format!("{:#02x}", stack_size));
                        self.ldp("x29", "lr", "sp", Some(Index::post(16)));
                    }
                    self.skip_line();
                    self.label(&end_label);

                    Ok(())
                }
                ast::Expr::Literal(_) => todo!(),
                _ => unreachable!(),
            },
        }
    }
}
