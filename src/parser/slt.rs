use std::collections::HashMap;

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
        self.variables.insert(
            name.to_string(),
            Variable::new(self.offset, Value::Str(s), self.scope),
        );
        self.offset -= 16;
    }

    pub fn add_negative_integer(&mut self, name: &str, i: u32) {
        self.variables.insert(
            name.to_string(),
            Variable::new(self.offset, Value::NegInt(i), self.scope),
        );
        self.offset -= 16;
    }

    pub fn add_integer(&mut self, name: &str, i: u32) {
        self.variables.insert(
            name.to_string(),
            Variable::new(self.offset, Value::Int(i), self.scope),
        );
        self.offset -= 16;
    }

    pub fn add_boolean(&mut self, name: &str, b: bool) {
        self.variables.insert(
            name.to_string(),
            Variable::new(self.offset, Value::Bool(b), self.scope),
        );
        self.offset -= 16;
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }
}

impl Default for SymbolLookupTable {
    fn default() -> SymbolLookupTable {
        SymbolLookupTable {
            variables: HashMap::new(),
            offset: -16,
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
    Int(u32),
    NegInt(u32),
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
            position: 0,
            parent: self,
            childs: &self.slt.children,
        }
    }

    pub fn find_variable(&self, name: &str) -> Option<&Variable> {
        match self.slt.get_variable(name) {
            Some(var) => Some(var),
            None => self.parent.and_then(|p| p.find_variable(name)),
        }
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
    position: usize,
    parent: &'a NavigableSlt<'a>,
    childs: &'a [SymbolLookupTable],
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = NavigableSlt<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        self.childs.get(self.position - 1).map(|c| NavigableSlt {
            slt: c,
            parent: Some(self.parent),
        })
    }
}
