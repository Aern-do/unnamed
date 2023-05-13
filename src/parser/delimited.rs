use std::{fmt::Debug, marker::PhantomData, ops::Index};

use crate::{common::error::Result, lexer::token::Token};

use super::{
    cursor::Cursor,
    primitive::{LeftBrace, LeftParenthesis, RightBrace, RightParenthesis},
    Parse,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Delimited<'source, L: Parse<'source>, T: Parse<'source>, R: Parse<'source>> {
    _left_delimiter: PhantomData<L>,
    pub inner: T,
    _right_delimiter: PhantomData<R>,
    _lifetime: PhantomData<&'source ()>,
}

impl<'soure, L: Parse<'soure>, T: Parse<'soure> + Debug, R: Parse<'soure>> Debug
    for Delimited<'soure, L, T, R>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Delimited").field("inner", &self.inner).finish()
    }
}

impl<'source, L: Parse<'source>, T: Parse<'source>, R: Parse<'source>> Delimited<'source, L, T, R> {
    pub fn new(inner: T) -> Self {
        Self {
            _left_delimiter: Default::default(),
            inner,
            _right_delimiter: Default::default(),
            _lifetime: Default::default(),
        }
    }
}

impl<'source, L: Parse<'source>, T: Parse<'source>, R: Parse<'source>> Parse<'source>
    for Delimited<'source, L, T, R>
{
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        cursor.parse::<L>()?;

        let inner = cursor.parse()?;

        cursor.parse::<R>()?;

        Ok(Self {
            _left_delimiter: Default::default(),
            inner,
            _right_delimiter: Default::default(),
            _lifetime: Default::default(),
        })
    }
}

pub type Parenthesized<'source, T> = Delimited<'source, LeftParenthesis, T, RightParenthesis>;
pub type Braced<'source, T> = Delimited<'source, LeftBrace, T, RightBrace>;

#[cfg(test)]
mod tests {
    use crate::{
        parser::primitive::{Integer, LeftParenthesis, RightParenthesis},
        tests,
    };

    use super::Delimited;

    tests! {
        test_delimited<Delimited<LeftParenthesis, Integer, RightParenthesis>>("(1)"): Delimited::new(Integer("1"));
    }
}
