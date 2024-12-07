use crate::ast::*;

pub struct RuntimeError {
    pub err: String,
}

impl Expression {
    pub fn evaluate(&self) -> Result<Primary, RuntimeError> {
        self.0.evaluate()
    }
}

impl EqualityOp {
    pub fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
        match self {
            EqualityOp::NotEquals => Ok(Primary::Boolean(left != right)),
            EqualityOp::EqualEquals => Ok(Primary::Boolean(left == right)),
        }
    }
}

impl Equality {
    pub fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let left = self.comparision.evaluate()?;
            let right = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.comparision.evaluate()
        }
    }
}

impl ComparisionOp {
    pub fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
        let left = left.get_number().ok_or(RuntimeError {
            err: "Operands must be numbers.".into(),
        })?;
        let right = right.get_number().ok_or(RuntimeError {
            err: "Operands must be numbers.".into(),
        })?;

        let val = match self {
            ComparisionOp::Less => left < right,
            ComparisionOp::LessEqual => left <= right,
            ComparisionOp::Greater => left > right,
            ComparisionOp::GreaterEqual => left >= right,
        };
        Ok(Primary::Boolean(val))
    }
}

impl Comparision {
    pub fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let left = self.term.evaluate()?;
            let right = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.term.evaluate()
        }
    }
}

impl TermOp {
    pub fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
        let left = left.get_number().ok_or(RuntimeError {
            err: "Operands must be numbers.".into(),
        })?;
        let right = right.get_number().ok_or(RuntimeError {
            err: "Operands must be numbers.".into(),
        })?;

        let val = match self {
            TermOp::Plus => left + right,
            TermOp::Minus => left - right,
        };

        Ok(Primary::Number(val))
    }
}

impl Term {
    pub fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let left = self.factor.evaluate()?;
            let right = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.factor.evaluate()
        }
    }
}

impl FactorOp {
    pub fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
        let left = left.get_number().ok_or(RuntimeError {
            err: "Operands must be numbers.".into(),
        })?;
        let right = right.get_number().ok_or(RuntimeError {
            err: "Operands must be numbers.".into(),
        })?;

        let val = match self {
            FactorOp::Mul => left * right,
            FactorOp::Div => left / right,
        };
        Ok(Primary::Number(val))
    }
}

impl Factor {
    pub fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let left = self.unary.evaluate()?;
            let right = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.unary.evaluate()
        }
    }
}

impl UnaryOp {
    pub fn evaluate(&self, exp: Primary) -> Result<Primary, RuntimeError> {
        match self {
            UnaryOp::Minus => {
                let n = exp.get_number().ok_or(RuntimeError {
                    err: "Operand must be a number".into(),
                })?;
                Ok(Primary::Number(-n))
            }
            UnaryOp::Bang => {
                if let Some(b) = exp.get_bool() {
                    Ok(Primary::Boolean(!b))
                } else if exp == Primary::Nil {
                    Ok(Primary::Boolean(true))
                } else {
                    Ok(Primary::Boolean(false))
                }
            }
        }
    }
}

impl Unary {
    pub fn evaluate(&self) -> Result<Primary, RuntimeError> {
        match self {
            Unary::Un(op, unary) => op.evaluate(unary.evaluate()?),
            Unary::Pr(primary) => primary.evaluate(),
        }
    }
}

impl Primary {
    pub fn evaluate(&self) -> Result<Primary, RuntimeError> {
        match self {
            Primary::ParenExpr(expression) => expression.evaluate(),
            pr => Ok(pr.clone()),
        }
    }
}

pub fn debug_primary(p: Primary) -> String {
    match p {
        Primary::Number(n) => n.to_string(),
        Primary::String(s) => s.clone(),
        Primary::Boolean(v) => v.to_string(),
        Primary::Nil => "nil".into(),
        Primary::ParenExpr(expression) => format!("(group {expression})"),
    }
}
