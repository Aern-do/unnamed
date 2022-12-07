use std::{fmt::Debug, iter::Peekable, str::Chars};

use crate::shared::span::Span;

use super::chunk::Chunk;
// Cursor iterating a string and producing slices of it
#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    input: Peekable<Chars<'a>>,
    raw: &'a str,
    end: usize,
    start: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            raw: input,
            start: Default::default(),
            end: Default::default(),
        }
    }
    pub fn next_char(&mut self) -> char {
        let char = self.input.next().unwrap();
        self.end += char.len_utf8();
        char
    }
    pub fn peek(&mut self) -> char {
        self.input.peek().copied().unwrap()
    }
    pub fn eof(&self) -> bool {
        self.raw.len() == self.end
    }
    pub fn reset(&mut self) {
        self.start = self.end;
    }
    pub fn span(&self) -> Span {
        Span::new(self.start, self.end)
    }
    pub fn slice(&self) -> &'a str {
        &self.raw[self.start..self.end]
    }
    pub fn chunk(&mut self) -> Chunk<'a> {
        let span = self.span();
        let slice = self.slice();
        self.reset();
        Chunk::new(slice, span)
    }
}
#[cfg(test)]
mod tests {
    use crate::shared::span::Span;

    use super::Cursor;

    #[test]
    fn basic() {
        let mut cursor = Cursor::new("123123");
        assert_eq!('1', cursor.next_char());
        assert_eq!('2', cursor.peek());
        assert_eq!('2', cursor.next_char());
        let chunk = cursor.chunk();
        assert_eq!(chunk.slice, "12");
        assert_eq!(chunk.span, Span::new(0, 2));
    }
    #[test]
    fn utf8() {
        let mut cursor = Cursor::new("1😎Ϩ");
        cursor.next_char();
        cursor.next_char();
        assert_eq!(cursor.span(), Span::new(0, 5));
        cursor.next_char();
        assert_eq!(cursor.span(), Span::new(0, 7));
    }
}
