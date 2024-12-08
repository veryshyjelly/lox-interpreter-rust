use crate::evaluate::{Eval, RuntimeError};

use super::*;

#[derive(Debug, Clone)]
pub struct Declaration {}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    name: String,
    super_class: Option<String>,
    functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct FunDecl(pub Function);
#[derive(Debug, Clone)]

pub struct VarDecl {
    name: String,
    expr: Expression,
}

impl Eval for Declaration {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}

impl Eval for ClassDecl {
    fn evaluate(&self) -> Result<Primary, RuntimeError> {
        todo!()
    }
}
