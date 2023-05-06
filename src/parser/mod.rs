use std::ops::Index;

use crate::{common::error::Result, lexer::token::Token};

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
