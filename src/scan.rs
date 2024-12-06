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
            let token = match c {
                '(' => Token::new(TokenType::LeftParen, "(".into(), Literal::None),
                ')' => Token::new(TokenType::RightParen, ")".into(), Literal::None),
                '{' => Token::new(TokenType::LeftBrace, "{".into(), Literal::None),
                '}' => Token::new(TokenType::RightBrace, "}".into(), Literal::None),
                _ => continue,
            };
            self.tokens.push(token);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), Literal::None));
    }
}
