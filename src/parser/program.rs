use crate::ir::{Fn, Program};
use crate::parser::slt::{Builder, SymbolLookupTable, Visitor};

impl<'prog> Visitor<'prog> for Program<'prog> {
    fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable<'prog>) {
        let Self { func, .. } = self;

        builder.new_region(slt);
        func.iter()
            .for_each(|f| f.visit(builder, slt.last_children_mut().unwrap()));
    }
}

impl<'prog> Visitor<'prog> for Fn<'prog> {
    fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable<'prog>) {
        // TODO: add function declaration to the slt
        let Self { stmts, .. } = self;

        builder.new_region(slt);
        let child_mut = slt.last_children_mut().unwrap();

        for stmt in stmts {
            stmt.visit(builder, child_mut);
        }
    }
}
