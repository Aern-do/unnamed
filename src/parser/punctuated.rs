use std::{marker::PhantomData, ops::Index};

use crate::{common::error::Result, lexer::token::Token};

use super::{cursor::Cursor, primitive::Empty, Parse, SyntaxKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Punctuated<
    'source,
    T: Parse<'source>,
    S: SyntaxKind<'source> + Parse<'source>,
    P: SyntaxKind<'source> = Empty,
> {
    pub elements: Vec<T>,
    _separator: PhantomData<S>,
    _stop: PhantomData<P>,
    _lifetime: PhantomData<&'source ()>,
}

impl<
        'source,
        T: Parse<'source>,
        S: SyntaxKind<'source> + Parse<'source>,
        P: SyntaxKind<'source>,
    > Punctuated<'source, T, S, P>
{
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            elements,
            _separator: Default::default(),
            _lifetime: Default::default(),
            _stop: Default::default(),
        }
    }
}

impl<
        'source,
        T: Parse<'source>,
        S: SyntaxKind<'source> + Parse<'source>,
        P: SyntaxKind<'source>,
    > Parse<'source> for Punctuated<'source, T, S, P>
{
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        let mut elements = vec![];

        while !cursor.is_eof() && !P::test(cursor) {
            elements.push(cursor.parse()?);

            if !S::test(cursor) {
                break;
            }

            S::parse(cursor)?;
        }
        Ok(Self::new(elements))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::token::Token,
        parser::{
            cursor::Cursor,
            primitive::{Comma, Integer, RightParenthesis},
        },
        tests,
    };

    use super::Punctuated;

    tests! {
        test_no_elements<Punctuated<Integer, Comma>>(""): Punctuated::new(vec![]);
        test_one_element<Punctuated<Integer, Comma>>("1"): Punctuated::new(vec![Integer("1")]);
        test_many_elements<Punctuated<Integer, Comma>>("1, 2, 3"): Punctuated::new(vec![Integer("1"), Integer("2"), Integer("3")]);
        test_custom_stop<Punctuated<Integer, Comma, RightParenthesis>>(")"): Punctuated::new(vec![]);
    }
}
