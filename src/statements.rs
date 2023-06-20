use crate::{
    expressions::Expression, identifier::Identifier, token::Token,
    utils::map_token_to_literal::map_token_to_literal,
};
use std::any::Any;

pub trait Statement {
    fn token_literal(&self) -> String;
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
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
}
