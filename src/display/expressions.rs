use crate::ast::*;
use std::fmt::Display;

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for LogicOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for LogicAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for EqualityOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EqualityOp::NotEquals => write!(f, "!="),
            EqualityOp::EqualEquals => write!(f, "=="),
        }
    }
}

impl Display for Equality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((op, rest)) = &self.rest {
            write!(f, "({} {} {})", op, rest, self.comparision)
        } else {
            write!(f, "{}", self.comparision)
        }
    }
}

impl Display for ComparisionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisionOp::Less => write!(f, "<"),
            ComparisionOp::LessEqual => write!(f, "<="),
            ComparisionOp::Greater => write!(f, ">"),
            ComparisionOp::GreaterEqual => write!(f, ">="),
        }
    }
}

impl Display for Comparision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((op, rest)) = &self.rest {
            write!(f, "({} {} {})", op, rest, self.term)
        } else {
            write!(f, "{}", self.term)
        }
    }
}

impl Display for TermOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermOp::Plus => write!(f, "+"),
            TermOp::Minus => write!(f, "-"),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((op, rest)) = &self.rest {
            write!(f, "({} {} {})", op, rest, self.factor)
        } else {
            write!(f, "{}", self.factor)
        }
    }
}

impl Display for FactorOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorOp::Mul => write!(f, "*"),
            FactorOp::Div => write!(f, "/"),
        }
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((op, rest)) = &self.rest {
            write!(f, "({} {} {})", op, rest, self.unary)
        } else {
            write!(f, "{}", self.unary)
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Bang => write!(f, "!"),
        }
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unary::Un(unary_op, unary) => write!(f, "({} {})", unary_op, unary),
            Unary::Call(call) => call.fmt(f),
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Primary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Primary::Number(n) => format_float(n),
            Primary::String(s) => s.clone(),
            Primary::Boolean(v) => v.to_string(),
            Primary::Nil => "nil".into(),
            Primary::ParenExpr(expression) => format!("(group {expression})"),
            Primary::Identifier(_) => todo!(),
            Primary::This => todo!(),
        };
        write!(f, "{v}")
    }
}

fn format_float(value: &f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.1}", value)
    } else {
        value.to_string()
    }
}
