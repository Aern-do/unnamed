use std::fmt::{self, Display};

use owo_colors::{OwoColorize, Stream};

use crate::lexer::token::{Chunk, TokenKind};

pub struct Error<'source> {
    pub chunk: Option<Chunk<'source>>,
    pub kind: ErrorKind,
}

impl<'source> Error<'source> {
    pub fn new(kind: ErrorKind, chunk: Option<Chunk<'source>>) -> Self {
        Self { chunk, kind }
    }
}

pub enum ErrorKind {
    UnexpectedEof,
    UnexpectedToken {
        expected: &'static [TokenKind],
        received: Option<TokenKind>,
    },
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

impl<'source> Display for Error<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let None = self.chunk {
            writeln!(
                f,
                "{}: {kind}",
                // 💀
                "[ERROR]"
                    .if_supports_color(Stream::Stdout, |text| text.bold())
                    .if_supports_color(Stream::Stdout, |text| text.red()),
                kind = self
                    .kind
                    .if_supports_color(Stream::Stdout, |text| text.red())
            )?;
            return Ok(());
        }

        writeln!(
            f,
            "{span}: {}: {kind}",
            "[ERROR]"
                .if_supports_color(Stream::Stdout, |text| text.bold())
                .if_supports_color(Stream::Stdout, |text| text.red()),
            span = self.chunk.unwrap().position,
            kind = self
                .kind
                .if_supports_color(Stream::Stdout, |text| text.red())
        )?;

        writeln!(
            f,
            "{} {slice}",
            "->".if_supports_color(Stream::Stdout, |text| text.bright_black()),
            slice = self.chunk.unwrap().slice
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::lexer::token::{Chunk, Position, TokenKind};

    use super::{Error, ErrorKind};
    use owo_colors::set_override;

    fn test(error: Error<'static>, output: &'static str) {
        set_override(false);
        assert_eq!(error.to_string(), output);
    }

    #[test]
    fn test_unexpected_eof() {
        test(
            Error::new(ErrorKind::UnexpectedEof, None),
            "[ERROR]: unexpected end of file\n",
        )
    }

    #[test]
    fn test_unexpected_token_one() {
        test(
            Error::new(
                ErrorKind::UnexpectedToken {
                    expected: &[TokenKind::Plus],
                    received: Some(TokenKind::Minus),
                },
                Some(Chunk::new(
                    Position::new(0, 1, 0, 1, Path::new("test.u")),
                    "-",
                )),
            ),
            "test.u:1:1: [ERROR]: expected plus, received minus\n-> -\n",
        )
    }

    #[test]
    fn test_unexpected_token_two() {
        test(
            Error::new(
                ErrorKind::UnexpectedToken {
                    expected: &[TokenKind::Plus, TokenKind::Multiply],
                    received: Some(TokenKind::Minus),
                },
                Some(Chunk::new(
                    Position::new(0, 1, 0, 1, Path::new("test.u")),
                    "-",
                )),
            ),
            "test.u:1:1: [ERROR]: expected plus or multiply, received minus\n-> -\n",
        )
    }

    #[test]
    fn test_unexpected_token_there() {
        test(
            Error::new(
                ErrorKind::UnexpectedToken {
                    expected: &[TokenKind::Plus, TokenKind::Multiply, TokenKind::Division],
                    received: Some(TokenKind::Minus),
                },
                Some(Chunk::new(
                    Position::new(0, 1, 0, 1, Path::new("test.u")),
                    "-",
                )),
            ),
            "test.u:1:1: [ERROR]: expected plus, multiply or division, received minus\n-> -\n",
        )
    }

    #[test]
    fn test_unexpected_token_eof() {
        test(
            Error::new(
                ErrorKind::UnexpectedToken {
                    expected: &[TokenKind::Plus, TokenKind::Multiply, TokenKind::Division],
                    received: None,
                },
                None,
            ),
            "[ERROR]: expected plus, multiply or division, received EOF\n",
        )
    }
}
