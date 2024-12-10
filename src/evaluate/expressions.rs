use super::*;

impl Eval for Expression {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}

impl Eval for Equality {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let (right, env) = self.comparision.evaluate(env)?;
            let (left, env) = next.evaluate(env)?;
            let res = op.evaluate(left, right)?;
            Ok((res, env))
        } else {
            self.comparision.evaluate(env)
        }
    }
}

impl Eval for Comparision {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let (right, env) = self.term.evaluate(env)?;
            let (left, env) = next.evaluate(env)?;
            let res = op.evaluate(left, right)?;
            Ok((res, env))
        } else {
            self.term.evaluate(env)
        }
    }
}

impl Eval for Term {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let (right, env) = self.factor.evaluate(env)?;
            let (left, env) = next.evaluate(env)?;
            let res = op.evaluate(left, right)?;
            Ok((res, env))
        } else {
            self.factor.evaluate(env)
        }
    }
}

impl Eval for Factor {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let (right, env) = self.unary.evaluate(env)?;
            let (left, env) = next.evaluate(env)?;
            let res = op.evaluate(left, right)?;
            Ok((res, env))
        } else {
            self.unary.evaluate(env)
        }
    }
}

impl Eval for Unary {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        match self {
            Unary::Un(op, unary) => {
                let (un, env) = unary.evaluate(env)?;
                Ok((op.evaluate(un)?, env))
            }
            Unary::Call(call) => call.evaluate(env),
        }
    }
}

impl Eval for Call {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        let (pr, env) = self.prime.evaluate(env)?;
        todo!()
    }
}

impl Eval for Primary {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        match self {
            Primary::ParenExpr(expression) => expression.evaluate(env),
            Primary::Number(n) => Ok((Object::Number(*n), env)),
            Primary::String(s) => Ok((Object::String(s.clone()), env)),
            Primary::Boolean(b) => Ok((Object::Boolean(*b), env)),
            Primary::Identifier(id) => {
                let val = env.0.get(id).ok_or(RuntimeError {
                    err: format!("unbound variable {}", id),
                })?;
                Ok((val.clone(), env))
            }
            Primary::SuperId(id) => todo!(),
            Primary::This => todo!(),
            Primary::Nil => Ok((Object::Nil, env)),
        }
    }
}
