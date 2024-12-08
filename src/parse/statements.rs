use super::*;

impl Statement {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        if let Ok((stmt, rem)) = ExprStmt::parse(src) {
            Ok((Statement::ExprStmt(stmt), rem))
        } else if let Ok((stmt, rem)) = IfStmt::parse(src) {
            Ok((Statement::IfStmt(stmt), rem))
        } else if let Ok((stmt, rem)) = ForStmt::parse(src) {
            Ok((Statement::ForStmt(stmt), rem))
        } else if let Ok((stmt, rem)) = WhileStmt::parse(src) {
            Ok((Statement::WhileStmt(stmt), rem))
        } else if let Ok((stmt, rem)) = PrntStmt::parse(src) {
            Ok((Statement::PrntStmt(stmt), rem))
        } else if let Ok((stmt, rem)) = RtrnStmt::parse(src) {
            Ok((Statement::RtrnStmt(stmt), rem))
        } else if let Ok((stmt, rem)) = Block::parse(src) {
            Ok((Statement::Block(stmt), rem))
        } else {
            Err(ParseError {
                err: "".into(),
                tok: &src[0],
            })
        }
    }
}

impl ExprStmt {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (expr, rem) = Expression::parse(src)?;
        let rem = match_tok(rem, TokenType::Semicolon, "';' after expression")?;
        Ok((ExprStmt(expr), rem))
    }
}

impl IfStmt {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        todo!()
    }
}

impl ForStmt {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        todo!()
    }
}

impl WhileStmt {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        todo!()
    }
}

impl RtrnStmt {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        todo!()
    }
}

impl PrntStmt {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        todo!()
    }
}

impl Block {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        todo!()
    }
}
