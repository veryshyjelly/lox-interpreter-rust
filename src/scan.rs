use crate::token::{Literal, Token, TokenType};

pub struct Scanner {
    src: Vec<char>,
    pub tokens: Vec<Token>,
    pub errors: Vec<ErrToken>,
}

pub struct ErrToken {
    pub line: usize,
    pub tok: char,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            src: input.chars().collect(),
            tokens: vec![],
            errors: vec![],
        }
    }

    pub fn scan(&mut self) {
        let mut iter = self.src.iter().peekable();
        let mut line = 1;

        while let Some(c) = iter.next() {
            use Literal::*;
            use TokenType::*;
            let (tp, eme, lrl) = match c {
                '(' => (LeftParen, "(", None),
                ')' => (RightParen, ")", None),
                '{' => (LeftBrace, "{", None),
                '}' => (RightBrace, "}", None),
                ',' => (Comma, ",", None),
                '.' => (Dot, ".", None),
                '-' => (Minus, "-", None),
                '+' => (Plus, "+", None),
                ';' => (Semicolon, ";", None),
                '*' => (Star, "*", None),
                '=' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (EqualEqual, "==", None)
                    } else {
                        (Equal, "=", None)
                    }
                }
                '!' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (BangEqual, "!=", None)
                    } else {
                        (Bang, "!", None)
                    }
                }
                '>' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (GreaterEqual, ">=", None)
                    } else {
                        (Greater, ">", None)
                    }
                }
                '<' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (LessEqual, "<=", None)
                    } else {
                        (Less, "<", None)
                    }
                }
                '\n' => {
                    line += 1;
                    continue;
                }
                c => {
                    self.errors.push(ErrToken {
                        line: line,
                        tok: c.clone(),
                    });
                    continue;
                }
            };
            let token = Token::new(tp, eme.into(), lrl);
            self.tokens.push(token);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), Literal::None));
    }
}
