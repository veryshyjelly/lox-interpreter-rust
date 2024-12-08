#[derive(Debug, Clone)]
pub struct Expression(pub Assignment);

#[derive(Debug, Clone)]
pub enum Assignment {
    Ass(Call, String, Box<Assignment>),
    LogicOr(LogicOr),
}

#[derive(Debug, Clone)]
pub struct LogicOr {
    pub and: LogicAnd,
    pub rest: Option<Box<LogicOr>>,
}

#[derive(Debug, Clone)]
pub struct LogicAnd {
    pub eq: Equality,
    pub rest: Option<Box<LogicAnd>>,
}

#[derive(Debug, Clone)]
pub struct Equality {
    pub comparision: Comparision,
    pub rest: Option<(EqualityOp, Box<Equality>)>,
}

#[derive(Debug, Clone)]
pub struct Comparision {
    pub term: Term,
    pub rest: Option<(ComparisionOp, Box<Comparision>)>,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub factor: Factor,
    pub rest: Option<(TermOp, Box<Term>)>,
}

#[derive(Debug, Clone)]
pub struct Factor {
    pub unary: Unary,
    pub rest: Option<(FactorOp, Box<Factor>)>,
}

#[derive(Debug, Clone)]
pub enum Unary {
    Un(UnaryOp, Box<Unary>),
    Call(Call),
}

#[derive(Debug, Clone)]
pub struct Call {}

#[derive(Debug, Clone)]
pub enum Calling {
    FuncCall(Option<Arguments>),
    Mthd(String),
}

#[derive(Debug, Clone)]
pub enum Primary {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    ParenExpr(Box<Expression>),
    GlobalId(String),
    This,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Function {}
#[derive(Debug, Clone)]
pub struct Parameters {}
#[derive(Debug, Clone)]
pub struct Arguments {}

impl Primary {
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Primary::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Primary::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Primary::Boolean(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Minus,
    Bang,
}

#[derive(Debug, Copy, Clone)]
pub enum EqualityOp {
    NotEquals,
    EqualEquals,
}

#[derive(Debug, Copy, Clone)]
pub enum ComparisionOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Copy, Clone)]
pub enum TermOp {
    Plus,
    Minus,
}

#[derive(Debug, Copy, Clone)]
pub enum FactorOp {
    Mul,
    Div,
}
