#[derive(Debug, Clone)]
pub struct Expression(pub Equality);

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
    Pr(Primary),
}

#[derive(Debug, Clone)]
pub enum Primary {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    ParenExpr(Box<Expression>),
}

impl PartialEq for Primary {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Primary::Number(n) => {
                if let Some(m) = other.get_number() {
                    *n == m
                } else {
                    false
                }
            }
            Primary::String(s) => {
                if let Some(t) = other.get_string() {
                    s == &t
                } else {
                    false
                }
            }
            Primary::Boolean(b) => {
                if let Some(c) = other.get_bool() {
                    b == &c
                } else {
                    false
                }
            }
            Primary::Nil => match other {
                Primary::Nil => true,
                _ => false,
            },
            Primary::ParenExpr(_) => false,
        }
    }
}

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

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
    Bang,
}

#[derive(Debug, Clone)]
pub enum EqualityOp {
    NotEquals,
    EqualEquals,
}

#[derive(Debug, Clone)]
pub enum ComparisionOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub enum TermOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum FactorOp {
    Mul,
    Div,
}
