use std::collections::HashMap;

pub trait Visitor<'prog> {
    fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable<'prog>);
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SymbolLookupTable<'prog> {
    pub variables: HashMap<&'prog str, Variable<'prog>>,
    pub offset: i32,
    pub region: u32,
    pub scope: u32,

    pub children: Vec<SymbolLookupTable<'prog>>,
}

impl<'prog> SymbolLookupTable<'prog> {
    pub fn last_children_mut(&mut self) -> Option<&mut SymbolLookupTable<'prog>> {
        self.children.last_mut()
    }

    pub fn add_string(&mut self, name: &'prog str, s: &'prog str) {
        self.offset += 1;
        self.variables
            .insert(name, Variable::new(self.offset, Value::Str(s), self.scope));
    }

    pub fn add_integer(&mut self, name: &'prog str, i: i64) {
        self.offset += 1;
        self.variables
            .insert(name, Variable::new(self.offset, Value::Int(i), self.scope));
    }

    pub fn add_boolean(&mut self, name: &'prog str, b: bool) {
        self.offset += 1;
        self.variables
            .insert(name, Variable::new(self.offset, Value::Bool(b), self.scope));
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable<'prog> {
    pub offset: i32,
    pub value: Value<'prog>,
    pub scope: u32,
}

impl<'prog> Variable<'prog> {
    fn new(offset: i32, value: Value, scope: u32) -> Variable {
        Variable {
            offset,
            value,
            scope,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
