use std::io::Write;

use crate::parser::slt::Variable;

use super::{A64Compiler, Index, Position};

impl<W: Write> A64Compiler<W> {
    pub fn load_string(&mut self, name: &str, register: &str) {
        writeln!(self.writer, "\tadrp {}, {}@PAGE", register, name).expect("writer error");
        writeln!(
            self.writer,
            "\tadd {}, {}, {}@PAGEOFF",
            register, register, name
        )
        .expect("writer error");
    }

    pub fn load_variable(&mut self, variable: &Variable, slt_scope: u32) {
        let diff = slt_scope - variable.scope;
        self.mov("x9", "x29");
        for _ in 0..diff {
            self.ldr("x9", "x9", None);
        }
        self.ldr("x9", "x8", Some(Index::offset(variable.offset)));
    }

    pub fn str(&mut self, src: &str, dst: &str, index: Option<Index>) {
        match index {
            Some(index) => match index.position {
                Position::Pre => writeln!(
                    self.writer,
                    "\tstr {}, [{}, {}]!",
                    src,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Post => writeln!(
                    self.writer,
                    "\tstr {}, [{}], {}",
                    src,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Offset => writeln!(
                    self.writer,
                    "\tstr {}, [{}, {}]",
                    src,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
            },
            None => writeln!(self.writer, "\tstr {}, [{}]", src, dst).expect("writer error"),
        }
    }

    pub fn stp(&mut self, src1: &str, src2: &str, dst: &str, index: Option<Index>) {
        match index {
            Some(index) => match index.position {
                Position::Pre => writeln!(
                    self.writer,
                    "\tstp {}, {}, [{}, {}]!",
                    src1,
                    src2,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Post => writeln!(
                    self.writer,
                    "\tstp {}, {}, [{}], {}",
                    src1,
                    src2,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Offset => writeln!(
                    self.writer,
                    "\tstp {}, {}, [{}, {}]",
                    src1,
                    src2,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
            },
            None => {
                writeln!(self.writer, "\tstp {}, {}, [{}]", src1, src2, dst).expect("writer error")
            }
        }
    }

    pub fn ldr(&mut self, src: &str, dst: &str, index: Option<Index>) {
        match index {
            Some(index) => match index.position {
                Position::Pre => writeln!(
                    self.writer,
                    "\tldr {}, [{}, {}]!",
                    dst,
                    src,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Post => writeln!(
                    self.writer,
                    "\tldr {}, [{}], {}",
                    dst,
                    src,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Offset => writeln!(
                    self.writer,
                    "\tldr {}, [{}, {}]",
                    dst,
                    src,
                    index.format_offset()
                )
                .expect("writer error"),
            },
            None => writeln!(self.writer, "\tldr {}, [{}]", dst, src).expect("writer error"),
        }
    }

    pub fn ldp(&mut self, src1: &str, src2: &str, dst: &str, index: Option<Index>) {
        match index {
            Some(index) => match index.position {
                Position::Pre => writeln!(
                    self.writer,
                    "\tldp {}, {}, [{}, {}]!",
                    src1,
                    src2,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Post => writeln!(
                    self.writer,
                    "\tldp {}, {}, [{}], {}",
                    src1,
                    src2,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
                Position::Offset => writeln!(
                    self.writer,
                    "\tldp {}, {}, [{}, {}]",
                    src1,
                    src2,
                    dst,
                    index.format_offset()
                )
                .expect("writer error"),
            },
            None => {
                writeln!(self.writer, "\tldp {}, {}, [{}]", src1, src2, dst).expect("writer error")
            }
        }
    }

    pub fn mov(&mut self, dst: &str, value: &str) {
        writeln!(self.writer, "\tmov {}, {}", dst, value).expect("writer error");
    }

    pub fn add(&mut self, dst: &str, src1: &str, src2: &str) {
        writeln!(self.writer, "\tadd {}, {}, {}", dst, src1, src2).expect("writer error");
    }

    pub fn sub(&mut self, dst: &str, src1: &str, src2: &str) {
        writeln!(self.writer, "\tsub {}, {}, {}", dst, src1, src2).expect("writer error");
    }

    pub fn mul(&mut self, dst: &str, src1: &str, src2: &str) {
        writeln!(self.writer, "\tmul {}, {}, {}", dst, src1, src2).expect("writer error");
    }

    pub fn sdiv(&mut self, dst: &str, src1: &str, src2: &str) {
        writeln!(self.writer, "\tsdiv {}, {}, {}", dst, src1, src2).expect("writer error");
    }

    pub fn udiv(&mut self, dst: &str, src1: &str, src2: &str) {
        writeln!(self.writer, "\tudiv {}, {}, {}", dst, src1, src2).expect("writer error");
    }

    pub fn msub(&mut self, dst: &str, src1: &str, src2: &str, src3: &str) {
        writeln!(self.writer, "\tmsub {}, {}, {}, {}", dst, src1, src2, src3)
            .expect("writer error");
    }

    pub fn comment(&mut self, comment: &str) {
        writeln!(self.writer, "\t// {}", comment)
            .unwrap_or_else(|_| panic!("writer error in comment({})", comment));
    }

    pub fn skip_line(&mut self) {
        writeln!(self.writer).expect("writer error in skip_line()");
    }

    pub fn b(&mut self, label: &str, flag: &str) {
        writeln!(self.writer, "\tb{} _{}", flag, label)
            .unwrap_or_else(|_| panic!("writer error in b(label: {}, flag: {})", label, flag));
    }

    pub fn label(&mut self, label: &str) {
        writeln!(self.writer, "_{}:", label)
            .unwrap_or_else(|_| panic!("writer error in label({})", label));
    }

    pub fn cmp(&mut self, src: &str) {
        writeln!(self.writer, "\tcmp {}, 0x0", src)
            .unwrap_or_else(|_| panic!("writer error in cmp({})", src));
    }
}
