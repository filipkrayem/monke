pub mod ast;
pub mod identifier;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;
pub mod utils;

fn main() {
    println!("Monke REPL");
    println!("----------");

    repl::read().unwrap();
}
