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
        self.0.evaluate(env)
    }
}

impl Eval for IfStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        if self
            .pred
            .evaluate(env.clone())?
            .0
            .get_bool()
            .ok_or(RuntimeError {
                err: "expected bool".into(),
            })?
        {
            self.if_stmt.evaluate(env)
        } else if let Some(el) = &self.else_stmt {
            el.evaluate(env)
        } else {
            Ok((Object::Nil, env))
        }
    }
}

impl Eval for ForStmt {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        let (res, env) = self.first_dec.evaluate(env)?;

        Ok((res, env))
    }
}

impl Eval for ForDec {
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
        let (val, env) = self.0.evaluate(env)?;
        println!("{val}");
        Ok((val, env))
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
