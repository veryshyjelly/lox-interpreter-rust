use crate::ast::*;
use crate::token::{Literal, Token, TokenType};

pub struct Parser<'a> {
    src: &'a [Token],
    pub exprs: Vec<Expression>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token]) -> Self {
        Self {
            src: input,
            exprs: vec![],
        }
    }
    pub fn parse(&mut self) {
        while let Ok((expr, rst)) = Expression::parse(&self.src) {
            self.exprs.push(expr);
            self.src = rst;
        }
    }
}

impl Expression {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        let (eq, rem) = Equality::parse(src)?;
        Ok((Expression(eq), rem))
    }
}

impl EqualityOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        match src[0].token_type {
            TokenType::BangEqual => Ok((EqualityOp::NotEquals, &src[1..])),
            TokenType::Equal => Ok((EqualityOp::Equals, &src[1..])),
            _ => Err(src),
        }
    }
}

impl Equality {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        let (comparision, mut rem) = Comparision::parse(src)?;
        let mut rest = vec![];
        while let Ok((op, r)) = EqualityOp::parse(rem) {
            let (cmp, r) = Comparision::parse(r)?;
            rem = r;
            rest.push((op, cmp));
        }
        Ok((Equality { comparision, rest }, rem))
    }
}

impl ComparisionOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        match src[0].token_type {
            TokenType::Greater => Ok((ComparisionOp::Greater, &src[1..])),
            TokenType::GreaterEqual => Ok((ComparisionOp::GreaterEqual, &src[1..])),
            TokenType::Less => Ok((ComparisionOp::Less, &src[1..])),
            TokenType::LessEqual => Ok((ComparisionOp::LessEqual, &src[1..])),
            _ => Err(src),
        }
    }
}

impl Comparision {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        let (term, mut rem) = Term::parse(src)?;
        let mut rest = vec![];
        while let Ok((op, r)) = ComparisionOp::parse(rem) {
            let (term, r) = Term::parse(r)?;
            rem = r;
            rest.push((op, term));
        }
        Ok((Comparision { term, rest }, rem))
    }
}

impl TermOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        match src[0].token_type {
            TokenType::Minus => Ok((TermOp::Minus, &src[1..])),
            TokenType::Plus => Ok((TermOp::Plus, &src[1..])),
            _ => Err(src),
        }
    }
}

impl Term {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        let (factor, mut rem) = Factor::parse(src)?;
        let mut rest = vec![];
        while let Ok((op, r)) = TermOp::parse(rem) {
            let (factor, r) = Factor::parse(r)?;
            rem = r;
            rest.push((op, factor));
        }
        Ok((Term { factor, rest }, rem))
    }
}

impl FactorOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        match src[0].token_type {
            TokenType::Star => Ok((FactorOp::Mul, &src[1..])),
            TokenType::Slash => Ok((FactorOp::Div, &src[1..])),
            _ => Err(src),
        }
    }
}

impl Factor {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        let (unary, mut rem) = Unary::parse(src)?;
        let mut rest = vec![];
        while let Ok((op, r)) = FactorOp::parse(rem) {
            let (factor, r) = Unary::parse(r)?;
            rem = r;
            rest.push((op, factor));
        }
        Ok((Factor { unary, rest }, rem))
    }
}

impl UnaryOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        match src[0].token_type {
            TokenType::Bang => Ok((UnaryOp::Bang, &src[1..])),
            TokenType::Minus => Ok((UnaryOp::Minus, &src[1..])),
            _ => Err(src),
        }
    }
}

impl Unary {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        if let Ok((op, rem)) = UnaryOp::parse(src) {
            let (un, rem) = Unary::parse(rem)?;
            Ok((Unary::Un(op, Box::new(un)), rem))
        } else {
            let (pr, rem) = Primary::parse(src)?;
            Ok((Unary::Pr(pr), rem))
        }
    }
}

impl Primary {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), &'a [Token]> {
        match src[0].token_type {
            TokenType::Number => {
                if let Literal::Number(n) = src[0].literal {
                    Ok((Primary::Number(n), &src[1..]))
                } else {
                    Err(src)
                }
            }
            TokenType::String => {
                if let Literal::String(s) = src[0].literal.clone() {
                    Ok((Primary::String(s), &src[1..]))
                } else {
                    Err(src)
                }
            }
            TokenType::True => Ok((Primary::True, &src[1..])),
            TokenType::False => Ok((Primary::False, &src[1..])),
            TokenType::Nil => Ok((Primary::Nil, &src[1..])),
            TokenType::LeftParen => {
                let (expr, rst) = Expression::parse(&src[1..])?;
                if let TokenType::RightParen = rst[0].token_type {
                    Err(src)
                } else {
                    Ok((Primary::ParenExpr(Box::new(expr)), rst))
                }
            }
            _ => Err(src),
        }
    }
}