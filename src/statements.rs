use crate::{
    expressions::Expression, identifier::Identifier, token::Token,
    utils::map_token_to_literal::map_token_to_literal,
};
use std::any::Any;

pub trait Statement {
    fn token_literal(&self) -> String;
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
    fn string(&self) -> String;
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
    fn string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value.string()
        )
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> String {
        return map_token_to_literal(&self.token);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.return_value.string())
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> String {
        return map_token_to_literal(&self.token);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        self.expression.string()
    }
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> ExpressionStatement {
        return ExpressionStatement { token, expression };
    }
}
