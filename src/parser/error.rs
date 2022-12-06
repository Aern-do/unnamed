use std::{
    error,
    fmt::{self, Display},
};

use crate::lexer::{chunk::Chunk, token::TokenKind};

#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub chunk: Option<Chunk<'a>>,
    pub kind: ErrorKind,
}

impl<'a> Error<'a> {
    pub fn new(kind: ErrorKind, chunk: Chunk<'a>) -> Self {
        Self {
            chunk: Some(chunk),
            kind,
        }
    }
    pub fn empty(kind: ErrorKind) -> Self {
        Self { chunk: None, kind }
    }
    pub fn optional(kind: ErrorKind, chunk: Option<Chunk<'a>>) -> Self {
        Self { kind, chunk }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    UnexpectedEof,
    UnexpectedToken {
        expected: &'static [TokenKind],
        received: Option<TokenKind>,
    },
}
impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::UnexpectedEof => write!(f, "Unexpected end of input"),
            ErrorKind::UnexpectedToken { expected, received } => write!(
                f,
                "Unexpected token, expected {:?} received {:?}",
                expected, received
            ),
        }
    }
}
impl<'a> error::Error for Error<'a> {}
