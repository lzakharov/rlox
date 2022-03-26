use std::fmt;

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Token {
    typ: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token {
            typ: typ,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }

    pub fn new_eof(line: usize) -> Token {
        Token {
            typ: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: line,
        }
    }

    pub fn is(&self, typ: TokenType) -> bool {
        self.typ == typ
    }

    pub fn get_type(&self) -> TokenType {
        self.typ
    }

    pub fn get_literal(&self) -> Option<Object> {
        self.literal.clone()
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
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

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
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
