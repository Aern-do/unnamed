use std::path::PathBuf;

use lexer::token::{Chunk, Position};
use parser::error::Error;

use crate::lexer::token::TokenKind;

pub mod lexer;
pub mod parser;

fn main() {
    let mut path_buf = PathBuf::new();
    path_buf.push("pososi");
    path_buf.push("posos.u");
    let error = Error {
        chunk: Some(Chunk {
            position: Position::new(0, 0, 0, 4, path_buf.as_path()),
            slice: "12 /",
        }),
        kind: parser::error::ErrorKind::UnexpectedToken {
            expected: &[
                TokenKind::Integer,
                TokenKind::LeftParenthesis,
                TokenKind::RightParenthesis,
            ],
            received: Some(TokenKind::Division),
        },
    };
    println!("{}", error);
}
