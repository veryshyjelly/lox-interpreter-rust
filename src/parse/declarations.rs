use super::*;

impl Declaration {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        if let Ok((cls, rem)) = ClassDecl::parse(src) {
            Ok((Declaration::ClassDecl(cls), rem))
        } else if let Ok((fun, rem)) = FunDecl::parse(src) {
            Ok((Declaration::FunDecl(fun), rem))
        } else if let Ok((var, rem)) = VarDecl::parse(src) {
            Ok((Declaration::VarDecl(var), rem))
        } else {
            let (stmt, rem) = Statement::parse(src)?;
            Ok((Declaration::Statement(stmt), rem))
        }
    }
}

impl ClassDecl {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::Class, "class declaration")?;
        let (class_name, mut rem) = get_identifier(rem)?;
        let mut super_class = None;
        if let Ok(r) = match_tok(rem, TokenType::Less, "derivation") {
            let (sup_class_name, r) = get_identifier(r)?;
            let _ = super_class.insert(sup_class_name);
            rem = r;
        }
        let mut funcs = vec![];
        while let Ok((func, r)) = Function::parse(rem) {
            funcs.push(func);
            rem = r;
        }
        Ok((
            ClassDecl {
                name: class_name,
                super_class,
                functions: funcs,
            },
            rem,
        ))
    }
}

impl FunDecl {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::Fun, "fun")?;
        let (func, rem) = Function::parse(rem)?;
        Ok((FunDecl(func), rem))
    }
}

impl VarDecl {
    pub fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let rem = match_tok(src, TokenType::Var, "var")?;
        let (name, rem) = get_identifier(rem)?;
        if let Ok(rem) = match_tok(src, TokenType::Equal, "=") {
            let (expr, rem) = Expression::parse(rem)?;
            let rem = match_tok(rem, TokenType::Semicolon, ";")?;
            Ok((
                VarDecl {
                    name,
                    expr: Some(expr),
                },
                rem,
            ))
        } else {
            let rem = match_tok(rem, TokenType::Semicolon, ";")?;
            Ok((VarDecl { name, expr: None }, rem))
        }
    }
}

impl Function {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (name, rem) = get_identifier(src)?;
        let mut rem = match_tok(rem, TokenType::LeftParen, "(")?;
        let mut params = None;
        if let Ok((parm, r)) = Parameters::parse(rem) {
            let _ = params.insert(parm);
            rem = r;
        }
        let (body, rem) = Block::parse(rem)?;
        Ok((Function { name, params, body }, rem))
    }
}

impl Parameters {
    fn parse<'a>(src: &'a [Token]) -> Result<(Self, &'a [Token]), ParseError<'a>> {
        let (name, mut rem) = get_identifier(src)?;
        let mut params = Parameters {
            param: name,
            rest: None,
        };

        while let Ok(r) = match_tok(rem, TokenType::Comma, ",") {
            let (name, r) = get_identifier(r)?;
            let next = Parameters {
                param: name,
                rest: Some(Box::new(params)),
            };
            params = next;
            rem = r;
        }

        Ok((params, rem))
    }
}
