use crate::ast::{declarations::*, expressions::*, statements::*};

pub trait Eval {
    fn evaluate(&self) -> Result<Primary, RuntimeError>;
}

pub trait EvalBinOp {
    fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError>;
}

pub trait EvalUnOp {
    fn evaluate(&self, exp: Primary) -> Result<Primary, RuntimeError>;
}

#[derive(Clone)]
pub struct RuntimeError {
    pub err: String,
}

impl Eval for Statement {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        match self {
            Statement::ExprStmt(expression) => expression.evaluate(),
            Statement::ForStmt(for_stmt) => todo!(),
            Statement::IfStmt(if_stmt) => todo!(),
            Statement::RtrnStmt(rtrn_stmt) => todo!(),
            Statement::WhileStmt(while_stmt) => todo!(),
            Statement::PrntStmt(prnt_stmt) => todo!(),
            Statement::Block(block) => todo!(),
        }
    }
}

impl Eval for Expression {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        // self.0.evaluate()
        todo!()
    }
}

impl Eval for ExprStmt {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}

impl Eval for IfStmt {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}

impl Eval for ForStmt {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}

impl Eval for WhileStmt {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}

impl Eval for PrntStmt {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}

impl Eval for Block {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}

impl EvalBinOp for EqualityOp {
    fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
        match self {
            EqualityOp::NotEquals => Ok(Primary::Boolean(left != right)),
            EqualityOp::EqualEquals => Ok(Primary::Boolean(left == right)),
        }
    }
}

impl Eval for Equality {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.comparision.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.comparision.evaluate()
        }
    }
}

impl EvalBinOp for ComparisionOp {
    fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
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

impl Eval for Comparision {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.term.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.term.evaluate()
        }
    }
}

impl EvalBinOp for TermOp {
    fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
        let err = RuntimeError {
            err: "Operands must be two numbers or two strings.".into(),
        };
        if let Some(left) = left.get_number() {
            let right = right.get_number().ok_or(err)?;

            let val = match self {
                TermOp::Plus => left + right,
                TermOp::Minus => left - right,
            };

            Ok(Primary::Number(val))
        } else if let Some(left) = left.get_string() {
            let right = right.get_string().ok_or(err.clone())?;
            match self {
                TermOp::Plus => Ok(Primary::String(left + &right)),
                _ => Err(err),
            }
        } else {
            Err(err)
        }
    }
}

impl Eval for Term {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.factor.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.factor.evaluate()
        }
    }
}

impl EvalBinOp for FactorOp {
    fn evaluate(&self, left: Primary, right: Primary) -> Result<Primary, RuntimeError> {
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

impl Eval for Factor {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.unary.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.unary.evaluate()
        }
    }
}

impl EvalUnOp for UnaryOp {
    fn evaluate(&self, exp: Primary) -> Result<Primary, RuntimeError> {
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

impl Eval for Unary {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        match self {
            Unary::Un(op, unary) => op.evaluate(unary.evaluate()?),
            Unary::Call(primary) => todo!(),
        }
    }
}

impl Eval for Primary {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
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
        Primary::Identifier(_) => todo!(),
        Primary::This => todo!(),
    }
}
