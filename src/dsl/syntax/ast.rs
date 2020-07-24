#[derive(Debug)]
pub enum Expr<'a> {
    Lambda(Vec<Argument<'a>>, Box<Expr<'a>>),
    Literal(Literal<'a>),
    BinOp(Box<Expr<'a>>, BinOperator, Box<Expr<'a>>),
    UnOp(UnOperator, Box<Expr<'a>>),
    Block(Vec<Statement<'a>>),
}

#[derive(Debug)]
pub enum Statement<'a> {
    Instrution(Expr<'a>),
    Return(Expr<'a>),
    Continue,
    Break,
}

#[derive(Debug)]
pub enum Literal<'a> {
    Num(isize),
    Str(&'a str),
    Bool(bool),
}

#[derive(Debug)]
pub struct Argument<'a> {
    name: &'a str,
}

impl<'a> Argument<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub enum UnOperator {
    Not,
    Pos,
    Neg,
}

#[derive(Debug)]
pub enum BinOperator {
    Mul,
    Div,
    Add,
    Sub,
}
