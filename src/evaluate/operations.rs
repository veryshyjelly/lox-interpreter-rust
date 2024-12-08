use super::*;

impl EvalBinOp for EqualityOp {
    fn evaluate(&self, left: Object, right: Object) -> Result<Object, RuntimeError> {
        match self {
            EqualityOp::NotEquals => Ok(Object::Boolean(left != right)),
            EqualityOp::EqualEquals => Ok(Object::Boolean(left == right)),
        }
    }
}

impl EvalBinOp for ComparisionOp {
    fn evaluate(&self, left: Object, right: Object) -> Result<Object, RuntimeError> {
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
        Ok(Object::Boolean(val))
    }
}

impl EvalBinOp for TermOp {
    fn evaluate(&self, left: Object, right: Object) -> Result<Object, RuntimeError> {
        let err = RuntimeError {
            err: "Operands must be two numbers or two strings.".into(),
        };
        if let Some(left) = left.get_number() {
            let right = right.get_number().ok_or(err)?;

            let val = match self {
                TermOp::Plus => left + right,
                TermOp::Minus => left - right,
            };

            Ok(Object::Number(val))
        } else if let Some(left) = left.get_string() {
            let right = right.get_string().ok_or(err.clone())?;
            match self {
                TermOp::Plus => Ok(Object::String(left + &right)),
                _ => Err(err),
            }
        } else {
            Err(err)
        }
    }
}

impl EvalBinOp for FactorOp {
    fn evaluate(&self, left: Object, right: Object) -> Result<Object, RuntimeError> {
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
        Ok(Object::Number(val))
    }
}

impl EvalUnOp for UnaryOp {
    fn evaluate(&self, exp: Object) -> Result<Object, RuntimeError> {
        match self {
            UnaryOp::Minus => {
                let n = exp.get_number().ok_or(RuntimeError {
                    err: "Operand must be a number".into(),
                })?;
                Ok(Object::Number(-n))
            }
            UnaryOp::Bang => {
                if let Some(b) = exp.get_bool() {
                    Ok(Object::Boolean(!b))
                } else if exp == Object::Nil {
                    Ok(Object::Boolean(true))
                } else {
                    Ok(Object::Boolean(false))
                }
            }
        }
    }
}
