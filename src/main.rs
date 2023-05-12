use std::{path::Path, result};

use lexer::Lexer;
use rustyline::{error::ReadlineError, DefaultEditor, Result};

use crate::parser::expressions::Expression;

pub mod common;
pub mod lexer;
pub mod parser;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                let cursor = lexer::cursor::Cursor::new(&line, Path::new("repl"));
                let lexer = Lexer::new(cursor);

                let tokens = match lexer.collect::<result::Result<Vec<_>, _>>() {
                    Ok(tokens) => tokens,
                    Err(err) => {
                        eprintln!("{err}");
                        continue;
                    }
                };

                let mut cursor = parser::cursor::Cursor::new(tokens.len(), tokens);
                let expression = match cursor.parse::<Expression>() {
                    Ok(expression) => expression,
                    Err(err) => {
                        eprintln!("{err}");
                        continue;
                    }
                };

                println!("{:#?}", expression);
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
        };
    }
    Ok(())
}
