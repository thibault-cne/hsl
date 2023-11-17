use std::io::{BufRead, Write};

use super::{Compiler, State};
use crate::parser::{ast::Node, slt::NavigableSlt};

pub struct MacOsARM {
    string_literals: Vec<(String, String)>,
}

impl Compiler for MacOsARM {
    fn new() -> Self {
        MacOsARM {
            string_literals: Vec::new(),
        }
    }

    fn evaluate_node<'n, R: BufRead, W: Write, C: Compiler>(
        &mut self,
        ast: &crate::parser::ast::Node,
        state: &mut State<R, W, C>,
        slt: &NavigableSlt<'n>
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
                        write!(state.writer, "\tadr X1, {}\n\tstr X1, [sp, #-16]\n", name)
                            .map_err(|e| e.to_string())?;
                        self.string_literals.push((name.to_string(), s.to_string()))
                    }
                    crate::parser::slt::Value::Integer(i) => {
                        write!(state.writer, "\tmov X1, #{}\n\tstr X1, [sp, #-16]\n", i)
                            .map_err(|e| e.to_string())?
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

                let child = slt.childs().next().unwrap();

                for statement in statements {
                    self.evaluate_node(statement, state, &child)?;
                }

                write!(
                    state.writer,
                    "{}",
                    "\n\tmov     X0, #0\n\tmov     X16, #1\n\tsvc     0\n"
                )
                .map_err(|x| x.to_string())?;

                write!(state.writer, ".data\n",).map_err(|x| x.to_string())?;
                write!(
                    state.writer,
                    "{}",
                    "helloworld:      .ascii  \"Hello World!\\n\"\n"
                )
                .map_err(|x| x.to_string())?;
                for (name, s) in self.string_literals.iter() {
                    write!(state.writer, "{}:      .ascii  \"{}\"\n", name, s)
                        .map_err(|x| x.to_string())?;
                }
                Ok(())
            }
            Node::Print(node) => {
                // Validate it's a value
                match &**node {
                    Node::Float(_)
                    | Node::Boolean(_)
                    | Node::String(_)
                    | Node::Integer(_) => (),
                    Node::Identifier(ident) => {
                        match slt.find_variable(ident).unwrap().value {
                            crate::parser::slt::Value::String(_) => {
                                write!(
                                    state.writer,
                                    "\tadrp X0, ptfStr\n\tadd	X0, X0, ptfStr\n\t",
                                ).map_err(|e| e.to_string())?
                            }
                            _ => (),
                        }
                    }
                    _ => return Err("Return not a value".to_string()),
                };

                // TODO: just call the printf function as everything must be
                // in the stack
                Ok(())
            }
            _ => todo!(),
        }
    }
}
