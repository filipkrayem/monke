use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::lexer;

const PROMPT: &str = ">>> ";

pub fn read() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(PROMPT);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

                let mut lexer = lexer::Lexer::new(line);

                let tokens = lexer.all_tokens();
                println!("Tokens: {:#?}", tokens);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    return Ok(());
}
