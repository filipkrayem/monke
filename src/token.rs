#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),
    Illegal,
    Eof,
    Equal,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    Bang,
    Minus,
    Slash,
    Asterisk,
    LessThan,
    GreaterThan,
    If,
    Else,
    Return,
    True,
    False,
    EqualEqual,
    NotEqual,
}