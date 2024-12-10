use super::*;

impl Eval for Statement {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        match self {
            Statement::ExprStmt(expression) => expression.evaluate(env),
            Statement::ForStmt(for_stmt) => for_stmt.evaluate(env),
            Statement::IfStmt(if_stmt) => if_stmt.evaluate(env),
            Statement::RtrnStmt(rtrn_stmt) => rtrn_stmt.evaluate(env),
            Statement::WhileStmt(while_stmt) => while_stmt.evaluate(env),
            Statement::PrntStmt(prnt_stmt) => prnt_stmt.evaluate(env),
            Statement::Block(block) => block.evaluate(env),
        }
    }
}

impl Eval for ExprStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for IfStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for ForStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for WhileStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for PrntStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for Block {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for RtrnStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}
