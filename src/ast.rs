use rug::Integer;


#[derive(Debug, Clone, PartialEq)]
pub struct Prog {
    pub decls: Vec<Decl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decl {
    pub name:   String,
    pub typing: Typing,
    pub body:   Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Typing {
    Concrete(String),
    Function(Vec<Typing>, Vec<Typing>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Id(String),
    Block(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    Int(Integer),
    Float(f64),
    Char(char),
    Str(String),
    List(Vec<Expr>),
}


impl Prog {
    pub fn new(decls: Vec<Decl>) -> Self {
        Self { decls }
    }
}

impl Decl {
    pub fn new(name: String, typing: Typing, body: Expr) -> Self {
        Self { name, typing, body }
    }
}
