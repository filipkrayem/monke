use std::any::Any;

pub trait Expression {
    fn token_literal(&self) -> String;
    fn expression_node(&self);
    fn string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl std::fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}
