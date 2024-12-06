use std::fmt::Display;

pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        let d = match self {
            LeftParen => "LEFT_PAREN",
            RightParen => "RIGHT_PAREN",
            LeftBrace => "LEFT_BRACE",
            RightBrace => "RIGHT_BRACE",
            Comma => "COMMA",
            Dot => "DOT",
            Minus => "MINUS",
            Plus => "PLUS",
            Semicolon => "SEMICOLON",
            Slash => "SLASH",
            Star => "STAR",
            Bang => "BANG",
            BangEqual => "BANG_EQUAL",
            Equal => "EQUAL",
            EqualEqual => "EQUAL_EQUAL",
            Greater => "GREATER",
            GreaterEqual => "GREATER_EQUAL",
            Less => "LESS",
            LessEqual => "LESS_EQUAL",
            Identifier => "IDENTIFIER",
            String => "STRING",
            Number => "NUMBER",
            And => "AND",
            Class => "CLASS",
            Else => "ELSE",
            False => "FALSE",
            Fun => "FUN",
            For => "FOR",
            If => "IF",
            Nil => "NIL",
            Or => "OR",
            Print => "PRINT",
            Return => "RETURN",
            Super => "SUPER",
            This => "THIS",
            True => "TRUE",
            Var => "VAR",
            While => "WHILE",
            Eof => "EOF",
        };
        write!(f, "{}", d)
    }
}

pub enum Literal {
    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Literal::*;
        match self {
            None => write!(f, "null"),
        }
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
        }
    }
}
