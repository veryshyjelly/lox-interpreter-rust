use crate::ast::{declarations::*, expressions::*, statements::*};
use environment::Env;
pub use object::*;

pub mod declarations;
pub mod environment;
pub mod expressions;
pub mod object;
pub mod operations;
pub mod statements;

pub trait Eval {
    fn evaluate(&self, env: Env) -> Result<(Object, Env), RuntimeError>;
}

pub trait EvalBinOp {
    fn evaluate(&self, left: Object, right: Object) -> Result<Object, RuntimeError>;
}

pub trait EvalUnOp {
    fn evaluate(&self, exp: Object) -> Result<Object, RuntimeError>;
}

#[derive(Clone)]
pub struct RuntimeError {
    pub err: String,
}
