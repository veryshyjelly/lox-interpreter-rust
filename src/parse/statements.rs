use super::*;

impl Statement {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        Self::parse_expr_stmt(src)
            .or_else(|_| Self::parse_for_stmt(src))
            .or_else(|_| Self::parse_if_stmt(src))
            .or_else(|_| Self::parse_prnt_stmt(src))
            .or_else(|_| Self::parse_rtrn_stmt(src))
            .or_else(|_| Self::parse_while_stmt(src))
            .or_else(|_| Self::parse_block(src))
    }

    fn parse_expr_stmt<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (stmt, rem) = ExprStmt::parse(src)?;
        Ok((Statement::ExprStmt(stmt), rem))
    }

    fn parse_if_stmt<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (stmt, rem) = IfStmt::parse(src)?;
        Ok((Statement::IfStmt(stmt), rem))
    }

    fn parse_for_stmt<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (stmt, rem) = ForStmt::parse(src)?;
        Ok((Statement::ForStmt(stmt), rem))
    }

    fn parse_while_stmt<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (stmt, rem) = WhileStmt::parse(src)?;
        Ok((Statement::WhileStmt(stmt), rem))
    }

    fn parse_rtrn_stmt<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (stmt, rem) = RtrnStmt::parse(src)?;
        Ok((Statement::RtrnStmt(stmt), rem))
    }

    fn parse_prnt_stmt<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (stmt, rem) = PrntStmt::parse(src)?;
        Ok((Statement::PrntStmt(stmt), rem))
    }

    fn parse_block<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (stmt, rem) = Block::parse(src)?;
        Ok((Statement::Block(stmt), rem))
    }
}

impl ExprStmt {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (expr, rem) = Expression::parse(src)?;
        let rem = match_tok(rem, TokenType::Semicolon, "';' after expression")?;
        Ok((ExprStmt(expr), rem))
    }
}

impl IfStmt {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::If, "if")?;
        let rem = match_tok(rem, TokenType::LeftParen, "'(' after if")?;
        let (pred, rem) = Expression::parse(rem)?;
        let rem = match_tok(rem, TokenType::RightParen, ")")?;
        let (if_stmt, mut rem) = Statement::parse(rem)?;
        let mut else_stmt = None;
        if let Ok(r) = match_tok(rem, TokenType::Else, "else") {
            let (e, r) = Statement::parse(r)?;
            rem = r;
            let _ = else_stmt.insert(Box::new(e));
        }
        Ok((
            IfStmt {
                pred,
                if_stmt: Box::new(if_stmt),
                else_stmt,
            },
            rem,
        ))
    }
}

impl ForStmt {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::For, "for")?;
        let rem = match_tok(rem, TokenType::LeftParen, "'(' after for")?;
        let (first_dec, mut rem) = ForDec::parse(rem)?;
        let mut scnd_expr = None;
        if let Ok((scnd, r)) = Expression::parse(rem) {
            let _ = scnd_expr.insert(scnd);
            rem = r;
        }
        let mut rem = match_tok(rem, TokenType::Semicolon, "';' in for")?;
        let mut thrd_expr = None;
        if let Ok((thrd, r)) = Expression::parse(rem) {
            let _ = thrd_expr.insert(thrd);
            rem = r;
        }
        let rem = match_tok(rem, TokenType::RightParen, "')' after expression")?;
        let (body, rem) = Statement::parse(rem)?;
        Ok((
            ForStmt {
                first_dec,
                scnd_expr,
                thrd_expr,
                body: Box::new(body),
            },
            rem,
        ))
    }
}

impl ForDec {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        if let Ok((var, rem)) = VarDecl::parse(src) {
            Ok((ForDec::VarDecl(var), rem))
        } else if let Ok((expr, rem)) = ExprStmt::parse(src) {
            Ok((ForDec::ExprStmt(expr), rem))
        } else {
            let rem = match_tok(src, TokenType::Semicolon, ";")?;
            Ok((ForDec::Nil, rem))
        }
    }
}

impl WhileStmt {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::While, "while")?;
        let rem = match_tok(rem, TokenType::LeftParen, "'(' after while")?;
        let (pred, rem) = Expression::parse(rem)?;
        let rem = match_tok(rem, TokenType::RightParen, "')' after expression")?;
        let (if_stmt, rem) = Statement::parse(rem)?;
        Ok((
            WhileStmt {
                pred,
                stmt: Box::new(if_stmt),
            },
            rem,
        ))
    }
}

impl RtrnStmt {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::Return, "'return'")?;
        if let Ok((e, rem)) = Expression::parse(rem) {
            let rem = match_tok(rem, TokenType::Semicolon, ";")?;
            Ok((RtrnStmt(Some(e)), rem))
        } else {
            let rem = match_tok(rem, TokenType::Semicolon, ";")?;
            Ok((RtrnStmt(None), rem))
        }
    }
}

impl PrntStmt {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::Print, "'print'")?;
        let (expr, rem) = Expression::parse(rem)?;
        let rem = match_tok(rem, TokenType::Semicolon, ";")?;
        Ok((PrntStmt(expr), rem))
    }
}

impl Block {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let mut rem = match_tok(src, TokenType::LeftBrace, "'{' before block")?;
        let mut decrs = vec![];
        while let Ok((dec, r)) = Declaration::parse(rem) {
            decrs.push(dec);
            rem = r;
        }
        let rem = match_tok(rem, TokenType::RightBrace, "'}' after block")?;
        Ok((Block(decrs), rem))
    }
}
