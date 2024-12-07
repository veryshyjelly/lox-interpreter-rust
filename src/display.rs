use std::fmt::{Display, Pointer};

use crate::ast::*;

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for Equality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.comparision.fmt(f)?;
        for (op, cmp) in &self.rest {
            op.fmt(f)?;
            cmp.fmt(f)?;
        }
        Ok(())
    }
}

impl Display for ComparisionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Comparision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.term.fmt(f)?;
        for (op, term) in &self.rest {
            op.fmt(f)?;
            term.fmt(f)?;
        }
        Ok(())
    }
}

impl Display for TermOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.factor.fmt(f)?;
        for (op, fact) in &self.rest {
            op.fmt(f)?;
            fact.fmt(f)?;
        }
        Ok(())
    }
}

impl Display for FactorOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.unary.fmt(f)?;
        for (op, un) in &self.rest {
            op.fmt(f)?;
            un.fmt(f)?;
        }
        Ok(())
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
            Unary::Pr(primary) => primary.fmt(f),
        }
    }
}

impl Display for Primary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Primary::Number(n) => format_float(n),
            Primary::String(s) => s.clone(),
            Primary::True => "true".into(),
            Primary::False => "false".into(),
            Primary::Nil => "nil".into(),
            Primary::ParenExpr(expression) => format!("(group {expression})"),
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
