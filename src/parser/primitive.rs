use std::ops::Index;

use crate::{
    common::error::Result,
    lexer::token::{Token, TokenKind},
};

use super::{cursor::Cursor, Parse, SyntaxKind};

macro_rules! implement_primitive {
    ($($kind: ident),*) => {
        $(
            #[derive(Debug, Default, Clone, PartialEq, Eq)]
            pub struct $kind;
            impl<'source> Parse<'source> for $kind {
                fn parse<I: Index<usize, Output = Token<'source>>>(
                    cursor: &mut Cursor<'source, I>,
                ) -> Result<'source, Self> {
                    cursor.consume(&[TokenKind::$kind])?;
                    Ok($kind)
                }
            }

            impl<'source> SyntaxKind<'source> for $kind {
                fn test<I: Index<usize, Output = Token<'source>>>(
                    cursor: &Cursor<'source, I>,
                ) -> bool {
                    cursor.test(&[TokenKind::$kind]).unwrap_or_default()
                }
            }
        )*
    };
}

macro_rules! implement_primitive_inner {
        ($($kind: ident<$lt: lifetime>),*) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $kind<$lt>(pub &$lt str);

            impl<$lt> Parse<$lt> for $kind<$lt> {
                fn parse<I: Index<usize, Output = Token<$lt>>>(
                    cursor: &mut Cursor<$lt, I>,
                ) -> Result<$lt, Self> {
                    let token = cursor.consume(&[TokenKind::$kind])?;
                    Ok($kind(token.chunk.slice))
                }
            }

            impl<$lt> SyntaxKind<$lt> for $kind<$lt> {
                fn test<I: Index<usize, Output = Token<$lt>>>(
                    cursor: &Cursor<$lt, I>,
                ) -> bool {
                    cursor.test(&[TokenKind::$kind]).unwrap_or_default()
                }
            }
        )*
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Empty;
impl<'source> SyntaxKind<'source> for Empty {
    fn test<I: Index<usize, Output = Token<'source>>>(_: &Cursor<'source, I>) -> bool {
        false
    }
}

impl<'source, T: Parse<'source> + SyntaxKind<'source>> Parse<'source> for Option<T> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        if T::test(cursor) {
            Ok(Some(cursor.parse()?))
        } else {
            Ok(None)
        }
    }
}

impl<'source, T: Parse<'source>> Parse<'source> for Box<T> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        Ok(Box::new(cursor.parse()?))
    }
}

implement_primitive!(
    Plus,
    Minus,
    Multiply,
    Division,
    Less,
    LessEq,
    GreeterEq,
    Greeter,
    Eq,
    Assignment,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Semicolon,
    FuncKw,
    IfKw,
    ElseKw,
    WhileKw,
    ReturnKw,
    LetKw,
    MutKw
);
implement_primitive_inner!(Integer<'source>, Float<'source>, Identifier<'source>);
