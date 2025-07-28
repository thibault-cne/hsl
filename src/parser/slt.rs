use std::collections::HashMap;

pub trait Visitor {
    fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable);
}

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolLookupTable {
    pub variables: HashMap<String, Variable>,
    pub offset: i32,
    pub region: u32,
    pub scope: u32,

    pub children: Vec<SymbolLookupTable>,
}

impl SymbolLookupTable {
    pub fn last_children_mut(&mut self) -> Option<&mut SymbolLookupTable> {
        self.children.last_mut()
    }

    pub fn add_string(&mut self, name: &str, s: String) {
        self.offset += 1;
        self.variables.insert(
            name.to_string(),
            Variable::new(self.offset, Value::Str(s), self.scope),
        );
    }

    pub fn add_integer(&mut self, name: &str, i: i64) {
        self.offset += 1;
        self.variables.insert(
            name.to_string(),
            Variable::new(self.offset, Value::Int(i), self.scope),
        );
    }

    pub fn add_boolean(&mut self, name: &str, b: bool) {
        self.offset += 1;
        self.variables.insert(
            name.to_string(),
            Variable::new(self.offset, Value::Bool(b), self.scope),
        );
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }
}

impl Default for SymbolLookupTable {
    fn default() -> SymbolLookupTable {
        SymbolLookupTable {
            variables: HashMap::new(),
            offset: 0,
            region: 0,
            scope: 0,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub offset: i32,
    pub value: Value,
    pub scope: u32,
}

impl Variable {
    fn new(offset: i32, value: Value, scope: u32) -> Variable {
        Variable {
            offset,
            value,
            scope,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Bool(bool),
}

pub struct Builder {
    region_count: u32,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { region_count: 0 }
    }

    pub fn region(&mut self) -> SymbolLookupTable {
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
pub struct NavigableSlt<'a> {
    pub slt: &'a SymbolLookupTable,
    pub parent: Option<&'a NavigableSlt<'a>>,
}

impl<'a> NavigableSlt<'a> {
    pub fn childs(&'a self) -> ChildIterator<'a> {
        ChildIterator {
            parent: self,
            childs: self.slt.children.iter(),
        }
    }

    pub fn find_variable(&self, name: &str) -> Option<&Variable> {
        match self.slt.get_variable(name) {
            Some(var) => Some(var),
            None => self.parent.and_then(|p| p.find_variable(name)),
        }
    }
}

impl<'a> core::ops::Deref for NavigableSlt<'a> {
    type Target = SymbolLookupTable;

    fn deref(&self) -> &Self::Target {
        self.slt
    }
}

impl<'a> From<&'a SymbolLookupTable> for NavigableSlt<'a> {
    fn from(value: &'a SymbolLookupTable) -> Self {
        NavigableSlt {
            slt: value,
            parent: None,
        }
    }
}

pub struct ChildIterator<'a> {
    parent: &'a NavigableSlt<'a>,
    childs: std::slice::Iter<'a, SymbolLookupTable>,
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = NavigableSlt<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.childs.next().map(|c| NavigableSlt {
            slt: c,
            parent: Some(self.parent),
        })
    }
}
