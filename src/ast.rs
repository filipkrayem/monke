use crate::{token::Token, utils::map_token_to_literal::map_token_to_literal};

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement {
    fn token_literal(&self) -> String;
    fn statement_node(&self);
}

trait Expression {
    fn token_literal(&self) -> String;
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> String {
        return map_token_to_literal(&self.token);
    }
}

pub struct Identifier {
    token: Token,
    value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    fn token_literal(&self) -> String {
        return map_token_to_literal(&self.token);
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
