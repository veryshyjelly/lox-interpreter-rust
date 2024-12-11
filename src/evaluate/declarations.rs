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
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError> {
        todo!()
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
