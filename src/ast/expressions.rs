use super::*;

#[derive(Clone)]
pub struct Expression(pub Assignment);

#[derive(Clone)]
pub enum Assignment {
    Assign(Call, Box<Assignment>),
    LogicOr(LogicOr),
}

#[derive(Clone)]
pub struct LogicOr {
    pub and: LogicAnd,
    pub rest: Option<Box<LogicOr>>,
}

#[derive(Clone)]
pub struct LogicAnd {
    pub eq: Equality,
    pub rest: Option<Box<LogicAnd>>,
}

#[derive(Clone)]
pub struct Equality {
    pub comparision: Comparision,
    pub rest: Option<(EqualityOp, Box<Equality>)>,
}

#[derive(Clone)]
pub struct Comparision {
    pub term: Term,
    pub rest: Option<(ComparisionOp, Box<Comparision>)>,
}

#[derive(Clone)]
pub struct Term {
    pub factor: Factor,
    pub rest: Option<(TermOp, Box<Term>)>,
}

#[derive(Clone)]
pub struct Factor {
    pub unary: Unary,
    pub rest: Option<(FactorOp, Box<Factor>)>,
}

#[derive(Clone)]
pub enum Unary {
    Un(UnaryOp, Box<Unary>),
    Call(Call),
}

#[derive(Clone)]
pub struct Call {
    pub prime: Primary,
    pub rest: Vec<Calling>,
}

#[derive(Clone)]
pub enum Calling {
    FuncCall(Option<Arguments>),
    Mthd(String),
}

#[derive(Clone)]
pub enum Primary {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    ParenExpr(Box<Expression>),
    SuperId(String),
    This,
    Nil,
}

#[derive(Clone)]
pub enum UnaryOp {
    Minus,
    Bang,
}

#[derive(Clone)]
pub enum EqualityOp {
    NotEquals,
    EqualEquals,
}

#[derive(Clone)]
pub enum ComparisionOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Clone)]
pub enum TermOp {
    Plus,
    Minus,
}

#[derive(Clone)]
pub enum FactorOp {
    Mul,
    Div,
}
