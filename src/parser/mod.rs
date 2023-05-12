use std::ops::Index;

use crate::{common::error::Result, lexer::token::Token};

use self::{
    cursor::Cursor,
    delimited::Braced,
    expressions::Expression,
    primitive::{RightBrace, Semicolon},
    punctuated::Punctuated,
};

pub mod cursor;
pub mod delimited;
pub mod error;
pub mod expressions;
pub mod function;
pub mod primitive;
pub mod punctuated;

pub type Block<'source> =
    Braced<'source, Punctuated<'source, Expression<'source>, Semicolon, RightBrace>>;

pub trait SyntaxKind<'source> {
    fn test<I: Index<usize, Output = Token<'source>>>(cursor: &Cursor<'source, I>) -> bool;
}

pub trait Parse<'source>: Sized {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self>;
}

#[macro_export]
macro_rules! tests {
    ($($name: ident$(<$generic: ty>)?($input: literal): $expected: expr);+ $(;)?) => {
        use $crate::{lexer::{self, *}, parser::*};
        use std::{path::Path, result, fmt::Debug};

        fn compare<P: Parse<'static> + Debug + PartialEq>(recivied: P, expected: P) {
            assert_eq!(recivied, expected)
        }
        $(
            #[test]
            fn $name() {
                let cursor = lexer::cursor::Cursor::new($input, Path::new("test.u"));
                let lexer = Lexer::new(cursor);
                let tokens = lexer.collect::<result::Result<Vec<_>, _>>().unwrap();
                let mut cursor: Cursor<Vec<Token>> = Cursor::new(tokens.len(), tokens);
                let parsed = cursor.parse$(::<$generic>)?().unwrap();
                compare(parsed, $expected) // idk
            }
        )+

    };
}
