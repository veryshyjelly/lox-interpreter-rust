use crate::token::{Literal, Token, TokenType};

pub struct Scanner {
    src: Vec<char>,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            src: input.chars().collect(),
            tokens: vec![],
        }
    }

    pub fn scan(&mut self) {
        let iter = self.src.iter().enumerate().peekable();

        for (_, c) in iter {
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
                _ => continue,
            };
            let token = Token::new(tp, eme.into(), lrl);
            self.tokens.push(token);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), Literal::None));
    }
}
