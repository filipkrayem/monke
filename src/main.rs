pub mod ast;
pub mod lexer;
pub mod repl;
pub mod token;
pub mod utils;

fn main() {
    println!("Monke REPL");
    println!("----------");

    repl::read().unwrap();
}
