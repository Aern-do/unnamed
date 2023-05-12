use std::{marker::PhantomData, ops::Index};

use crate::{common::error::Result, lexer::token::Token};

use super::{
    cursor::Cursor,
    primitive::{LeftBraces, LeftParenthesis, RightBraces, RightParenthesis},
    Parse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delimited<'source, L: Parse<'source>, T: Parse<'source>, R: Parse<'source>> {
    _left_delimiter: PhantomData<L>,
    pub inner: T,
    _right_delimiter: PhantomData<R>,
    _lifetime: PhantomData<&'source ()>,
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
pub type Braced<'source, T> = Delimited<'source, LeftBraces, T, RightBraces>;

#[cfg(test)]
mod tests {
    use crate::{tests, parser::primitive::{Integer, LeftParenthesis, RightParenthesis}};

    use super::Delimited;

    tests! {
        test_delimited<Delimited<LeftParenthesis, Integer, RightParenthesis>>("(1)"): Delimited::new(Integer("1"));
    }
}