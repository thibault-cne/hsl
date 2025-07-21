use crate::ir::Program;
use crate::parser::slt::{Builder, SymbolLookupTable, Visitor};

impl Visitor for Program {
    fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable) {
        let Self { stmts } = self;

        builder.new_region(slt);
        stmts
            .iter()
            .for_each(|stmt| stmt.visit(builder, slt.last_children_mut().unwrap()));
    }
}
