use crate::ast::{declarations::*, expressions::*, statements::*};
pub use object::*;

pub mod environment;
pub mod object;
pub mod operations;

pub trait Eval {
    fn evaluate(&self) -> Result<Object, RuntimeError>;
}

pub trait EvalBinOp {
    fn evaluate(&self, left: Object, right: Object) -> Result<Object, RuntimeError>;
}

pub trait EvalUnOp {
    fn evaluate(&self, exp: Object) -> Result<Object, RuntimeError>;
}

#[derive(Clone)]
pub struct RuntimeError {
    pub err: String,
}

impl Eval for Statement {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        match self {
            Statement::ExprStmt(expression) => expression.evaluate(),
            Statement::ForStmt(for_stmt) => for_stmt.evaluate(),
            Statement::IfStmt(if_stmt) => if_stmt.evaluate(),
            Statement::RtrnStmt(rtrn_stmt) => rtrn_stmt.evaluate(),
            Statement::WhileStmt(while_stmt) => while_stmt.evaluate(),
            Statement::PrntStmt(prnt_stmt) => prnt_stmt.evaluate(),
            Statement::Block(block) => block.evaluate(),
        }
    }
}

impl Eval for Expression {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for ExprStmt {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for IfStmt {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for ForStmt {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for WhileStmt {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for PrntStmt {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for Block {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for RtrnStmt {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for Equality {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.comparision.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.comparision.evaluate()
        }
    }
}

impl Eval for Comparision {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.term.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.term.evaluate()
        }
    }
}

impl Eval for Term {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.factor.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.factor.evaluate()
        }
    }
}

impl Eval for Factor {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.unary.evaluate()?;
            let left = next.evaluate()?;
            op.evaluate(left, right)
        } else {
            self.unary.evaluate()
        }
    }
}

impl Eval for Unary {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        match self {
            Unary::Un(op, unary) => op.evaluate(unary.evaluate()?),
            Unary::Call(primary) => todo!(),
        }
    }
}

impl Eval for Primary {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        match self {
            Primary::ParenExpr(expression) => expression.evaluate(),
            Primary::Number(_) => todo!(),
            Primary::String(_) => todo!(),
            Primary::Boolean(_) => todo!(),
            Primary::Identifier(_) => todo!(),
            Primary::GlobalId(_) => todo!(),
            Primary::This => todo!(),
            Primary::Nil => todo!(),
        }
    }
}

impl Eval for Declaration {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for ClassDecl {
    fn evaluate(&self) -> Result<Object, RuntimeError> {
        todo!()
    }
}
