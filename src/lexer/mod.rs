use unicode_ident::{is_xid_continue, is_xid_start};

use crate::common::{
    error::{Error, Result},
    CommonErrorKind,
};

use self::{
    cursor::Cursor,
    error::ErrorKind,
    token::{Token, TokenKind},
};

pub mod cursor;
pub mod error;
pub mod token;

pub struct Lexer<'source> {
    cursor: Cursor<'source>,
}

impl<'source> Lexer<'source> {
    pub fn new(cursor: Cursor<'source>) -> Self {
        Self { cursor }
    }

    pub fn is_number_start(&mut self) -> bool {
        self.cursor.peek().is_ascii_digit()
    }

    pub fn is_number_continue(&mut self) -> bool {
        let char = self.cursor.peek();
        char.is_ascii_digit() || char == '.'
    }

    pub fn is_identifier_start(&mut self) -> bool {
        is_xid_start(self.cursor.peek())
    }

    pub fn is_identifier_continue(&mut self) -> bool {
        is_xid_continue(self.cursor.peek())
    }

    pub fn is_whitespace(&mut self) -> bool {
        self.cursor.peek().is_whitespace()
    }

    pub fn lex_number(&mut self) -> Result<'source, Token<'source>> {
        let mut floating_points = 0_u8;

        while !self.cursor.is_eof() && self.is_number_continue() {
            if self.cursor.peek() == '.' {
                floating_points += 1;
            }
            self.cursor.next_char();
        }

        match floating_points {
            1 => Ok(Token::new(TokenKind::Float, self.cursor.chunk())),
            0 => Ok(Token::new(TokenKind::Integer, self.cursor.chunk())),
            _ => Err(Error::new(
                CommonErrorKind::Lexer(ErrorKind::TooManyFloatingPoints(floating_points)),
                Some(self.cursor.chunk()),
            )),
        }
    }

    pub fn skip_whitespaces(&mut self) {
        while !self.cursor.is_eof() && self.is_whitespace() {
            self.cursor.next_char();
        }

        self.cursor.reset();
    }

    pub fn lex_identifier(&mut self) -> Result<'source, Token<'source>> {
        self.cursor.next_char();
        while !self.cursor.is_eof() && self.is_identifier_continue() {
            self.cursor.next_char();
        }
        let chunk = self.cursor.chunk();
        Ok(match chunk.slice {
            "func" => Token::new(TokenKind::FuncKw, chunk),
            "if" => Token::new(TokenKind::IfKw, chunk),
            "else" => Token::new(TokenKind::ElseKw, chunk),
            _ => Token::new(TokenKind::Identifier, chunk),
        })
    }

    pub fn lex_special_symbols(&mut self) -> Result<'source, Token<'source>> {
        let kind = match self.cursor.next_char() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Division,
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            _ => {
                return Err(Error::new(
                    CommonErrorKind::Lexer(ErrorKind::UnexpectedToken),
                    Some(self.cursor.chunk()),
                ))
            }
        };

        Ok(Token::new(kind, self.cursor.chunk()))
    }

    pub fn lex(&mut self) -> Result<'source, Token<'source>> {
        if self.is_number_start() {
            return self.lex_number();
        }
        if self.is_identifier_start() {
            return self.lex_identifier();
        }

        self.lex_special_symbols()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<'source, Token<'source>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        if self.cursor.is_eof() {
            return None;
        }
        Some(self.lex())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        cursor::Cursor,
        token::{Chunk, Position, Token, TokenKind},
        Lexer,
    };
    use std::path::Path;

    macro_rules! tests {
        ($($test_name: ident($input: literal) = $($kind: ident: $slice: literal at $start: literal..$end: literal),+);* $(;)*) => {
            $(
                #[test]
                fn $test_name() {
                    let cursor = Cursor::new($input, &Path::new("test.u"));
                    let mut lexer = Lexer::new(cursor);
                    $(
                        assert_eq!(Token::new(TokenKind::$kind, Chunk::new(
                            Position::new($start, $end, 0, 0, Path::new("test.u")),
                            $slice
                        )), lexer.next().unwrap().unwrap());
                    )*
                }
            )*
        };
    }

    tests!(
        test_integer("123") = Integer: "123" at 0..3;
        test_float("1.0") = Float: "1.0" at 0..3;
        test_identifier("test") = Identifier: "test" at 0..4;
        test_unicode_identifier("проверка") = Identifier: "проверка" at 0..16;
        test_plus("+") = Plus: "+" at 0..1;
        test_minus("-") = Minus: "-" at 0..1;
        test_multiply("*") = Multiply: "*" at 0..1;
        test_division("/") = Division: "/" at 0..1;
        test_left_parenthesis("(") = LeftParenthesis: "(" at 0..1;
        test_right_parenthesis(")") = RightParenthesis: ")" at 0..1;
        test_left_braces("{") = LeftBrace: "{" at 0..1;
        test_right_braces("}") = RightBrace: "}" at 0..1;
        test_comma(",") = Comma: "," at 0..1;
        test_colon(":") = Colon: ":" at 0..1;
        test_semicolon(";") = Semicolon: ";" at 0..1;
        test_func_kw("func") = FuncKw: "func" at 0..4;
        test_if_kw("if") = IfKw: "if" at 0..2;
        test_else_kw("else") = ElseKw: "else" at 0..4;
        test_skip_whitespaces("  123  456  ") = Integer: "123" at 2..5, Integer: "456" at 7..10;
        test_complex("2 + 2 * 2") = Integer: "2" at 0..1, Plus: "+" at 2..3, Integer: "2" at 4..5, Multiply: "*" at 6..7, Integer: "2" at 8..9;
    );

    #[test]
    fn test_unexpected_token() {
        let cursor = Cursor::new("`", &Path::new("main.u"));
        let mut lexer = Lexer::new(cursor);
        assert!(lexer.next().unwrap().is_err())
    }

    #[test]
    fn test_too_many_floating_points() {
        let cursor = Cursor::new("1.2.3", &Path::new("main.u"));
        let mut lexer = Lexer::new(cursor);
        assert!(lexer.next().unwrap().is_err())
    }

    #[test]
    fn test_empty() {
        let cursor = Cursor::new("", &Path::new("main.u"));
        let mut lexer = Lexer::new(cursor);
        assert!(lexer.next().is_none())
    }

    #[test]
    fn test_empty_whitespaces() {
        let cursor = Cursor::new("    \n\t", &Path::new("main.u"));
        let mut lexer = Lexer::new(cursor);
        assert!(lexer.next().is_none())
    }
}
