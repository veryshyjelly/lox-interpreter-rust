use super::*;

impl Eval for Declaration {
    fn evaluate(&self, env: Rc<RefCell<Env>>) -> Result<Object, RuntimeError> {
        match self {
            Declaration::ClassDecl(class_decl) => class_decl.evaluate(env),
            Declaration::FunDecl(fun_decl) => fun_decl.evaluate(env),
            Declaration::VarDecl(var_decl) => var_decl.evaluate(env),
            Declaration::Statement(statement) => statement.evaluate(env),
        }
    }
}

impl Eval for ClassDecl {
    fn evaluate(&self, _: Rc<RefCell<Env>>) -> Result<Object, RuntimeError> {
        todo!()
    }
}

impl Eval for FunDecl {
    fn evaluate(&self, env: Rc<RefCell<Env>>) -> Result<Object, RuntimeError> {
        let func = self.0.evaluate(env.clone())?;
        let name = func.get_function().unwrap().name.clone();
        env.borrow_mut().values.insert(name, func);
        Ok(Object::Nil)
    }
}

impl Eval for VarDecl {
    fn evaluate(&self, env: Rc<RefCell<Env>>) -> Result<Object, RuntimeError> {
        let res = if let Some(e) = &self.expr {
            e.evaluate(env.clone())?
        } else {
            Object::Nil
        };
        env.borrow_mut()
            .values
            .insert(self.name.clone(), res.clone());
        Ok(res)
    }
}

impl Eval for Function {
    fn evaluate(&self, env: Rc<RefCell<Env>>) -> Result<Object, RuntimeError> {
        let params = self
            .params
            .as_ref()
            .map_or(Ok(vec![]), |x| x.evaluate(env.clone()))?;

        let res =
            |passed: Vec<Object>, params: &Vec<String>, body: &Block, env: Rc<RefCell<Env>>| {
                if passed.len() != params.len() {
                    return Err(RuntimeError {
                        err: format!(
                            "Expect {} arguments but got {}.",
                            params.len(),
                            passed.len()
                        ),
                    });
                }
                let params = params.clone();

                let env = Env::new_box_it(Some(env));

                for (name, value) in params.into_iter().zip(passed) {
                    env.borrow_mut().values.insert(name, value);
                }

                for expr in &body.0 {
                    let val = expr.evaluate(env.clone())?;
                    match val {
                        Object::Return(v) => return Ok(v.as_ref().clone()),
                        _ => {}
                    }
                }

                Ok(Object::Nil)
            };

        Ok(Object::Function(ExFn {
            name: self.name.clone(),
            fun: Arc::new(res),
            body: self.body.clone(),
            env: env.clone(),
            params,
        }))
    }
}

impl Parameters {
    fn evaluate(&self, env: Rc<RefCell<Env>>) -> Result<Vec<String>, RuntimeError> {
        let mut res = self.rest.as_ref().map_or(Ok(vec![]), |x| x.evaluate(env))?;
        let this = self.param.clone();
        res.push(this);
        Ok(res)
    }
}
