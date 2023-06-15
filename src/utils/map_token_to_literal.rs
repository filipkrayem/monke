use crate::token::Token;

pub fn map_token_to_literal(token: &Token) -> String {
    match token {
        Token::Ident(ident) => ident.to_string(),
        Token::Int(int) => int.to_string(),
        Token::Equal => String::from("="),
        Token::Plus => String::from("+"),
        Token::Comma => String::from(","),
        Token::Semicolon => String::from(";"),
        Token::LParen => String::from("("),
        Token::RParen => String::from(")"),
        Token::LBrace => String::from("{"),
        Token::RBrace => String::from("}"),
        Token::Bang => String::from("!"),
        Token::Minus => String::from("-"),
        Token::Slash => String::from("/"),
        Token::Asterisk => String::from("*"),
        Token::LessThan => String::from("<"),
        Token::GreaterThan => String::from(">"),
        Token::Function => String::from("fn"),
        Token::Let => String::from("let"),
        Token::If => String::from("if"),
        Token::Else => String::from("else"),
        Token::Return => String::from("return"),
        Token::True => String::from("true"),
        Token::False => String::from("false"),
        Token::EqualEqual => String::from("=="),
        Token::NotEqual => String::from("!="),
        Token::Illegal => String::from("ILLEGAL"),
        Token::Eof => String::from("EOF"),
    }
}
