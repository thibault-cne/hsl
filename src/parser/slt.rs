use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolLookupTable {
    variables: HashMap<String, Variable>,
    offset: i32,
    region: usize,
    scope: usize,

    children: Vec<SymbolLookupTable>,
}

impl SymbolLookupTable {
    pub fn last_children_mut(&mut self) -> Option<&mut SymbolLookupTable> {
        self.children.last_mut()
    }

    pub fn add_float(&mut self, name: &str, f: f32) {
        self.variables
            .insert(name.to_string(), Variable::float(f, self.offset));
        self.offset += 16;
    }

    pub fn add_string(&mut self, name: &str, s: String) {
        self.variables
            .insert(name.to_string(), Variable::string(s, self.offset));
        self.offset += 16;
    }

    pub fn add_integer(&mut self, name: &str, i: u32) {
        self.variables
            .insert(name.to_string(), Variable::integer(i, self.offset));
        self.offset += 16;
    }

    pub fn add_boolean(&mut self, name: &str, b: bool) {
        self.variables
            .insert(name.to_string(), Variable::boolean(b, self.offset));
        self.offset += 16;
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
    offset: i32,
    pub value: Value,
}

impl Variable {
    fn integer(i: u32, offset: i32) -> Variable {
        Variable {
            offset,
            value: Value::Integer(i),
        }
    }

    fn boolean(b: bool, offset: i32) -> Variable {
        Variable {
            offset,
            value: Value::Boolean(b),
        }
    }

    fn float(f: f32, offset: i32) -> Variable {
        Variable {
            offset,
            value: Value::Float(f),
        }
    }

    fn string(s: String, offset: i32) -> Variable {
        Variable {
            offset,
            value: Value::String(s),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Float(f32),
    Integer(u32),
    Boolean(bool),
}

pub struct Builder {
    region_count: usize,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { region_count: 0 }
    }

    pub fn region(&mut self) -> SymbolLookupTable {
        let mut new = SymbolLookupTable::default();
        new.region = self.region_count;
        self.region_count += 1;
        new
    }

    pub fn new_region(&mut self, parent: &mut SymbolLookupTable) {
        let mut new = SymbolLookupTable::default();

        new.region = self.region_count;
        new.scope = parent.scope + 1;

        parent.children.push(new);

        self.region_count += 1;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NavigableSlt<'a> {
    slt: &'a SymbolLookupTable,
    parent: Option<&'a NavigableSlt<'a>>,
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
            None => self.parent.map(|p| p.find_variable(name)).flatten(),
        }
    }
}

impl<'a> Into<NavigableSlt<'a>> for &'a SymbolLookupTable {
    fn into(self) -> NavigableSlt<'a> {
        NavigableSlt {
            slt: self,
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
