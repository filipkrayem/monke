use crate::expressions::Expression;
use crate::token::Literal;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: &Token) -> Identifier {
        return Identifier {
            token: token.clone(),
            value: token.literal(),
        };
    }
}

impl Expression for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal();
    }
    fn expression_node(&self) {}
}
