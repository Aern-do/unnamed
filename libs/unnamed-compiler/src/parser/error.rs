use std::fmt::{self, Display};

use crate::lexer::token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    UnexpectedEof,
    UnexpectedToken { expected: &'static [TokenKind], received: Option<TokenKind> },
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UnexpectedEof => write!(f, "unexpected end of file"),
            ErrorKind::UnexpectedToken { expected, received } => {
                write!(f, "expected ")?;
                // Nano#2724 ❤️
                match *expected {
                    [] => panic!("array shouldn't be empty"),
                    [expected_kind] => write!(f, "{expected_kind}, ")?,
                    [expected_kinds @ .., last_expected_kind] => {
                        for (index, expected_kind) in expected_kinds.iter().enumerate() {
                            write!(f, "{expected_kind}")?;
                            if index != expected_kinds.len() - 1 {
                                write!(f, ", ")?;
                            }
                        }

                        write!(f, " or {}, ", last_expected_kind)?;
                    }
                }

                write!(f, "received ")?;
                match received {
                    Some(received_kind) => write!(f, "{received_kind}"),
                    None => write!(f, "EOF"),
                }
            }
        }
    }
}
