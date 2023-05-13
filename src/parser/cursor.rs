use std::ops::Index;

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
pub struct Cursor<'source, I: Index<usize, Output = Token<'source>>> {
    tokens: I,
    len: usize,
    position: usize,
}

impl<'source, I: Index<usize, Output = Token<'source>>> Cursor<'source, I> {
    pub fn new(len: usize, tokens: I) -> Self {
        Self { tokens, position: Default::default(), len }
    }

    pub fn move_cursor(&mut self, position: usize) {
        self.position = position;
    }

    pub fn increment_cursor(&mut self) {
        self.position += 1;
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.len
    }

    pub fn next_token(&mut self) -> Result<'source, Token<'source>> {
        if self.position >= self.len {
            return Err(Error::new(CommonErrorKind::Parser(ErrorKind::UnexpectedEof), None));
        }
        let token = self.tokens[self.position];
        self.increment_cursor();
        Ok(token)
    }

    pub fn peek(&self) -> Result<'source, Token<'source>> {
        if self.position >= self.len {
            return Err(Error::new(CommonErrorKind::Parser(ErrorKind::UnexpectedEof), None));
        }
        Ok(self.tokens[self.position])
    }

    pub fn test(&self, expected: &'static [TokenKind]) -> Result<'source, bool> {
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

    pub fn check(&mut self, expected: &'static [TokenKind]) -> Result<'source, Token<'source>> {
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

    pub fn parse<P: Parse<'source>>(&mut self) -> Result<'source, P> {
        P::parse(self)
    }

    pub fn parse_without_consume<P: Parse<'source>>(&mut self) -> Result<'source, P> {
        let previous_position = self.position;
        let parsed = P::parse(self)?;
        self.move_cursor(previous_position);
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Index, path::Path};

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

    fn create_cursor(tokens: Vec<Token>) -> Cursor<impl Index<usize, Output = Token>> {
        Cursor::new(tokens.len(), tokens)
    }

    #[test]
    fn test_next_token() {
        let mut cursor = create_cursor(vec![create_token(TokenKind::Plus)]);
        assert_eq!(cursor.next_token().unwrap().kind, TokenKind::Plus);
    }

    #[test]
    fn test_peek() {
        let cursor = create_cursor(vec![create_token(TokenKind::Plus)]);
        assert_eq!(cursor.peek().unwrap().kind, TokenKind::Plus);
    }

    #[test]
    fn test_cursor_test() {
        let cursor = create_cursor(vec![create_token(TokenKind::Plus)]);
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
