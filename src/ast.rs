use crate::statements::Statement;

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return String::from("");
        }
    }
    pub fn string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            out.push_str(&statement.string());
        }
        return out;
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        expressions::Expression,
        identifier::Identifier,
        lexer::*,
        parser::Parser,
        statements::{LetStatement, Statement},
        token::Token,
    };

    use super::Program;

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

    #[test]
    fn return_statements() {
        let input = "
        return 5;
        return 420;
        return add(5);
        ";

        let mut lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        for statement in program.statements {
            assert_eq!(statement.token_literal(), "return");
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

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token::Let,
                name: Box::new(Identifier::new(&Token::Ident(String::from("myVar")))),
                value: Box::new(Identifier::new(&Token::Ident(String::from("anotherVar")))),
            })],
        };
        assert_eq!(program.string(), "let myVar = anotherVar;");
    }
}
