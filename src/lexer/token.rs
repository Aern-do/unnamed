use std::{
    cmp::max,
    fmt::{self, Debug, Display},
    ops::Add,
    path::Path,
};

#[derive(Clone, Copy, Debug, Eq)]
pub struct Position<'source> {
    start: usize,
    end: usize,
    line: usize,
    column: usize,
    path: &'source Path,
}

impl<'source> PartialEq for Position<'source> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl<'source> Position<'source> {
    pub fn new(start: usize, end: usize, line: usize, column: usize, path: &'source Path) -> Self {
        Self { start, end, line, column, path }
    }
}

impl<'source> Add for Position<'source> {
    type Output = Position<'source>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.start,
            rhs.end,
            max(self.line, rhs.line),
            max(self.column, rhs.column),
            self.path,
        )
    }
}

impl<'source> Display for Position<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.path.display(), self.line + 1, self.column)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chunk<'source> {
    pub position: Position<'source>,
    pub slice: &'source str,
}

impl<'source> Chunk<'source> {
    pub fn new(span: Position<'source>, slice: &'source str) -> Self {
        Self { position: span, slice }
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
    Float,
    Identifier,

    Plus,
    Minus,
    Multiply,
    Division,
    Less,
    Greeter,
    LessEq,
    GreeterEq,
    Eq,
    Assignment,

    Comma,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,

    FuncKw,
    IfKw,
    ElseKw,
    WhileKw,
    LetKw,
    MutKw,
    ReturnKw,
    TrueKw,
    FalseKw
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Integer => write!(f, "{{integer}}"),
            TokenKind::Float => write!(f, "{{float}}"),
            TokenKind::Plus => write!(f, "'+'"),
            TokenKind::Minus => write!(f, "'-'"),
            TokenKind::Multiply => write!(f, "'*'"),
            TokenKind::Division => write!(f, "'/'"),
            TokenKind::LeftParenthesis => write!(f, "'('"),
            TokenKind::RightParenthesis => write!(f, "')'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Identifier => write!(f, "{{identifier}}"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::Semicolon => write!(f, "';'"),
            TokenKind::FuncKw => write!(f, "'func'"),
            TokenKind::LeftBrace => write!(f, "'{{'"),
            TokenKind::RightBrace => write!(f, "'}}'"),
            TokenKind::IfKw => write!(f, "'if'"),
            TokenKind::ElseKw => write!(f, "'else'"),
            TokenKind::WhileKw => write!(f, "'while'"),
            TokenKind::Less => write!(f, "'<'"),
            TokenKind::Greeter => write!(f, "'>'"),
            TokenKind::LessEq => write!(f, "'<='"),
            TokenKind::GreeterEq => write!(f, "'>='"),
            TokenKind::Eq => write!(f, "'=='"),
            TokenKind::Assignment => write!(f, "'='"),
            TokenKind::LetKw => write!(f, "'let'"),
            TokenKind::MutKw => write!(f, "'mut'"),
            TokenKind::ReturnKw => write!(f, "'return'"),
            TokenKind::TrueKw => write!(f, "'true'"),
            TokenKind::FalseKw => write!(f, "'false'"),
        }
    }
}

impl<'source> Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} at {}", self.kind, self.chunk.slice, self.chunk.position)
    }
}
