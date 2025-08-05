#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Default)]
pub struct SymbolLookupTable<'prog> {
    pub variables: HashMap<&'prog str, (Variable<'prog>, crate::lexer::token::Span)>,
    pub funcs: HashMap<&'prog str, (Fn<'prog>, crate::lexer::token::Span)>,

    pub offset: i32,
    pub region: u32,
    pub scope: u32,

    pub children: Vec<SymbolLookupTable<'prog>>,
}

impl<'prog> SymbolLookupTable<'prog> {
    pub fn last_children_mut(&mut self) -> Option<&mut SymbolLookupTable<'prog>> {
        self.children.last_mut()
    }

    /// Add a new variable to the slt. It returns an `Option<()>`, if the result is not
    /// `Option::None` then a variable with the same name has already been pushed to the slt
    pub fn add_variable<T: Into<Variable<'prog>>>(
        &mut self,
        var: T,
        span: crate::lexer::token::Span,
    ) -> Option<(Variable<'prog>, crate::lexer::token::Span)> {
        self.offset += 1;
        let mut var = var.into();
        var.offset = self.offset;
        var.scope = self.scope;
        self.variables.insert(var.id, (var, span))
    }

    pub fn add_function<T: Into<Fn<'prog>>>(
        &mut self,
        func: T,
        span: crate::lexer::token::Span,
    ) -> Option<(Fn<'prog>, crate::lexer::token::Span)> {
        let func = func.into();
        self.funcs.insert(func.id, (func, span))
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name).map(|(var, _)| var)
    }
}

pub enum Type {
    Ptr(InnerType),
    Val(InnerType),
}

pub enum InnerType {
    Str,
    Int,
    Bool,
    Void,
}

pub struct Variable<'prog> {
    pub id: &'prog str,
    pub ty: Type,
    pub value: Value<'prog>,
    pub offset: i32,
    pub scope: u32,
}

pub struct Fn<'prog> {
    pub id: &'prog str,
    pub ty: Type,
}

pub enum Value<'prog> {
    Str(&'prog str),
    Int(i64),
    Bool(bool),
}

pub struct Builder<'prog> {
    region_count: u32,
    _marker: core::marker::PhantomData<SymbolLookupTable<'prog>>,
}

impl<'prog> Builder<'prog> {
    pub fn new() -> Builder<'prog> {
        Builder {
            region_count: 0,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn region(&mut self) -> SymbolLookupTable<'prog> {
        let new = SymbolLookupTable {
            region: self.region_count,
            ..Default::default()
        };
        self.region_count += 1;
        new
    }

    pub fn new_region(&mut self, parent: &mut SymbolLookupTable) {
        let new = SymbolLookupTable {
            region: self.region_count,
            scope: parent.scope + 1,
            ..Default::default()
        };

        parent.children.push(new);

        self.region_count += 1;
    }
}

pub struct NavigableSlt<'prog> {
    pub slt: &'prog SymbolLookupTable<'prog>,
    pub parent: Option<&'prog NavigableSlt<'prog>>,
}

impl<'prog> NavigableSlt<'prog> {
    pub fn childs(&'prog self) -> ChildIterator<'prog> {
        ChildIterator {
            parent: self,
            childs: self.slt.children.iter(),
        }
    }

    pub fn find_variable(&self, name: &str) -> Option<&Variable<'prog>> {
        match self.slt.get_variable(name) {
            Some(var) => Some(var),
            None => self.parent.and_then(|p| p.find_variable(name)),
        }
    }
}

impl<'prog> core::ops::Deref for NavigableSlt<'prog> {
    type Target = SymbolLookupTable<'prog>;

    fn deref(&self) -> &Self::Target {
        self.slt
    }
}

impl<'prog> From<&'prog SymbolLookupTable<'prog>> for NavigableSlt<'prog> {
    fn from(value: &'prog SymbolLookupTable<'prog>) -> Self {
        NavigableSlt {
            slt: value,
            parent: None,
        }
    }
}

pub struct ChildIterator<'prog> {
    parent: &'prog NavigableSlt<'prog>,
    childs: std::slice::Iter<'prog, SymbolLookupTable<'prog>>,
}

impl<'prog> Iterator for ChildIterator<'prog> {
    type Item = NavigableSlt<'prog>;

    fn next(&mut self) -> Option<Self::Item> {
        self.childs.next().map(|c| NavigableSlt {
            slt: c,
            parent: Some(self.parent),
        })
    }
}

macro_rules! impl_variable_from {
    (@inner) => {};
    (@inner $inner_ty:tt < $from_ty:ty > ; $($tt:tt)*) => {
        impl<'prog> From<(&'prog str, Type, $from_ty)> for Variable<'prog> {
            fn from(value: (&'prog str, Type, $from_ty)) -> Self {
                Variable {
                    id: value.0,
                    ty: value.1,
                    offset: 0,
                    value: Value::$inner_ty(value.2),
                    scope: 0,
                }
            }
        }

        impl_variable_from! { @inner $($tt)* }
    };
    ($($tt:tt)*) => {
        impl_variable_from! { @inner $($tt)* }
    }
}

impl_variable_from! {
    Str<&'prog str>;
    Bool<bool>;
    Int<i64>;
}
