use crate::utils::map_token_to_literal::map_token_to_literal;

#[derive(Debug, PartialEq, Clone)]
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

pub trait Literal {
    fn literal(&self) -> String;
}

impl Literal for Token {
    fn literal(&self) -> String {
        return map_token_to_literal(self);
    }
}
