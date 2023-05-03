use std::iter::Peekable;

use crate::{
    common::{
        error::{Error, Result},
        CommonErrorKind,
    },
    lexer::token::{Token, TokenKind},
    parser::error::ErrorKind,
};

use super::Parse;

#[derive(Debug, Clone)]
pub struct Cursor<'source, I: Iterator<Item = Token<'source>>> {
    tokens: Peekable<I>,
}

impl<'source, I: Iterator<Item = Token<'source>>> Cursor<'source, I> {
    pub fn new(iter: I) -> Self {
        Self { tokens: iter.peekable() }
    }

    pub fn next_token(&mut self) -> Result<'source, Token<'source>> {
        self.tokens
            .next()
            .ok_or_else(|| Error::new(CommonErrorKind::Parser(ErrorKind::UnexpectedEof), None))
    }

    pub fn peek(&mut self) -> Result<'source, Token<'source>> {
        self.tokens
            .peek()
            .copied()
            .ok_or_else(|| Error::new(CommonErrorKind::Parser(ErrorKind::UnexpectedEof), None))
    }

    pub fn test(&mut self, expected: &'static [TokenKind]) -> Result<'source, bool> {
        match self.peek() {
            Ok(token) if expected.contains(&token.kind) => Ok(true),
            Ok(..) => Ok(false),

            Err(err) if err.kind == CommonErrorKind::Parser(ErrorKind::UnexpectedEof) => Ok(false),
            Err(err) => Err(err),
        }
    }

    pub fn consume(&mut self, expected: &'static [TokenKind]) -> Result<'source, Token<'source>> {
        if self.test(expected)? {
            self.next_token()
        } else {
            let token = self.peek()?;

            Err(Error::new(
                CommonErrorKind::Parser(ErrorKind::UnexpectedToken {
                    expected,
                    received: Some(token.kind),
                }),
                Some(token.chunk),
            ))
        }
    }

    pub fn test_and_return(&mut self, expected: &'static [TokenKind]) -> Result<'source, Token<'source>> {
        if self.test(expected)? {
            self.peek()
        } else {
            let token = self.peek()?;

            Err(Error::new(
                CommonErrorKind::Parser(ErrorKind::UnexpectedToken {
                    expected,
                    received: Some(token.kind),
                }),
                Some(token.chunk),
            ))
        }
    }

    pub fn parse<P: Parse<'source>>(&mut self) -> Result<'source, P>
    where
        I: Clone,
    {
        P::parse(self)
    }

    pub fn parse_without_consume<P: Parse<'source>>(&mut self) -> Result<'source, P>
    where
        I: Clone,
    {
        P::parse(&mut self.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        lexer::token::{Chunk, Position, Token, TokenKind},
        parser::cursor::Cursor,
    };

    fn create_token(kind: TokenKind) -> Token<'static> {
        Token {
            kind,
            chunk: Chunk { position: Position::new(0, 1, 1, 1, &Path::new("test.u")), slice: "+" },
        }
    }

    fn create_cursor(tokens: Vec<Token>) -> Cursor<impl Iterator<Item = Token>> {
        Cursor::new(tokens.into_iter())
    }

    #[test]
    fn test_next_token() {
        let mut cursor = create_cursor(vec![create_token(TokenKind::Plus)]);
        assert_eq!(cursor.next_token().unwrap().kind, TokenKind::Plus);
    }

    #[test]
    fn test_peek() {
        let mut cursor = create_cursor(vec![create_token(TokenKind::Plus)]);
        assert_eq!(cursor.peek().unwrap().kind, TokenKind::Plus);
    }

    #[test]
    fn test_test() {
        let mut cursor = create_cursor(vec![create_token(TokenKind::Plus)]);
        assert_eq!(cursor.test(&[TokenKind::Plus]).unwrap(), true);
        assert_eq!(cursor.test(&[TokenKind::Integer]).unwrap(), false);
    }

    #[test]
    fn test_consume() {
        let mut cursor = create_cursor(vec![create_token(TokenKind::Plus)]);
        assert!(cursor.consume(&[TokenKind::Plus]).is_ok());
        assert!(cursor.consume(&[TokenKind::Integer]).is_err());
    }

    #[test]
    fn test_unexpected_eof() {
        let mut cursor = create_cursor(vec![]);
        assert!(cursor.next_token().is_err());
        assert!(cursor.peek().is_err());
    }
}
