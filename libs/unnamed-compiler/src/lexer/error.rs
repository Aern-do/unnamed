use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    UnexpectedToken,
    TooManyFloatingPoints(u8),
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UnexpectedToken => write!(f, "unexpected token"),
            ErrorKind::TooManyFloatingPoints(points) => {
                write!(f, "Too many decimal points, expected 1, received {points}")
            }
        }
    }
}
