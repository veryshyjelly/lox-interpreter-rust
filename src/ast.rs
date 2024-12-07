#[derive(Debug)]
pub struct Expression(pub Equality);

#[derive(Debug)]
pub struct Equality {
    pub comparision: Comparision,
    pub rest: Vec<(EqualityOp, Comparision)>,
}

#[derive(Debug)]
pub struct Comparision {
    pub term: Term,
    pub rest: Vec<(ComparisionOp, Term)>,
}

#[derive(Debug)]
pub struct Term {
    pub factor: Factor,
    pub rest: Vec<(TermOp, Factor)>,
}

#[derive(Debug)]
pub struct Factor {
    pub unary: Unary,
    pub rest: Vec<(FactorOp, Unary)>,
}

#[derive(Debug)]
pub enum Unary {
    Un(UnaryOp, Box<Unary>),
    Pr(Primary),
}

#[derive(Debug)]
pub enum Primary {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    ParenExpr(Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus,
    Bang,
}

#[derive(Debug)]
pub enum EqualityOp {
    NotEquals,
    Equals,
}

#[derive(Debug)]
pub enum ComparisionOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug)]
pub enum TermOp {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum FactorOp {
    Mul,
    Div,
}
