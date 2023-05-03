use crate::{common::error::Result, lexer::token::Token};

use self::cursor::Cursor;

pub mod cursor;
pub mod error;
pub mod expression;
pub mod primitive;

pub trait Parse<'source>: Sized {
    fn parse<I: Iterator<Item = Token<'source>> + Clone>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self>;
}
