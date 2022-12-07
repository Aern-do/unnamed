use std::{fmt::Debug, vec::IntoIter};

use crate::lexer::{self, token::Token, Lexer};

use self::{cursor::Cursor, error::Error};

pub mod ast;
pub mod cursor;
pub mod error;
pub mod expression;
pub mod program;
pub mod statement;
pub mod utils;
#[derive(Clone, Debug)]
pub struct Parser<'a, I: Iterator<Item = Token<'a>>> {
    cursor: Cursor<'a, I>,
}

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
    pub fn new(cursor: Cursor<'a, I>) -> Self {
        Self { cursor }
    }
}
pub fn test<
    'a,
    T: PartialEq + Eq + Debug,
    F: FnOnce(&mut Parser<'a, IntoIter<Token<'a>>>) -> Result<T, Error<'a>>,
>(
    input: &'a str,
    expected: T,
    get_node: F,
) {
    let cursor = lexer::cursor::Cursor::new(input);
    let lexer = Lexer::new(cursor);
    let tokens = lexer.collect::<Result<Vec<_>, _>>().unwrap();
    let cursor = Cursor::new(tokens.into_iter());
    let mut parser = Parser::new(cursor);
    let received = get_node(&mut parser).unwrap();
    assert_eq!(received, expected);
}
