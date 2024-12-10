use super::*;

pub struct Expression(pub Assignment);

pub enum Assignment {
    Assign(Call, Box<Assignment>),
    LogicOr(LogicOr),
}

pub struct LogicOr {
    pub and: LogicAnd,
    pub rest: Option<Box<LogicOr>>,
}

pub struct LogicAnd {
    pub eq: Equality,
    pub rest: Option<Box<LogicAnd>>,
}

pub struct Equality {
    pub comparision: Comparision,
    pub rest: Option<(EqualityOp, Box<Equality>)>,
}

pub struct Comparision {
    pub term: Term,
    pub rest: Option<(ComparisionOp, Box<Comparision>)>,
}

pub struct Term {
    pub factor: Factor,
    pub rest: Option<(TermOp, Box<Term>)>,
}

pub struct Factor {
    pub unary: Unary,
    pub rest: Option<(FactorOp, Box<Factor>)>,
}

pub enum Unary {
    Un(UnaryOp, Box<Unary>),
    Call(Call),
}

pub struct Call {
    pub prime: Primary,
    pub rest: Vec<Calling>,
}

pub enum Calling {
    FuncCall(Option<Arguments>),
    Mthd(String),
}

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

pub enum UnaryOp {
    Minus,
    Bang,
}

pub enum EqualityOp {
    NotEquals,
    EqualEquals,
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
