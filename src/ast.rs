use rug::Integer;


pub struct Prog {
    pub decls: Vec<Decl>,
}

pub struct Decl {
    pub name:   String,
    pub typing: Typing,
    pub body:   Expr,
}

pub enum Typing {
    Concrete(String),
    Function(Vec<Typing>, Vec<Typing>),
}

pub enum Expr {
    Literal(Literal),
    Id(String),
    Block(Vec<Expr>),
}

pub enum Literal {
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
