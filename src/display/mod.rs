use crate::ast::*;
use std::fmt::Display;

mod declarations;
mod expressions;
mod statements;

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in &self.declarations {
            writeln!(f, "{d}")?
        }
        Ok(())
    }
}

fn indent(s: String) -> String {
    s.lines().collect::<Vec<_>>().join("\n\t")
}
