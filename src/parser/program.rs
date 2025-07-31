use crate::ir::{Fn, Program};
use crate::parser::slt::{Builder, SymbolLookupTable, Visitor};

impl<'prog> Visitor for Program<'prog> {
    fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable) {
        let Self { func, .. } = self;

        builder.new_region(slt);
        func.iter()
            .for_each(|f| f.visit(builder, slt.last_children_mut().unwrap()));
    }
}

impl<'prog> Visitor for Fn<'prog> {
    fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable) {
        // TODO: add function declaration to the slt
        let Self { stmts, .. } = self;

        builder.new_region(slt);
        stmts
            .iter()
            .for_each(|stmt| stmt.visit(builder, slt.last_children_mut().unwrap()));
    }
}
