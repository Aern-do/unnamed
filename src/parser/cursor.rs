use std::{iter::Peekable, slice::Iter};

use crate::lexer::token::{Token, TokenKind};

use super::error::{Error, ErrorKind};

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    tokens: Peekable<Iter<'a, Token<'a>>>,
}

impl<'a> Cursor<'a> {
    pub fn new(tokens: Iter<'a, Token<'a>>) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, Error<'a>> {
        self.tokens
            .next()
            .copied()
            .ok_or_else(|| Error::empty(ErrorKind::UnexpectedEof))
    }
    pub fn peek(&mut self) -> Result<Token<'a>, Error<'a>> {
        self.tokens
            .peek()
            .map(|token| **token)
            .ok_or_else(|| Error::empty(ErrorKind::UnexpectedEof))
    }
    pub fn test(&mut self, expected: &'static [TokenKind]) -> Result<bool, Error<'a>> {
        match self.peek() {
            Ok(token) => Ok(expected.contains(&token.kind)),
            Err(err) if err.kind == ErrorKind::UnexpectedEof => Ok(false),
            Err(err) => Err(err),
        }
    }
    pub fn eof(&mut self) -> bool {
        matches!(self.peek(), Err(err) if err.kind == ErrorKind::UnexpectedEof)
    }
    pub fn test_and_next(&mut self, expected: &'static [TokenKind]) -> Result<bool, Error<'a>> {
        if self.test(expected)? {
            self.consume(expected)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn consume(&mut self, expected: &'static [TokenKind]) -> Result<Token<'a>, Error<'a>> {
        if self.test(expected)? {
            self.next_token()
        } else {
            let (kind, chunk) = if self.eof() {
                (None, None)
            } else {
                let token = self.peek()?;
                (Some(token.kind), Some(token.chunk))
            };
            Err(Error::optional(
                ErrorKind::UnexpectedToken {
                    expected,
                    received: kind,
                },
                chunk,
            ))
        }
    }
}
