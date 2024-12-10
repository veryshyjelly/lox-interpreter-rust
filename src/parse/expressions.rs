use super::*;

impl Expression {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (eq, rem) = Assignment::parse(src)?;
        Ok((Expression(eq), rem))
    }
}

impl Assignment {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        Self::parse_ass(src).or_else(|_| Self::parse_logic_or(src))
    }

    pub fn parse_ass<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (call, rem) = Call::parse(src)?;
        let rem = match_tok(rem, TokenType::Equal, "'=' in assignment")?;
        let (rest, rem) = Assignment::parse(rem)?;
        Ok((Assignment::Assign(call, Box::new(rest)), rem))
    }

    pub fn parse_logic_or<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (or, rest) = LogicOr::parse(src)?;
        Ok((Assignment::LogicOr(or), rest))
    }
}

impl LogicOr {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (and, mut rem) = LogicAnd::parse(src)?;
        let mut logic_or = LogicOr { and, rest: None };
        while let Ok(r) = match_tok(rem, TokenType::Or, "or") {
            let (and, r) = LogicAnd::parse(r)?;
            let next = LogicOr {
                and,
                rest: Some(Box::new(logic_or)),
            };
            logic_or = next;
            rem = r;
        }
        Ok((logic_or, rem))
    }
}

impl LogicAnd {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (eq, mut rem) = Equality::parse(src, None)?;
        let mut logic_and = LogicAnd { eq, rest: None };
        while let Ok(r) = match_tok(rem, TokenType::And, "and") {
            let (eq, r) = Equality::parse(r, None)?;
            let next = LogicAnd {
                eq,
                rest: Some(Box::new(logic_and)),
            };
            logic_and = next;
            rem = r;
        }
        Ok((logic_and, rem))
    }
}

impl EqualityOp {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
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
        rest: Option<(EqualityOp, Box<Self>)>,
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
        rest: Option<(ComparisionOp, Box<Self>)>,
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
        rest: Option<(TermOp, Box<Self>)>,
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
            let (pr, rem) = Call::parse(src)?;
            Ok((Unary::Call(pr), rem))
        }
    }
}

impl Call {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (prime, mut rem) = Primary::parse(src)?;
        let mut callings = vec![];
        while let Ok((cls, r)) = Calling::parse(rem) {
            callings.push(cls);
            rem = r;
        }

        Ok((
            Call {
                prime,
                rest: callings,
            },
            rem,
        ))
    }
}

impl Calling {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::LeftParen => {
                if let Ok((arg, rem)) = Arguments::parse(&src[1..]) {
                    let rem = match_tok(rem, TokenType::RightParen, "')'")?;
                    Ok((Calling::FuncCall(Some(arg)), rem))
                } else {
                    let rem = match_tok(&src[1..], TokenType::RightParen, "')'")?;
                    Ok((Calling::FuncCall(None), rem))
                }
            }
            TokenType::Dot => {
                let (id, rem) = get_identifier(&src[1..])?;
                Ok((Calling::Mthd(id), rem))
            }
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect '(' or '.'".into(),
            }),
        }
    }
}

impl Primary {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        match src[0].token_type {
            TokenType::Number => {
                let s = src[0].literal.get_number().ok_or(ParseError {
                    tok: &src[0],
                    err: "Expect number.".into(),
                })?;
                Ok((Primary::Number(s), &src[1..]))
            }
            TokenType::String => {
                let s = src[0].literal.get_string().ok_or(ParseError {
                    tok: &src[0],
                    err: "Expect string.".into(),
                })?;
                Ok((Primary::String(s), &src[1..]))
            }
            TokenType::True => Ok((Primary::Boolean(true), &src[1..])),
            TokenType::False => Ok((Primary::Boolean(false), &src[1..])),
            TokenType::Nil => Ok((Primary::Nil, &src[1..])),
            TokenType::This => Ok((Primary::This, &src[1..])),
            TokenType::Super => {
                let rest = match_tok(&src[1..], TokenType::Dot, "'.' after super")?;
                let (id, rem) = get_identifier(rest)?;
                Ok((Primary::SuperId(id), rem))
            }
            TokenType::Identifier => {
                let (id, rem) = get_identifier(src)?;
                Ok((Primary::Identifier(id), rem))
            }
            TokenType::LeftParen => {
                let (expr, rst) = Expression::parse(&src[1..])?;
                let rst = match_tok(rst, TokenType::RightParen, "expression")?;

                Ok((Primary::ParenExpr(Box::new(expr)), rst))
            }
            _ => Err(ParseError {
                tok: &src[0],
                err: "Expect expression.".into(),
            }),
        }
    }
}

impl Arguments {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (expr, mut rem) = Expression::parse(src)?;
        let mut arguments = Arguments { expr, rest: None };
        while let Ok(r) = match_tok(rem, TokenType::Comma, ",") {
            let (expr, r) = Expression::parse(r)?;
            let next = Arguments {
                expr,
                rest: Some(Box::new(arguments)),
            };
            arguments = next;
            rem = r;
        }
        Ok((arguments, rem))
    }
}
