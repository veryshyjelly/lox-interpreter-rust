use super::*;

impl Eval for Declaration {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        match self {
            Declaration::ClassDecl(class_decl) => class_decl.evaluate(env),
            Declaration::FunDecl(fun_decl) => fun_decl.evaluate(env),
            Declaration::VarDecl(var_decl) => var_decl.evaluate(env),
            Declaration::Statement(statement) => statement.evaluate(env),
        }
    }
}

impl Eval for ClassDecl {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for FunDecl {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for VarDecl {
    fn evaluate(&self, mut env: Env) -> Result<(Object, Env), RuntimeError> {
        let v = Object::Nil;
        env.0.insert(self.name.clone(), v.clone());
        if let Some(e) = &self.expr {
            let (res, mut env) = e.evaluate(env)?;
            env.0.insert(self.name.clone(), res.clone());
            Ok((res, env))
        } else {
            Ok((v, env))
        }
    }
}

impl Eval for Function {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for Parameters {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}
