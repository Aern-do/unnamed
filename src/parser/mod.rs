use self::cursor::Cursor;

pub mod ast;
pub mod cursor;
pub mod error;
pub mod expression;
pub mod program;
pub mod statement;
pub mod utils;
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(cursor: Cursor<'a>) -> Self {
        Self { cursor }
    }
}
