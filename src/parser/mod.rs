use std::{fmt::Debug, ops::Index, path::Path, result};

use crate::{
    common::error::Result,
    lexer::{self, token::Token, Lexer},
};

use self::cursor::Cursor;

pub mod cursor;
pub mod error;
pub mod expression;
pub mod primitive;
pub mod punctuated;

pub trait Parse<'source>: Sized {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self>;
}

pub fn test<P: Parse<'static> + Debug + PartialEq>(input: &'static str, expected: P) {
    let cursor = lexer::cursor::Cursor::new(input, Path::new("test.u"));
    let lexer = Lexer::new(cursor);
    let tokens = lexer.collect::<result::Result<Vec<_>, _>>().unwrap();
    let mut cursor: Cursor<Vec<Token>> = Cursor::new(tokens.len(), tokens);
    assert_eq!(cursor.parse::<P>().unwrap(), expected);
}
