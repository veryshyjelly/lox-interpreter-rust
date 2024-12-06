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
            match c {
                '(' => self.tokens.push(Token::new(
                    TokenType::LeftParen,
                    "(".to_string(),
                    Literal::None,
                )),
                ')' => self.tokens.push(Token::new(
                    TokenType::RightParen,
                    ")".to_string(),
                    Literal::None,
                )),
                _ => {}
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), Literal::None));
    }
}
