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
        let iter = self.src.iter().enumerate().peekable();
        let mut line = 1;

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
