use crate::ast::*;
use crate::token::{Literal, Token, TokenType};

pub struct Parser<'a> {
    src: &'a [Token],
    pub exprs: Vec<Expression>,
}

pub struct ParseError<'a> {
    pub tok: &'a Token,
    pub err: String,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token]) -> Self {
        Self {
            src: input,
            exprs: vec![],
        }
    }
    pub fn parse(&mut self) -> Result<(), ParseError<'a>> {
        while self.src[0].token_type != TokenType::Eof {
            let (expr, rst) = Expression::parse(&self.src)?;
            self.exprs.push(expr);
            self.src = rst;
        }
        Ok(())
    }
}

impl Expression {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (eq, rem) = Equality::parse(src, None)?;
        Ok((Expression(eq), rem))
    }
}

impl EqualityOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::BangEqual => Ok((EqualityOp::NotEquals, &src[1..])),
            TokenType::EqualEqual => Ok((EqualityOp::EqualEquals, &src[1..])),
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect != or ==.".into(),
            }),
        }
    }
}

impl Equality {
    fn parse<'a>(
        src: &'a [Token],
        rest: Option<(EqualityOp, Box<Equality>)>,
    ) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (comparision, rem) = Comparision::parse(src, None)?;
        if let Ok((op, rem)) = EqualityOp::parse(rem) {
            let eq = Equality { comparision, rest };
            Equality::parse(rem, Some((op, Box::new(eq))))
        } else {
            Ok((Equality { comparision, rest }, rem))
        }
    }
}

impl ComparisionOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::Greater => Ok((ComparisionOp::Greater, &src[1..])),
            TokenType::GreaterEqual => Ok((ComparisionOp::GreaterEqual, &src[1..])),
            TokenType::Less => Ok((ComparisionOp::Less, &src[1..])),
            TokenType::LessEqual => Ok((ComparisionOp::LessEqual, &src[1..])),
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect > or >= or < or <=.".into(),
            }),
        }
    }
}

impl Comparision {
    fn parse<'a>(
        src: &'a [Token],
        rest: Option<(ComparisionOp, Box<Comparision>)>,
    ) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (term, rem) = Term::parse(src, None)?;
        if let Ok((op, rem)) = ComparisionOp::parse(rem) {
            let eq = Comparision { term, rest };
            Comparision::parse(rem, Some((op, Box::new(eq))))
        } else {
            Ok((Comparision { term, rest }, rem))
        }
    }
}

impl TermOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::Minus => Ok((TermOp::Minus, &src[1..])),
            TokenType::Plus => Ok((TermOp::Plus, &src[1..])),
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect - or +.".into(),
            }),
        }
    }
}

impl Term {
    fn parse<'a>(
        src: &'a [Token],
        rest: Option<(TermOp, Box<Term>)>,
    ) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (factor, rem) = Factor::parse(src, None)?;
        if let Ok((op, rem)) = TermOp::parse(rem) {
            let eq = Term { factor, rest };
            Term::parse(rem, Some((op, Box::new(eq))))
        } else {
            Ok((Term { factor, rest }, rem))
        }
    }
}

impl FactorOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::Star => Ok((FactorOp::Mul, &src[1..])),
            TokenType::Slash => Ok((FactorOp::Div, &src[1..])),
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect * or /.".into(),
            }),
        }
    }
}

impl Factor {
    fn parse<'a>(
        src: &'a [Token],
        rest: Option<(FactorOp, Box<Factor>)>,
    ) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (unary, rem) = Unary::parse(src)?;
        if let Ok((op, rem)) = FactorOp::parse(rem) {
            let eq = Factor { unary, rest };
            Factor::parse(rem, Some((op, Box::new(eq))))
        } else {
            Ok((Factor { unary, rest }, rem))
        }
    }
}

impl UnaryOp {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::Bang => Ok((UnaryOp::Bang, &src[1..])),
            TokenType::Minus => Ok((UnaryOp::Minus, &src[1..])),
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect ! or -.".into(),
            }),
        }
    }
}

impl Unary {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
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
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::Number => {
                if let Literal::Number(n) = src[0].literal {
                    Ok((Primary::Number(n), &src[1..]))
                } else {
                    Err(ParseError {
                        tok: &src[0],
                        err: "Expect number.".into(),
                    })
                }
            }
            TokenType::String => {
                if let Literal::String(s) = src[0].literal.clone() {
                    Ok((Primary::String(s), &src[1..]))
                } else {
                    Err(ParseError {
                        tok: &src[0],
                        err: "Expect string.".into(),
                    })
                }
            }
            TokenType::True => Ok((Primary::Boolean(true), &src[1..])),
            TokenType::False => Ok((Primary::Boolean(false), &src[1..])),
            TokenType::Nil => Ok((Primary::Nil, &src[1..])),
            TokenType::LeftParen => {
                let (expr, rst) = Expression::parse(&src[1..])?;
                if let TokenType::RightParen = rst[0].token_type {
                    Ok((Primary::ParenExpr(Box::new(expr)), &rst[1..]))
                } else {
                    Err(ParseError {
                        tok: &src[0],
                        err: "Expect expression.".into(),
                    })
                }
            }
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect expression.".into(),
            }),
        }
    }
}
