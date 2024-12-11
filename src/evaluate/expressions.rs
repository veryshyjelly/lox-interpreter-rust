use environment::find_id;

use super::*;

impl Eval for Expression {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        self.0.evaluate(env)
    }
}

impl Eval for Assignment {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        match self {
            Assignment::Assign(call, assignment) => match &call.prime {
                Primary::Identifier(id) => {
                    let res = assignment.evaluate(env)?;
                    let ev = find_id(id, env).ok_or(RuntimeError {
                        err: format!("unbound variable {id}"),
                    })?;
                    ev.0.insert(id.clone(), res.clone());
                    Ok(res)
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
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        let right = self.and.evaluate(env)?;
        if let Some(rest) = &self.rest {
            let left = rest.evaluate(env)?;
            Ok(Self::or(left, right))
        } else {
            Ok(right)
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
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        let right = self.eq.evaluate(env)?;
        if let Some(rest) = &self.rest {
            let left = rest.evaluate(env)?;
            Ok(Self::and(left, right))
        } else {
            Ok(right)
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
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.comparision.evaluate(env)?;
            let left = next.evaluate(env)?;
            op.evaluate(left, right)
        } else {
            self.comparision.evaluate(env)
        }
    }
}

impl Eval for Comparision {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.term.evaluate(env)?;
            let left = next.evaluate(env)?;
            op.evaluate(left, right)
        } else {
            self.term.evaluate(env)
        }
    }
}

impl Eval for Term {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.factor.evaluate(env)?;
            let left = next.evaluate(env)?;
            op.evaluate(left, right)
        } else {
            self.factor.evaluate(env)
        }
    }
}

impl Eval for Factor {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        if let Some((op, next)) = &self.rest {
            let right = self.unary.evaluate(env)?;
            let left = next.evaluate(env)?;
            op.evaluate(left, right)
        } else {
            self.unary.evaluate(env)
        }
    }
}

impl Eval for Unary {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        match self {
            Unary::Un(op, unary) => {
                let un = unary.evaluate(env)?;
                op.evaluate(un)
            }
            Unary::Call(call) => call.evaluate(env),
        }
    }
}

impl Eval for Call {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        let mut pr = self.prime.evaluate(env)?;
        for r in &self.rest {
            pr = r.evaluate(pr, env)?;
        }
        Ok(pr)
    }
}

impl Calling {
    fn evaluate(&self, exp: Object, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        match &self {
            Calling::FuncCall(arguments) => {
                let func = exp.get_function().ok_or(RuntimeError {
                    err: "Expect function".into(),
                })?;
                let args = arguments.as_ref().map_or(Ok(vec![]), |x| x.evaluate(env))?;
                func.fun.as_ref()(args, func.params, func.body, func.env)
            }
            Calling::Mthd(_) => todo!(),
        }
    }
}

impl Eval for Primary {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Object, RuntimeError> {
        match self {
            Primary::ParenExpr(expression) => expression.evaluate(env),
            Primary::Number(n) => Ok(Object::Number(*n)),
            Primary::String(s) => Ok(Object::String(s.clone())),
            Primary::Boolean(b) => Ok(Object::Boolean(*b)),
            Primary::Identifier(id) => {
                let ev = find_id(id, env).ok_or(RuntimeError {
                    err: format!("unbound variable {}", id),
                })?;
                Ok(ev.0.get(id).unwrap().clone())
            }
            Primary::SuperId(_) => todo!(),
            Primary::This => todo!(),
            Primary::Nil => Ok(Object::Nil),
        }
    }
}

impl Arguments {
    fn evaluate(&self, env: &mut Vec<Env>) -> Result<Vec<Object>, RuntimeError> {
        let mut rest = self.rest.as_ref().map_or(Ok(vec![]), |x| x.evaluate(env))?;
        let this = self.expr.evaluate(env)?;
        rest.push(this);
        Ok(rest)
    }
}
