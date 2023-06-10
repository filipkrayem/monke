pub mod lexer;
pub mod repl;

fn main() {
    println!("Monke REPL");
    println!("----------");

    repl::read().unwrap();
}
