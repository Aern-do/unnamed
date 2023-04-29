use std::{
    fmt::{self, Debug, Display},
    ops::Add,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.start, rhs.end)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Chunk<'source> {
    pub span: Span,
    pub slice: &'source str,
}

impl<'source> Chunk<'source> {
    pub fn new(span: Span, slice: &'source str) -> Self {
        Self { span, slice }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub chunk: Chunk<'source>,
}

impl<'source> Token<'source> {
    pub fn new(kind: TokenKind, chunk: Chunk<'source>) -> Self {
        Self { kind, chunk }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Integer,
    Plus,
    Minus,
    Multiply,
    Division,
    LeftParenthesis,
    RightParenthesis,
}
impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Integer => write!(f, "Integer"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Multiply => write!(f, "Multiply"),
            TokenKind::Division => write!(f, "Division"),
            TokenKind::LeftParenthesis => write!(f, "Left parenthesis"),
            TokenKind::RightParenthesis => write!(f, "Right parenthesis"),
        }
    }
}

impl<'source> Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} at {}",
            self.kind, self.chunk.slice, self.chunk.span
        )
    }
}
