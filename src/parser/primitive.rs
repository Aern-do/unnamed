use crate::{
    common::error::Result,
    lexer::token::{Token, TokenKind},
};

use super::{cursor::Cursor, Parse};

macro_rules! implement_primitive {
    ($($kind: ident),*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $kind;
            impl<'source> Parse<'source> for $kind {
                fn parse<I: Iterator<Item = Token<'source>>>(
                    cursor: &mut Cursor<'source, I>,
                ) -> Result<'source, Self> {
                    cursor.consume(&[TokenKind::$kind])?;
                    Ok($kind)
                }
            }
            impl $kind {
                pub fn from_slice<'source>(_: &'source str) -> Self {
                    Self
                }
            }
        )*
    };
}

implement_primitive!(Plus, Minus, Multiply, Division, LeftParenthesis, RightParenthesis);

#[derive(Debug, Clone)]
pub struct Integer<'source>(pub &'source str);

impl<'source> Integer<'source> {
    pub fn from_slice(slice: &'source str) -> Self {
        Self(slice)
    }
}

impl<'source> Parse<'source> for Integer<'source> {
    fn parse<I: Iterator<Item = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        Ok(Self(cursor.consume(&[TokenKind::Integer])?.chunk.slice))
    }
}
