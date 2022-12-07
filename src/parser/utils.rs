use crate::lexer::token::{Token, TokenKind};

use super::{error::Error, Parser};

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
    pub fn parenthesized<T, F: FnOnce(&mut Self) -> Result<T, Error<'a>>>(
        &mut self,
        parse: F,
    ) -> Result<T, Error<'a>> {
        self.cursor.consume(&[TokenKind::LeftParenthesis])?;
        let parsed = parse(self)?;
        self.cursor.consume(&[TokenKind::RightParenthesis])?;
        Ok(parsed)
    }
    pub fn arguments<T, F: Fn(&mut Parser<'a, I>) -> Result<T, Error<'a>>>(
        &mut self,
        parse: F,
    ) -> Result<Vec<T>, Error<'a>> {
        let mut elements = vec![];
        loop {
            if self.cursor.test(&[TokenKind::RightParenthesis])? {
                break;
            }
            elements.push(parse(self)?);
            if !{
                if self.cursor.test(&[TokenKind::Comma])? {
                    self.cursor.next_token()?;
                    true
                } else {
                    false
                }
            } {
                break;
            }
        }
        Ok(elements)
    }
    pub fn optional_semicolon<T, F: FnOnce(&mut Self) -> Result<T, Error<'a>>>(
        &mut self,
        parse: F,
    ) -> Result<T, Error<'a>> {
        let result = parse(self)?;
        self.cursor.test_and_next(&[TokenKind::Semicolon])?;
        Ok(result)
    }
    pub fn semicolon<T, F: FnOnce(&mut Self) -> Result<T, Error<'a>>>(
        &mut self,
        parse: F,
    ) -> Result<T, Error<'a>> {
        let result = parse(self)?;
        self.cursor.consume(&[TokenKind::Semicolon])?;
        Ok(result)
    }
}
