use crate::{common::error::Result, lexer::token::Token};

use self::cursor::Cursor;

pub mod cursor;
pub mod error;
pub mod expression;
pub mod primitive;

#[macro_export]
macro_rules! group {
    ($name: ident<$lt: lifetime>: $($kind: ident$(<$lt_kind: lifetime>)?),*) => {

        #[derive(Debug, Clone)]
        pub enum $name<$lt> {
            $(
                $kind($kind$(<$lt_kind>)?)
            ),*
        }

        impl<'source> Parse<'source> for $name<'source> {
            fn parse<I: Iterator<Item = Token<'source>>>(cursor: &mut Cursor<'source, I>) -> Result<'source, Self> {
                let token = cursor.consume(&[$(
                    TokenKind::$kind
                ),*])?;

                Ok(match token.kind {
                    $(
                        TokenKind::$kind => Self::$kind($kind::from_slice(token.chunk.slice)),
                    )*
                    _ => unreachable!()
                })
            }
        }
    };

    ($name: ident: $($kind: ident),*) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $(
                $kind($kind)
            ),*
        }

        impl<'source> Parse<'source> for $name {
            fn parse<I: Iterator<Item = Token<'source>>>(cursor: &mut Cursor<'source, I>) -> Result<'source, Self> {
                let token = cursor.consume(&[$(
                    TokenKind::$kind
                ),*])?;

                Ok(match token.kind {
                    $(
                        TokenKind::$kind => Self::$kind($kind::from_slice(token.chunk.slice)),
                    )*
                    _ => unreachable!()
                })
            }
        }
    };
}

pub trait Parse<'source>: Sized {
    fn parse<I: Iterator<Item = Token<'source>> + Clone>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self>;
}
