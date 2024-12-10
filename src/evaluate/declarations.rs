use super::*;

impl Eval for Declaration {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
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
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
    }
}
