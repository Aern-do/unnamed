use std::iter::Peekable;

use crate::lexer::token::{Token, TokenKind};

use super::error::{Error, ErrorKind};

#[derive(Clone, Debug)]
pub struct Cursor<'a, I: Iterator<Item = Token<'a>>> {
    tokens: Peekable<I>,
}

impl<'a, I: Iterator<Item = Token<'a>>> Cursor<'a, I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, Error<'a>> {
        self.tokens
            .next()
            .ok_or_else(|| Error::empty(ErrorKind::UnexpectedEof))
    }
    pub fn peek(&mut self) -> Result<Token<'a>, Error<'a>> {
        self.tokens
            .peek()
            .copied()
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
#[cfg(test)]
mod tests {
    use std::vec::IntoIter;

    use crate::lexer::{
        self,
        token::{Token, TokenKind},
        Lexer,
    };

    use super::Cursor;

    fn cursor(input: &'static str) -> Cursor<'static, IntoIter<Token<'static>>> {
        let cursor = lexer::cursor::Cursor::new(input);
        let lexer = Lexer::new(cursor);
        let cursor = Cursor::new(lexer.collect::<Result<Vec<_>, _>>().unwrap().into_iter());
        cursor
    }
    #[test]
    fn next_token() {
        let mut cursor = cursor("12 34 78");
        assert_eq!(cursor.next_token().unwrap().slice(), "12");
        assert_eq!(cursor.next_token().unwrap().slice(), "34");
        assert_eq!(cursor.next_token().unwrap().slice(), "78");
    }
    #[test]
    fn peek() {
        let mut cursor = cursor("12 34 78");
        assert_eq!(cursor.next_token().unwrap().slice(), "12");
        assert_eq!(cursor.peek().unwrap().slice(), "34");
        assert_eq!(cursor.peek().unwrap().slice(), "34");
        assert_eq!(cursor.next_token().unwrap().slice(), "34");
    }
    #[test]
    fn eof() {
        let mut cursor = cursor("12 34 78");
        assert_eq!(cursor.next_token().unwrap().slice(), "12");
        assert_eq!(cursor.next_token().unwrap().slice(), "34");
        assert_eq!(cursor.next_token().unwrap().slice(), "78");
        assert!(cursor.eof());
    }
    #[test]
    fn test() {
        let mut cursor = cursor("abcd");
        assert!(cursor.test(&[TokenKind::Identifier]).unwrap());
    }
    #[test]
    #[should_panic]
    fn consume() {
        let mut cursor = cursor("abcd");
        cursor.consume(&[TokenKind::Integer]).unwrap();
    }
}
