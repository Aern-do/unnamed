use std::{marker::PhantomData, ops::Index};

use crate::{common::error::Result, lexer::token::Token};

use super::{cursor::Cursor, Parse};

#[derive(Debug, Clone)]
pub struct Punctuated<'source, T: Parse<'source>, P: Parse<'source>> {
    pub elements: Vec<T>,
    separator: PhantomData<&'source P>,
}

impl<'source, T: Parse<'source>, P: Parse<'source>> Parse<'source> for Punctuated<'source, T, P> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        let mut elements = vec![];
        loop {
            elements.push(cursor.parse()?);
            if cursor.parse_without_consume::<P>().is_err() {
                break;
            }
            cursor.parse::<P>()?;
        }
        Ok(Self { elements, separator: Default::default() })
    }
}
