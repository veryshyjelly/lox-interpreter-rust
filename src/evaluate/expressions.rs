use super::*;

impl Eval for Expression {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        self.0.evaluate(env)
    }
}

impl Eval for Assignment {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        match self {
            Assignment::Assign(call, assignment) => match &call.prime {
                Primary::Identifier(id) => {
                    let (res, mut env) = assignment.evaluate(env)?;
                    env.0.insert(id.clone(), res.clone());
                    Ok((res, env))
                }
                _ => Err(RuntimeError {
                    err: "Expect identifier".into(),
                }),
            },
            Assignment::LogicOr(logic_or) => logic_or.evaluate(env),
        }
    }
}

impl Eval for LogicOr {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        let (right, env) = self.and.evaluate(env)?;
        if let Some(rest) = &self.rest {
            let (left, env) = rest.evaluate(env)?;
            Ok((Self::or(left, right), env))
        } else {
            Ok((right, env))
        }
    }
}

impl LogicOr {
    fn or(left: Object, right: Object) -> Object {
        match left {
            Object::Boolean(b) => {
                if b {
                    left
                } else {
                    right
                }
            }
            Object::Nil => right,
            _ => left,
        }
    }
}

impl Eval for LogicAnd {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        let (right, env) = self.eq.evaluate(env)?;
        if let Some(rest) = &self.rest {
            let (left, env) = rest.evaluate(env)?;
            Ok((Self::and(left, right), env))
        } else {
            Ok((right, env))
        }
    }
}

impl LogicAnd {
    fn and(left: Object, right: Object) -> Object {
        match left {
            Object::Boolean(b) => {
                if b {
                    right
                } else {
                    left
                }
            }
            Object::Nil => left,
            _ => right,
        }
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
        let (mut pr, env) = self.prime.evaluate(env)?;
        for r in &self.rest {
            pr = r.evaluate(pr, &env)?;
        }
        Ok((pr, env))
    }
}

impl Calling {
    fn evaluate(&self, exp: Object, env: &Env) -> Result<Object, RuntimeError> {
        match &self {
            Calling::FuncCall(arguments) => {
                let func = exp.get_function().ok_or(RuntimeError {
                    err: "Expect function".into(),
                })?;
                let args = arguments.as_ref().map_or(Ok(vec![]), |x| x.evaluate(env))?;
                func.as_ref()(args)
            }
            Calling::Mthd(_) => todo!(),
        }
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
            Primary::SuperId(_) => todo!(),
            Primary::This => todo!(),
            Primary::Nil => Ok((Object::Nil, env)),
        }
    }
}

impl Arguments {
    fn evaluate(&self, env: &Env) -> Result<Vec<Object>, RuntimeError> {
        let mut rest = self.rest.as_ref().map_or(Ok(vec![]), |x| x.evaluate(env))?;
        let (this, _) = self.expr.evaluate(env.clone())?;
        rest.push(this);
        Ok(rest)
    }
}
