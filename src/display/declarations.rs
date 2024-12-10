use crate::ast::*;
use std::fmt::Display;

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::ClassDecl(class_decl) => write!(f, "{class_decl}"),
            Declaration::FunDecl(fun_decl) => write!(f, "{fun_decl}"),
            Declaration::VarDecl(var_decl) => write!(f, "{var_decl}"),
            Declaration::Statement(statement) => write!(f, "{statement}"),
        }
    }
}

impl Display for ClassDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for FunDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for VarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
