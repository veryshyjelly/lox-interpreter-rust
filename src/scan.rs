use crate::token::{Literal, Token, TokenType};

pub struct Scanner {
    src: Vec<char>,
    pub tokens: Vec<Token>,
    pub errors: Vec<ErrToken>,
}

pub struct ErrToken {
    pub line: usize,
    pub tok: String,
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
            let (tp, eme, lrl) = match c {
                '(' => (TokenType::LeftParen, "(".into(), Literal::None),
                ')' => (TokenType::RightParen, ")".into(), Literal::None),
                '{' => (TokenType::LeftBrace, "{".into(), Literal::None),
                '}' => (TokenType::RightBrace, "}".into(), Literal::None),
                ',' => (TokenType::Comma, ",".into(), Literal::None),
                '.' => (TokenType::Dot, ".".into(), Literal::None),
                '-' => (TokenType::Minus, "-".into(), Literal::None),
                '+' => (TokenType::Plus, "+".into(), Literal::None),
                ';' => (TokenType::Semicolon, ";".into(), Literal::None),
                '*' => (TokenType::Star, "*".into(), Literal::None),
                '=' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (TokenType::EqualEqual, "==".into(), Literal::None)
                    } else {
                        (TokenType::Equal, "=".into(), Literal::None)
                    }
                }
                '!' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (TokenType::BangEqual, "!=".into(), Literal::None)
                    } else {
                        (TokenType::Bang, "!".into(), Literal::None)
                    }
                }
                '>' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (TokenType::GreaterEqual, ">=".into(), Literal::None)
                    } else {
                        (TokenType::Greater, ">".into(), Literal::None)
                    }
                }
                '<' => {
                    if let Some(&'=') = iter.peek() {
                        iter.next();
                        (TokenType::LessEqual, "<=".into(), Literal::None)
                    } else {
                        (TokenType::Less, "<".into(), Literal::None)
                    }
                }
                '"' => {
                    let mut res = vec![];
                    while let Some(&c) = iter.next_if(|&&x| x != '"') {
                        res.push(c);
                    }

                    if Some(&&'"') == iter.peek() {
                        iter.next();
                    } else {
                        self.errors.push(ErrToken {
                            line,
                            tok: "Unterminated string.".into(),
                        });
                        continue;
                    }

                    let s: String = res.into_iter().collect();

                    (TokenType::String, format!("\"{s}\""), Literal::String(s))
                }
                '/' => {
                    if let Some(&'/') = iter.peek() {
                        iter.position(|&x| x == '\n');
                        line += 1;
                        continue;
                    } else {
                        (TokenType::Slash, "/".into(), Literal::None)
                    }
                }
                '\n' => {
                    line += 1;
                    continue;
                }
                c => {
                    if !c.is_whitespace() {
                        self.errors.push(ErrToken {
                            line,
                            tok: format!("Unexpected character: {}", c),
                        });
                    }
                    continue;
                }
            };
            let token = Token::new(tp, eme, lrl);
            self.tokens.push(token);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), Literal::None));
    }
}
