use std::{marker::PhantomData, ops::Index};

use crate::{
    common::error::Result,
    lexer::token::{Token, TokenKind},
};

use super::{cursor::Cursor, primitive::Comma, Parse};

pub trait Stop<'source> {
    fn check<I: Index<usize, Output = Token<'source>>>(
        cursor: &Cursor<'source, I>,
    ) -> Result<'source, bool>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyStop;
impl<'source> Stop<'source> for EmptyStop {
    fn check<I: Index<usize, Output = Token<'source>>>(
        _: &Cursor<'source, I>,
    ) -> Result<'source, bool> {
        Ok(false)
    }
}

pub trait Separator<'source> {
    fn check<I: Index<usize, Output = Token<'source>>>(
        cursor: &Cursor<'source, I>,
    ) -> Result<'source, bool>;
    fn consume<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, ()>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuated<
    'source,
    T: Parse<'source>,
    S: Separator<'source>,
    P: Stop<'source> = EmptyStop,
> {
    pub elements: Vec<T>,
    _separator: PhantomData<S>,
    _stop: PhantomData<P>,
    _lifetime: PhantomData<&'source ()>,
}

impl<'source, T: Parse<'source>, S: Separator<'source>, P: Stop<'source>>
    Punctuated<'source, T, S, P>
{
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            elements,
            _separator: PhantomData,
            _lifetime: PhantomData,
            _stop: Default::default(),
        }
    }
}

impl<'source, T: Parse<'source>, S: Separator<'source>, P: Stop<'source>> Parse<'source>
    for Punctuated<'source, T, S, P>
{
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        let mut elements = vec![];

        while !cursor.is_eof() && !P::check(cursor)? {
            elements.push(cursor.parse()?);

            if !S::check(cursor)? {
                break;
            }

            S::consume(cursor)?;
        }
        Ok(Self::new(elements))
    }
}

impl<'source> Separator<'source> for Comma {
    fn check<I: Index<usize, Output = Token<'source>>>(
        cursor: &Cursor<'source, I>,
    ) -> Result<'source, bool> {
        cursor.test(&[TokenKind::Comma])
    }

    fn consume<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, ()> {
        cursor.next_token()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::{
        common::error::Result,
        lexer::token::{Token, TokenKind},
        parser::{
            cursor::Cursor,
            primitive::{Comma, Integer},
            punctuated::Stop,
        },
        tests,
    };

    use super::Punctuated;

    #[derive(Debug, Clone, PartialEq)]
    struct RightParenthesisStop;

    impl<'source> Stop<'source> for RightParenthesisStop {
        fn check<I: Index<usize, Output = Token<'source>>>(
            cursor: &Cursor<'source, I>,
        ) -> Result<'source, bool> {
            cursor.test(&[TokenKind::RightParenthesis])
        }
    }

    tests! {
        test_no_elements<Punctuated<Integer, Comma>>(""): Punctuated::new(vec![]);
        test_one_element<Punctuated<Integer, Comma>>("1"): Punctuated::new(vec![Integer("1")]);
        test_many_elements<Punctuated<Integer, Comma>>("1, 2, 3"): Punctuated::new(vec![Integer("1"), Integer("2"), Integer("3")]);
        test_custom_stop<Punctuated<Integer, Comma, RightParenthesisStop>>(")"): Punctuated::new(vec![]);
    }
}
