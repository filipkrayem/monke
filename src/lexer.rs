use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,      // current position aka current char
    read_position: usize, // next char
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.read_char();

        lexer
    }

    pub fn all_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.next_token();

            if token == Token::Eof {
                break;
            }

            tokens.push(token);
        }

        tokens
    }

    fn read_char(&mut self) {
        self.ch = self
            .input
            .get(self.read_position)
            .unwrap_or(&'\0')
            .to_owned();

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        self.input
            .get(self.read_position)
            .unwrap_or(&'\0')
            .to_owned()
    }

    fn read_identifier(&mut self) -> String {
        let starting_position = self.position;

        while self.ch.is_letter() {
            self.read_char()
        }

        return self.input[starting_position..self.position]
            .iter()
            .collect();
    }

    fn read_digit(&mut self) -> String {
        let starting_position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char()
        }

        return self.input[starting_position..self.position]
            .iter()
            .collect();
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token: Token;

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::EqualEqual;
                } else {
                    token = Token::Equal;
                }
            }
            ';' => token = Token::Semicolon,
            '(' => token = Token::LParen,
            ')' => token = Token::RParen,
            ',' => token = Token::Comma,
            '+' => token = Token::Plus,
            '{' => token = Token::LBrace,
            '}' => token = Token::RBrace,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::NotEqual;
                } else {
                    token = Token::Bang;
                }
            }
            '-' => token = Token::Minus,
            '/' => token = Token::Slash,
            '*' => token = Token::Asterisk,
            '<' => token = Token::LessThan,
            '>' => token = Token::GreaterThan,
            '\0' => token = Token::Eof,
            _ => {
                if self.ch.is_letter() {
                    let ident = self.read_identifier();

                    token = match ident.as_str() {
                        "fn" => Token::Function,
                        "let" => Token::Let,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        "true" => Token::True,
                        "false" => Token::False,
                        _ => Token::Ident(ident),
                    };
                } else if self.ch.is_ascii_digit() {
                    let digit = self.read_digit();
                    token = Token::Int(digit);
                } else {
                    token = Token::Illegal;
                }
                return token;
            }
        }

        self.read_char();

        token
    }
}

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_letter(&self) -> bool {
        'a' <= self.to_ascii_lowercase() && self.to_ascii_lowercase() <= 'z' || self == &'_'
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer::*;

    #[test]
    fn next_token() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ";

        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Equal,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Equal,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Equal,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Equal,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::LessThan,
            Token::Int("10".to_string()),
            Token::GreaterThan,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".to_string()),
            Token::LessThan,
            Token::Int("10".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int("10".to_string()),
            Token::EqualEqual,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::NotEqual,
            Token::Int("9".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        tests.into_iter().for_each(|expected| {
            let token = lexer.next_token();
            assert_eq!(token, expected)
        })
    }
}
