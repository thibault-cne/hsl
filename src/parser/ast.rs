use super::slt::{Builder, SymbolLookupTable};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    DeclareLiteral(String, Box<Node>),
    Print(Box<Node>),
    Main(Vec<Node>),
    Float(f32),
    Integer(u32),
    String(String),
    Boolean(bool),
    Identifier(String),
    Noop,
}

impl Node {
    pub fn visit(&self, builder: &mut Builder, slt: &mut SymbolLookupTable) {
        match self {
            Node::DeclareLiteral(name, node) => match &**node {
                Node::Float(f) => slt.add_float(name, *f),
                Node::String(s) => slt.add_string(name, s.to_string()),
                Node::Integer(i) => slt.add_integer(name, *i),
                Node::Boolean(b) => slt.add_boolean(name, *b),
                _ => (),
            },
            Node::Main(nodes) => {
                builder.new_region(slt);
                for node in nodes {
                    node.visit(builder, slt.last_children_mut().unwrap());
                }
            }
            _ => (),
        }
    }
}
