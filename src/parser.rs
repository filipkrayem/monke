use crate::{
    ast::{LetStatement, Program, Statement},
    identifier::Identifier,
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser<'_> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: vec![],
        };

        // set both current_token and peek_token to actual values
        parser.next_token();
        parser.next_token();

        return parser;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.current_token != Token::Eof {
            let statement = self.parse_statement();

            if let Some(statement) = statement {
                program.statements.push(statement);
            }
            self.next_token();
        }

        return program;
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        let val = match self.current_token {
            Token::Let => return self.parse_let_statement(),
            _ => None,
        };
        return val;
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let illegal = Identifier::new(&Token::Illegal);

        let mut statement = LetStatement {
            token: self.current_token.clone(),
            name: Box::new(illegal.clone()),
            value: Box::new(illegal),
        };

        //if the next token is an identifier
        if let Token::Ident(_) = self.peek_token {
            // set the current token to the peek token
            self.next_token();
        } else {
            self.peek_error(Token::Ident(String::from("")));
            return None;
        }

        let statement_name = Identifier::new(&self.current_token);
        statement.name = Box::new(statement_name);

        if !self.expect_peek(Token::Equal) {
            return None;
        }

        loop {
            if self.current_token == Token::Semicolon {
                break;
            }
            self.next_token();
        }

        return Some(Box::new(statement));
    }

    fn peek_is(&mut self, token: Token) -> bool {
        return self.peek_token == token;
    }

    fn current_is(&mut self, token: Token) -> bool {
        return self.current_token == token;
    }

    fn peek_error(&mut self, token: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        );
        println!("{}", msg);
        self.errors.push(msg);
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_is(token.clone()) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token.clone());
            return false;
        }
    }
}
