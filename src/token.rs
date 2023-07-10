use crate::utils::map_token_to_literal::map_token_to_literal;

use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Clone)]
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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        // Compare only the variant discriminant
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only consider the variant, disregard the inner value
        std::mem::discriminant(self).hash(state);
    }
}
