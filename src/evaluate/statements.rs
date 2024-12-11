use super::*;

impl Eval for Statement {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
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
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        self.0.evaluate(env)
    }
}

fn get_do_or_not(v: Object) -> bool {
    match v {
        Object::Boolean(b) => b,
        Object::Nil => false,
        _ => true,
    }
}

impl Eval for IfStmt {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        let v = self.pred.evaluate(env)?;
        if get_do_or_not(v) {
            self.if_stmt.evaluate(env)
        } else if let Some(el) = &self.else_stmt {
            el.evaluate(env)
        } else {
            Ok(Object::Nil)
        }
    }
}

impl Eval for ForStmt {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        env.push(Env::default());
        let res = self.first_dec.evaluate(env)?;
        loop {
            if let Some(pred) = &self.scnd_expr {
                let p = pred.evaluate(env)?;
                if !get_do_or_not(p) {
                    break;
                }
            }
            self.body.evaluate(env)?;
            if let Some(post) = &self.thrd_expr {
                post.evaluate(env)?;
            }
        }
        env.pop();
        Ok(res)
    }
}

impl Eval for ForDec {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        match self {
            ForDec::VarDecl(var_decl) => var_decl.evaluate(env),
            ForDec::ExprStmt(expr_stmt) => expr_stmt.evaluate(env),
            ForDec::Nil => Ok(Object::Nil),
        }
    }
}

impl Eval for WhileStmt {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        while get_do_or_not(self.pred.evaluate(env)?) {
            self.stmt.evaluate(env)?;
        }
        Ok(Object::Nil)
    }
}

impl Eval for PrntStmt {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        let val = self.0.evaluate(env)?;
        println!("{val}");
        Ok(val)
    }
}

impl Eval for Block {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        env.push(Env::default());
        for d in &self.0 {
            d.evaluate(env)?;
        }
        env.pop();
        Ok(Object::Nil)
    }
}

impl Eval for RtrnStmt {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        todo!()
    }
}
