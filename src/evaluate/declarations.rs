use std::sync::Arc;

use super::*;

impl Eval for Declaration {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        match self {
            Declaration::ClassDecl(class_decl) => class_decl.evaluate(env),
            Declaration::FunDecl(fun_decl) => fun_decl.evaluate(env),
            Declaration::VarDecl(var_decl) => var_decl.evaluate(env),
            Declaration::Statement(statement) => statement.evaluate(env),
        }
    }
}

impl Eval for ClassDecl {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for FunDecl {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        self.0.evaluate(env)
    }
}

impl Eval for VarDecl {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        let res = if let Some(e) = &self.expr {
            e.evaluate(env)?
        } else {
            Object::Nil
        };
        env.last_mut()
            .unwrap()
            .0
            .insert(self.name.clone(), res.clone());
        Ok(res)
    }
}

impl Eval for Function {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        let res = |passed: Vec<Object>, params: Vec<String>, body: Block, env: Vec<Env>| {
            let params = params.clone();
            let mut env = env.clone();
            env.push(Env::default());
            for (name, value) in params.into_iter().zip(passed) {
                env.last_mut().unwrap().0.insert(name, value);
            }
            body.evaluate(&mut env)
        };

        let params = self
            .params
            .as_ref()
            .map_or(Ok(vec![]), |x| x.evaluate(env))?;

        Ok(Object::Function(ExFn {
            name: self.name.clone(),
            fun: Arc::new(res),
            body: self.body.clone(),
            params,
            env: env.clone(),
        }))
    }
}

impl Parameters {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Vec<String>, RuntimeError> {
        let mut res = self.rest.as_ref().map_or(Ok(vec![]), |x| x.evaluate(env))?;
        let this = self.param.clone();
        res.push(this);
        Ok(res)
    }
}
