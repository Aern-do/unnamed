use super::chunk::Chunk;

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub chunk: Chunk<'a>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, chunk: Chunk<'a>) -> Self {
        Self { kind, chunk }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Integer,
    Float,
    Identifier,

    Function,
    If,
    Else,
    While,
    Let,
    Mut,
    Return,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    Add,
    Sub,
    Mul,
    Div,
    Assignment,
    Equal,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Not,
    NotEq,
    And,
    Or,

    Comma,
    Arrow,
    Semicolon,
    Colon,
}
