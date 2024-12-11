use std::{ops::Deref, sync::Arc};

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
        let func = self.0.evaluate(env)?;
        let name = func.get_function().unwrap().name.clone();
        env.last_mut().unwrap().0.insert(name, func);
        Ok(Object::Nil)
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
        let res = |passed: Vec<Object>,
                   params: &Vec<String>,
                   body: &Block,
                   env1: &Vec<Env>,
                   env_global: &Vec<Env>| {
            let params = params.clone();
            let mut env = env_global.clone();

            env.extend_from_slice(&env1);
            env.push(Env::default());

            for (name, value) in params.into_iter().zip(passed) {
                env.last_mut().unwrap().0.insert(name, value);
            }

            for expr in &body.0 {
                let val = expr.evaluate(&mut env)?;
                match val {
                    Object::Return(v) => return Ok(v.deref().clone()),
                    _ => {}
                }
            }

            Ok(Object::Nil)
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
