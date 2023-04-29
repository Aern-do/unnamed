use std::{
    error,
    fmt::{self, Display},
    result,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnexpectedToken,
    UnexpectedEof,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedToken => write!(f, "unexpected token"),
            Error::UnexpectedEof => write!(f, "unexpected eof"),
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
