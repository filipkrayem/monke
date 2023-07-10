use std::collections::HashMap;

use crate::{
    ast::Program,
    expressions::Expression,
    identifier::Identifier,
    lexer::Lexer,
    statements::{ExpressionStatement, LetStatement, ReturnStatement, Statement},
    token::Token,
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,

    prefix_parse_fns: HashMap<Token, fn(&mut Parser<'a>) -> Option<Box<dyn Expression>>>,
    infix_parse_fns:
        HashMap<Token, fn(&mut Parser<'a>, Box<dyn Expression>) -> Option<Box<dyn Expression>>>,
}

enum Precedence {
    Lowest,
    Equals,      // ==
    Lessgreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // set both current_token and peek_token to actual values
        parser.next_token();
        parser.next_token();

        // register prefix parse functions
        // We are discarding the inner value of the Token::Ident variant, see token.rs impl Hash
        parser.register_prefix(Token::Ident(String::from("")), Parser::parse_identifier);
        return parser;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn register_prefix(
        &mut self,
        token: Token,
        func: fn(&mut Parser<'a>) -> Option<Box<dyn Expression>>,
    ) {
        self.prefix_parse_fns.insert(token, func);
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        let identifier = Identifier::new(&self.current_token);

        return Some(Box::new(identifier));
    }

    pub fn register_infix(
        &mut self,
        token: Token,
        func: fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>,
    ) {
        self.infix_parse_fns.insert(token, func);
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
            Token::Return => return self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
        return val;
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let illegal = Box::new(Identifier::new(&Token::Illegal));

        let mut statement = ExpressionStatement::new(self.current_token.clone(), illegal);

        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            statement.expression = expression;
        } else {
            return None;
        }

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        return Some(Box::new(statement));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self.prefix_parse_fns.get(&self.current_token);
        println!("{:?}", self.prefix_parse_fns);
        println!("current_token: {:?}", self.current_token);

        println!("prefix: {:?}", prefix);
        if prefix.is_none() {
            return None;
        }

        let left_exp = prefix.unwrap()(self);

        // while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
        //     let infix = self.infix_parse_fns.get(&self.peek_token);
        //
        //     if infix.is_none() {
        //         return left_exp;
        //     }
        //
        //     self.next_token();
        //
        //     left_exp = infix.unwrap()(self, left_exp.unwrap());
        // }

        return left_exp;
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

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let statement = ReturnStatement {
            token: self.current_token.clone(),
            return_value: Box::new(Identifier::new(&Token::Illegal)),
        };

        if !self.current_is(Token::Semicolon) {
            self.next_token();
        }

        return Some(Box::new(statement));
    }

    fn peek_is(&mut self, token: Token) -> bool {
        return self.peek_token == token;
    }

    fn peek_precdence(&mut self) -> Precedence {
        return Precedence::Lowest;
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

mod tests {
    use crate::{
        identifier::Identifier, lexer::Lexer, parser::Parser, statements::ExpressionStatement,
    };

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let mut l = Lexer::new(input.to_owned());
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();

        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements"
        );

        let stmt = &program.statements[0];
        if let Some(expression_statement) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
            if let Some(identifier) = expression_statement
                .expression
                .as_any()
                .downcast_ref::<Identifier>()
            {
                assert_eq!(identifier.value, "foobar");
                println!("Identifier: {:?}", identifier);
                // assert_eq!(identifier.token, "foobar");
            } else {
                panic!("Expression is not Identifier");
            }
        } else {
            panic!("Statement is not ExpressionStatement");
        }
    }

    fn check_parser_errors(p: &Parser) {
        let errors = &p.errors;

        if !errors.is_empty() {
            panic!("Parser errors: {:?}", errors);
        }
    }
}
