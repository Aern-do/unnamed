use std::{
    fmt::{self, Debug},
    str::Chars,
};

use itertools::{peek_nth, PeekNth};

use crate::shared::span::Span;

use super::chunk::Chunk;

// Cursor iterating a string and producing slices of it
#[derive(Clone)]
pub struct Cursor<'a> {
    input: PeekNth<Chars<'a>>,
    raw: &'a str,
    current: usize,
    prev: usize,
}
impl<'a> Debug for Cursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cursor")
            .field("raw", &self.raw)
            .field("current", &self.current)
            .field("prev", &self.prev)
            .finish()
    }
}
impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: peek_nth(input.chars()),
            raw: input,
            current: Default::default(),
            prev: Default::default(),
        }
    }
    pub fn next_char(&mut self) -> Option<char> {
        let char = self.input.next()?;
        self.current += char.len_utf8();
        Some(char)
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
    pub fn eof(&self) -> bool {
        self.raw.len() == self.current
    }
    pub fn lookup(&mut self, n: usize) -> Option<&char> {
        self.input.peek_nth(n)
    }
    pub fn reset(&mut self) {
        self.prev = self.current;
    }
    pub fn span(&self) -> Span {
        Span::new(self.prev, self.current)
    }
    pub fn slice(&self) -> &'a str {
        &self.raw[self.prev..self.current]
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
        assert_eq!(Some('1'), cursor.next_char());
        assert_eq!(Some(&'2'), cursor.peek());
        assert_eq!(Some('2'), cursor.next_char());
        let chunk = cursor.chunk();
        assert_eq!(chunk.slice, "12");
        assert_eq!(chunk.span, Span::new(0, 2));
    }
    #[test]
    fn utf8() {
        let mut cursor = Cursor::new("1😎Ϩ");
        cursor.next_char().unwrap();
        cursor.next_char().unwrap();
        assert_eq!(cursor.span(), Span::new(0, 5));
        cursor.next_char().unwrap();
        assert_eq!(cursor.span(), Span::new(0, 7));
    }
}
