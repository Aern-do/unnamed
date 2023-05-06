use std::ops::Index;

use crate::{
    common::error::Result,
    lexer::token::{Token, TokenKind},
};

use super::{cursor::Cursor, Parse};

macro_rules! implement_primitive {
    ($($kind: ident),*) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $kind;
            impl<'source> Parse<'source> for $kind {
                fn parse<I: Index<usize, Output = Token<'source>>>(
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

macro_rules! implement_primitive_inner {
    ($($kind: ident<$lt: lifetime>),*) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $kind<$lt>(pub &$lt str);

            impl<$lt> Parse<$lt> for $kind<$lt> {
                fn parse<I: Index<usize, Output = Token<$lt>>>(
                    cursor: &mut Cursor<$lt, I>,
                ) -> Result<$lt, Self> {
                    let token = cursor.consume(&[TokenKind::$kind])?;
                    Ok($kind(token.chunk.slice))
                }
            }
        )*
    };
}


implement_primitive!(Plus, Minus, Multiply, Division, LeftParenthesis, RightParenthesis, Comma);
implement_primitive_inner!(Integer<'source>, Float<'source>, Identifier<'source>);

#[cfg(test)]
mod tests {
    use crate::{tests};

    use super::{Plus, Minus, Multiply, Division, Integer, Float, Comma, LeftParenthesis, RightParenthesis, Identifier};

    tests! {
        test_plus("+"): Plus;
        test_minus("-"): Minus;
        test_multiply("*"): Multiply;
        test_division("/"): Division;
        test_left_parenthesis("("): LeftParenthesis;
        test_right_parenthesis(")"): RightParenthesis;
        test_comma(","): Comma;
        test_integer("3"): Integer("3");
        test_float("3.14"): Float("3.14");
        test_identifier("test"): Identifier("test")
    }
}