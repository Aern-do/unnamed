use std::{
    error,
    fmt::{self, Display},
};

use crate::shared::span::Span;

#[derive(Clone, Debug)]
pub struct Error {
    span: Span,
    kind: ErrorKind,
}

impl Error {
    pub fn new(span: Span, kind: ErrorKind) -> Self {
        Self { span, kind }
    }
    pub fn empty(kind: ErrorKind) -> Self {
        Self {
            span: Span::new(0, 0),
            kind,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorKind {
    UnexpectedToken,
    UnexpectedEof,
    TooManyFloatingPoints,
}
impl Display for Error {
    // TODO: Implement pretty formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::UnexpectedToken => write!(
                f,
                "Unexpected token at {}..{}",
                self.span.start, self.span.end
            ),
            ErrorKind::UnexpectedEof => write!(f, "Unexpected end of input"),
            ErrorKind::TooManyFloatingPoints => write!(f, "Too many floating points"),
        }
    }
}
impl error::Error for Error {}
