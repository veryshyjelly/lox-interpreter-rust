pub mod declarations;
pub mod expressions;
pub mod statements;

pub use declarations::*;
pub use expressions::*;
pub use statements::*;

use crate::ast::*;
use crate::token::{Literal, Token, TokenType};

pub struct Parser<'a> {
    src: &'a [Token],
    pub program: Option<Program>,
}

pub struct ParseError<'a> {
    pub tok: &'a Token,
    pub err: String,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token]) -> Self {
        Self {
            src: input,
            program: None,
        }
    }
    pub fn parse(&mut self) -> Result<(), ParseError<'a>> {
        let program = Program::parse(&self.src)?;
        let _ = self.program.insert(program);
        Ok(())
    }
}

impl Program {
    fn parse<'a>(mut src: &'a [Token]) -> Result<Self, ParseError<'a>> {
        let mut program = Program {
            declarations: vec![],
        };
        while src[0].token_type != TokenType::Eof {
            let (stmt, rest) = Declaration::parse(src)?;
            // program.declarations.push(stmt);
            // src = rest;
        }
        Ok(program)
    }
}

fn match_tok<'a>(
    src: &'a [Token],
    tp: TokenType,
    expect: &str,
) -> Result<&'a [Token], ParseError<'a>> {
    if src[0].token_type == tp {
        Ok(&src[1..])
    } else {
        Err(ParseError {
            tok: &src[0],
            err: format!("Expect {}.", expect),
        })
    }
}
