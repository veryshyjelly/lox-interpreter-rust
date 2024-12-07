pub struct Expression(pub Equality);

pub struct Equality {
    pub comparision: Comparision,
    pub rest: Vec<(EqualityOp, Comparision)>,
}

pub struct Comparision {
    pub term: Term,
    pub rest: Vec<(ComparisionOp, Term)>,
}

pub struct Term {
    pub factor: Factor,
    pub rest: Vec<(TermOp, Factor)>,
}

pub struct Factor {
    pub unary: Unary,
    pub rest: Vec<(FactorOp, Unary)>,
}

pub enum Unary {
    Un(UnaryOp, Box<Unary>),
    Pr(Primary),
}

pub enum Primary {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    ParenExpr(Box<Expression>),
}

pub enum UnaryOp {
    Minus,
    Bang,
}

pub enum EqualityOp {
    NotEquals,
    Equals,
}

pub enum ComparisionOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub enum TermOp {
    Plus,
    Minus,
}

pub enum FactorOp {
    Mul,
    Div,
}
