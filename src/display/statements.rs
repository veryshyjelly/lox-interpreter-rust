use crate::{ast::*, display::indent};
use std::fmt::Display;

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::ExprStmt(expr_stmt) => expr_stmt.fmt(f),
            Statement::ForStmt(for_stmt) => for_stmt.fmt(f),
            Statement::IfStmt(if_stmt) => if_stmt.fmt(f),
            Statement::PrntStmt(prnt_stmt) => prnt_stmt.fmt(f),
            Statement::RtrnStmt(rtrn_stmt) => rtrn_stmt.fmt(f),
            Statement::WhileStmt(while_stmt) => while_stmt.fmt(f),
            Statement::Block(block) => block.fmt(f),
        }
    }
}

impl Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.0)
    }
}

impl Display for IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "if ({}) \n\t{}",
            self.pred,
            indent(self.if_stmt.to_string())
        )?;
        if let Some(el) = &self.else_stmt {
            write!(f, "else \n\t {}", el)?
        }
        Ok(())
    }
}

impl Display for ForStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "for ({}; {}; {}) \n\t{}",
            self.first_dec,
            self.scnd_expr.as_ref().map_or("".into(), |x| x.to_string()),
            self.thrd_expr.as_ref().map_or("".into(), |x| x.to_string()),
            indent(self.body.to_string())
        )
    }
}

impl Display for ForDec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForDec::VarDecl(var_decl) => var_decl.fmt(f),
            ForDec::ExprStmt(expr_stmt) => expr_stmt.fmt(f),
            ForDec::Nil => Ok(()),
        }
    }
}

impl Display for WhileStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "while ({}) \n\t{}",
            self.pred,
            indent(self.stmt.to_string())
        )
    }
}

impl Display for RtrnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "return {};",
            self.0.as_ref().map_or("".into(), |x| x.to_string())
        )
    }
}

impl Display for PrntStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "print {};", self.0)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self
            .0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{{\n\t{}\n}}", indent(body))
    }
}
