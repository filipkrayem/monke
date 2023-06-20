use crate::{
    identifier::Identifier, token::Token, utils::map_token_to_literal::map_token_to_literal,
};
use std::any::Any;

trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement {
    fn token_literal(&self) -> String;
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression {
    fn token_literal(&self) -> String;
    fn expression_node(&self);
}

impl std::fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

impl std::fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> String {
        return map_token_to_literal(&self.token);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return String::from("");
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{ast::Expression, ast::LetStatement, lexer::*, parser::Parser};

    use super::Statement;

    #[test]
    fn let_statements() {
        let input = "
        let x = 5;
        let y = 420;
        let z = 69;
       ";

        let mut lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();

        let tests = vec!["x", "y", "z"];

        for (i, test) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            if !test_let_statement(statement, test) {
                return;
            }
        }
    }

    fn test_let_statement(statement: &Box<dyn Statement>, name: &str) -> bool {
        assert_eq!(statement.token_literal(), "let");

        if let Some(let_statement) = statement.as_any().downcast_ref::<LetStatement>() {
            assert_eq!(let_statement.name.value, name);

            assert_eq!(let_statement.name.token_literal(), name);
        } else {
            panic!("statement is not a LetStatement");
        }

        return true;
    }
}
