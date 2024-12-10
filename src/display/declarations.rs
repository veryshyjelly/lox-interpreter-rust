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
        write!(f, "class {}", self.name)?;
        if let Some(supa) = &self.super_class {
            write!(f, " < {supa}")?;
        }
        writeln!(f, "{{")?;
        for func in &self.functions {
            writeln!(f, "{func}")?;
        }
        writeln!(f, "}}")
    }
}

impl Display for FunDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fun {}", self.0)
    }
}

impl Display for VarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "var {}", self.name)?;
        if let Some(e) = &self.expr {
            write!(f, " = {e}")?;
        }
        writeln!(f, ";")
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}) {}",
            self.name,
            self.params.as_ref().map_or("".into(), |x| x.to_string()),
            self.body
        )
    }
}

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.param)?;
        match &self.rest {
            Some(v) => write!(f, ", {v}"),
            None => Ok(()),
        }
    }
}
