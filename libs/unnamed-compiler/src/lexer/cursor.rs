use std::{iter::Peekable, path::Path, str::Chars};

use super::token::{Chunk, Position};

#[derive(Clone, Debug)]
pub struct Cursor<'source> {
    input: Peekable<Chars<'source>>,
    path: &'source Path,
    raw: &'source str,
    end: usize,
    start: usize,
    line: usize,
    column: usize,
}

impl<'source> Cursor<'source> {
    pub fn new(input: &'source str, path: &'source Path) -> Self {
        Self {
            input: input.chars().peekable(),
            raw: input,
            path,
            start: Default::default(),
            end: Default::default(),
            line: Default::default(),
            column: Default::default(),
        }
    }

    pub fn next_char(&mut self) -> char {
        let char = self.input.next().unwrap();
        self.column += 1;
        if char == '\n' {
            self.line += 1;
            self.column = 0;
        }
        self.end += char.len_utf8();

        char
    }

    pub fn peek(&mut self) -> char {
        self.input.peek().copied().unwrap()
    }

    pub fn is_eof(&self) -> bool {
        self.raw.len() == self.end
    }

    pub fn reset(&mut self) {
        self.start = self.end;
    }

    pub fn span(&self) -> Position<'source> {
        Position::new(self.start, self.end, self.line, self.column, self.path)
    }

    pub fn slice(&self) -> &'source str {
        &self.raw[self.start..self.end]
    }

    pub fn chunk(&mut self) -> Chunk<'source> {
        let span = self.span();
        let slice = self.slice();

        self.reset();

        Chunk::new(span, slice)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::lexer::token::Position;

    use super::Cursor;

    #[test]
    fn test_basic() {
        let mut cursor = Cursor::new("123123", &Path::new("main.u"));
        assert_eq!('1', cursor.next_char());
        assert_eq!('2', cursor.peek());
        assert_eq!('2', cursor.next_char());
        let chunk = cursor.chunk();
        assert_eq!(chunk.slice, "12");
        assert_eq!(chunk.position, Position::new(0, 2, 0, 2, &Path::new("main.u")));
    }

    #[test]
    fn test_utf8() {
        let mut cursor = Cursor::new("1😎Ϩ", &Path::new("main.u"));
        cursor.next_char();
        cursor.next_char();
        assert_eq!(cursor.span(), Position::new(0, 5, 0, 2, &Path::new("main.u")));
        cursor.next_char();
        assert_eq!(cursor.span(), Position::new(0, 7, 0, 3, &Path::new("main.u")));
    }

    #[test]
    fn test_lines() {
        let mut cursor = Cursor::new("\n\n\n2", &Path::new("main.u"));
        cursor.next_char();
        cursor.next_char();
        cursor.next_char();
        cursor.next_char();
        assert_eq!(cursor.span(), Position::new(0, 4, 3, 1, &Path::new("main.u")));
    }
}
