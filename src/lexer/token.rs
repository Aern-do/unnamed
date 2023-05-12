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
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Integer => write!(f, "integer"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Plus => write!(f, "plus"),
            TokenKind::Minus => write!(f, "minus"),
            TokenKind::Multiply => write!(f, "multiply"),
            TokenKind::Division => write!(f, "division"),
            TokenKind::LeftParenthesis => write!(f, "left parenthesis"),
            TokenKind::RightParenthesis => write!(f, "right parenthesis"),
            TokenKind::Comma => write!(f, "comma"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Colon => write!(f, "colon"),
            TokenKind::Semicolon => write!(f, "semicolon"),
            TokenKind::FuncKw => write!(f, "func"),
            TokenKind::LeftBrace => write!(f, "left brace"),
            TokenKind::RightBrace => write!(f, "right brace"),
            TokenKind::IfKw => write!(f, "if"),
            TokenKind::ElseKw => write!(f, "else"),
            TokenKind::WhileKw => write!(f, "while"),
            TokenKind::Less => write!(f, "less"),
            TokenKind::Greeter => write!(f, "greeter"),
            TokenKind::LessEq => write!(f, "less or equal"),
            TokenKind::GreeterEq => write!(f, "greeter or eq"),
            TokenKind::Eq => write!(f, "eq"),
            TokenKind::Assignment => write!(f, "assignment"),
        }
    }
}

impl<'source> Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} at {}", self.kind, self.chunk.slice, self.chunk.position)
    }
}
