use std::fmt::{self, Debug, Display};
use std::result;

use owo_colors::{OwoColorize, Stream};

use crate::lexer::{error::ErrorKind as LexerErrorKind, token::Chunk};
use crate::parser::error::ErrorKind as ParserErrorKind;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Lexer(LexerErrorKind),
    Parser(ParserErrorKind),
}

impl ErrorKind {
    pub fn is_eof(&self) -> bool {
        matches!(self, ErrorKind::Parser(ParserErrorKind::UnexpectedEof))
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Lexer(lexer_error_kind) => write!(f, "{lexer_error_kind}"),
            ErrorKind::Parser(parser_error_kind) => write!(f, "{parser_error_kind}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error<'source> {
    pub chunk: Option<Chunk<'source>>,
    pub kind: ErrorKind,
}

impl<'source> Error<'source> {
    pub fn new(kind: ErrorKind, chunk: Option<Chunk<'source>>) -> Self {
        Self { chunk, kind }
    }
}

impl<'source> Display for Error<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.chunk.is_none() {
            writeln!(
                f,
                "{}: {kind}",
                // 💀
                "[ERROR]"
                    .if_supports_color(Stream::Stdout, |text| text.bold())
                    .if_supports_color(Stream::Stdout, |text| text.red()),
                kind = self.kind.if_supports_color(Stream::Stdout, |text| text.red())
            )?;
            return Ok(());
        }

        writeln!(
            f,
            "{span}: {}: {kind}",
            // 💀
            "[ERROR]"
                .if_supports_color(Stream::Stdout, |text| text.bold())
                .if_supports_color(Stream::Stdout, |text| text.red()),
            span = self.chunk.unwrap().position,
            kind = self.kind.if_supports_color(Stream::Stdout, |text| text.red())
        )?;

        writeln!(
            f,
            "{} {slice}",
            "->".if_supports_color(Stream::Stdout, |text| text.bright_black()),
            slice = self.chunk.unwrap().slice
        )
    }
}

impl<'source> std::error::Error for Error<'source> {}

pub type Result<'source, T> = result::Result<T, Error<'source>>;
