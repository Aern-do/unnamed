use unicode_ident::{is_xid_continue, is_xid_start};

use self::{
    cursor::Cursor,
    error::{Error, ErrorKind},
    token::{Token, TokenKind},
};

pub mod chunk;
pub mod cursor;
pub mod error;
pub mod token;
#[inline(always)]
fn transform<T>(option: Option<T>) -> Result<T, Error> {
    option.ok_or_else(|| Error::empty(ErrorKind::UnexpectedEof))
}
macro_rules! token {
    ($self: ident; $kind: ident) => {{
        transform($self.cursor.next_char())?;
        Ok(crate::lexer::token::Token::new(
            crate::lexer::token::TokenKind::$kind,
            $self.cursor.chunk(),
        ))
    }};
    (@NONEXT $self: ident; $kind: ident) => {{
        Ok(crate::lexer::token::Token::new(
            crate::lexer::token::TokenKind::$kind,
            $self.cursor.chunk(),
        ))
    }};
    ($self: ident; $next_char: literal -> $kind: ident | $else: ident) => {{
        transform($self.cursor.next_char())?;
        if !$self.cursor.eof() && (transform($self.cursor.peek())? == &$next_char) {
            transform($self.cursor.next_char())?;
            Ok(crate::lexer::token::Token::new(
                crate::lexer::token::TokenKind::$kind,
                $self.cursor.chunk(),
            ))
        } else {
            Ok(crate::lexer::token::Token::new(
                crate::lexer::token::TokenKind::$else,
                $self.cursor.chunk(),
            ))
        }
    }};
    ($self: ident; $next_char: literal -> $kind: ident) => {{
        transform($self.cursor.next_char())?;
        if !$self.cursor.eof() && (transform($self.cursor.peek())? == &$next_char) {
            transform($self.cursor.next_char())?;
            Ok(crate::lexer::token::Token::new(
                crate::lexer::token::TokenKind::$kind,
                $self.cursor.chunk(),
            ))
        } else {
            transform($self.cursor.next_char())?;
            Err(crate::lexer::error::Error::new(
                $self.cursor.span(),
                crate::lexer::error::ErrorKind::UnexpectedToken,
            ))
        }
    }};
}
#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(cursor: Cursor<'a>) -> Self {
        Self { cursor }
    }

    #[inline]
    pub fn is_identifier_start(&mut self) -> Option<bool> {
        Some(is_xid_start(*self.cursor.peek()?))
    }
    #[inline]
    pub fn is_identifier_continue(&mut self) -> Option<bool> {
        Some(is_xid_continue(*self.cursor.peek()?))
    }
    pub fn is_number(&mut self) -> Option<(bool, bool)> {
        let char = self.cursor.peek()?;
        let is_float = *char == '.';
        Some((('0'..'9').contains(char) || is_float, is_float))
    }
    pub fn is_number_start(&mut self) -> Option<bool> {
        let char = self.cursor.peek()?;
        Some(('0'..'9').contains(char))
    }
    pub fn is_number_continue(&mut self) -> Option<bool> {
        let char = self.cursor.peek()?;
        Some(('0'..'9').contains(char) || *char == '.')
    }
    pub fn is_skipable(&mut self) -> Option<bool> {
        Some(matches!(
            self.cursor.peek()?,
            '\u{0009}'
                | '\u{000A}'
                | '\u{000B}'
                | '\u{000C}'
                | '\u{000D}'
                | '\u{0020}'
                | '\u{0085}'
                | '\u{200E}'
                | '\u{200F}'
                | '\u{2028}'
                | '\u{2029}'
        ))
    }
    pub fn lex_number(&mut self) -> Result<Token<'a>, Error> {
        let mut is_error = false;
        let mut is_float = false;
        transform(self.cursor.next_char())?;
        while !self.cursor.eof() && transform(self.is_number_continue())? {
            if transform(self.cursor.peek())? == &'.' {
                if is_float {
                    is_error = true
                }
                is_float = true
            }
            transform(self.cursor.next_char())?;
        }
        if is_error {
            return Err(Error::new(
                self.cursor.span(),
                ErrorKind::TooManyFloatingPoints,
            ));
        }
        if is_float {
            Ok(Token::new(TokenKind::Float, self.cursor.chunk()))
        } else {
            Ok(Token::new(TokenKind::Integer, self.cursor.chunk()))
        }
    }
    pub fn lex_identifier(&mut self) -> Result<Token<'a>, Error> {
        transform(self.cursor.next_char())?;
        while !self.cursor.eof() && transform(self.is_identifier_continue())? {
            transform(self.cursor.next_char())?;
        }
        match self.cursor.slice() {
            "function" => token!(@NONEXT self; Function),
            "if" => token!(@NONEXT self; If),
            "else" => token!(@NONEXT self; Else),
            "while" => token!(@NONEXT self; While),
            "let" => token!(@NONEXT self; Let),
            "mut" => token!(@NONEXT self; Mut),
            "return" => token!(@NONEXT self; Return),
            _ => token!(@NONEXT self; Identifier),
        }
    }
    pub fn lex_with_skips(&mut self) -> Option<Result<Token<'a>, Error>> {
        while !self.cursor.eof() && self.is_skipable()? {
            self.cursor.next_char()?;
        }
        self.cursor.reset();
        if self.cursor.eof() {
            None
        } else {
            Some(self.lex())
        }
    }
    pub fn lex_other(&mut self) -> Result<Token<'a>, Error> {
        match *transform(self.cursor.peek())? {
            '(' => token!(self; LeftParenthesis),
            ')' => token!(self; RightParenthesis),
            '{' => token!(self; LeftBrace),
            '}' => token!(self; RightBrace),
            '>' => token!(self; '=' -> GreaterEq | Greater),
            '<' => token!(self; '=' -> LessEq | Less),
            '!' => token!(self; '=' -> NotEq | Not),
            '&' => token!(self; '&' -> And),
            '|' => token!(self; '|' -> Or),
            '+' => token!(self; Add),
            '-' => token!(self; '>' -> Arrow | Sub),
            '*' => token!(self; Mul),
            '/' => token!(self; Div),
            '=' => token!(self; '=' -> Equal | Assignment),
            ',' => token!(self; Comma),
            ':' => token!(self; Colon),
            ';' => token!(self; Semicolon),
            _ => {
                let error = Error::new(self.cursor.span(), ErrorKind::UnexpectedToken);
                self.cursor.reset();
                Err(error)
            }
        }
    }
    pub fn lex(&mut self) -> Result<Token<'a>, Error> {
        if transform(self.is_number_start())? {
            return self.lex_number();
        }
        if transform(self.is_identifier_start())? {
            return self.lex_identifier();
        }
        self.lex_other()
    }
}
#[cfg(test)]
mod tests {
    use super::{cursor::Cursor, token::TokenKind, Lexer};

    fn test(input: &'static str, tokens: &'static [TokenKind]) {
        let cursor = Cursor::new(input);
        let mut lexer = Lexer::new(cursor);
        let mut result = vec![];
        while let Some(token) = lexer.lex_with_skips() {
            result.push(token.unwrap());
        }
        for (index, token) in result.iter().enumerate() {
            assert_eq!(tokens[index], token.kind)
        }
    }
    #[test]
    #[should_panic]
    fn too_many_floating_points() {
        test("1.2.3", &[TokenKind::Float])
    }
    #[test]
    fn ascii_identifier() {
        test("foo", &[TokenKind::Identifier])
    }
    #[test]
    fn unicode_identifier() {
        test("名前のない", &[TokenKind::Identifier])
    }
    #[test]
    fn integer() {
        test("1", &[TokenKind::Integer]);
        test("3475345", &[TokenKind::Integer]);
    }
    #[test]
    fn float() {
        test("1.0", &[TokenKind::Float]);
        test("1.25", &[TokenKind::Float]);
        test("3.14", &[TokenKind::Float]);
    }
    #[test]
    fn keywords() {
        test("function", &[TokenKind::Function]);
        test("if", &[TokenKind::If]);
        test("else", &[TokenKind::Else]);
        test("while", &[TokenKind::While]);
        test("let", &[TokenKind::Let]);
        test("mut", &[TokenKind::Mut]);
        test("return", &[TokenKind::Return]);
    }
    #[test]
    fn operators() {
        test("+", &[TokenKind::Add]);
        test("-", &[TokenKind::Sub]);
        test("*", &[TokenKind::Mul]);
        test("/", &[TokenKind::Div]);
        test(">", &[TokenKind::Greater]);
        test(">=", &[TokenKind::GreaterEq]);
        test("<", &[TokenKind::Less]);
        test("<=", &[TokenKind::LessEq]);
        test("!", &[TokenKind::Not]);
        test("!=", &[TokenKind::NotEq]);
        test("||", &[TokenKind::Or]);
        test("&&", &[TokenKind::And]);
        test("==", &[TokenKind::Equal]);
        test("=", &[TokenKind::Assignment]);
    }
    #[test]
    fn other() {
        test(",", &[TokenKind::Comma]);
        test("->", &[TokenKind::Arrow]);
        test(";", &[TokenKind::Semicolon]);
        test(":", &[TokenKind::Colon]);
    }
    #[test]
    fn complex() {
        test(
            r#"
        function add(a: int, b: int) -> int {
            return a + b;
        }
        function main() {
            print(add(1, 2));
            print(2.0);
        }
        "#,
            &[
                TokenKind::Function,
                TokenKind::Identifier,
                TokenKind::LeftParenthesis,
                TokenKind::Identifier,
                TokenKind::Colon,
                TokenKind::Identifier,
                TokenKind::Comma,
                TokenKind::Identifier,
                TokenKind::Colon,
                TokenKind::Identifier,
                TokenKind::RightParenthesis,
                TokenKind::Arrow,
                TokenKind::Identifier,
                TokenKind::LeftBrace,
                TokenKind::Return,
                TokenKind::Identifier,
                TokenKind::Add,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::RightBrace,
                TokenKind::Function,
                TokenKind::Identifier,
                TokenKind::LeftParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::LeftBrace,
                TokenKind::Identifier,
                TokenKind::LeftParenthesis,
                TokenKind::Identifier,
                TokenKind::LeftParenthesis,
                TokenKind::Integer,
                TokenKind::Comma,
                TokenKind::Integer,
                TokenKind::RightParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::Semicolon,
                TokenKind::Identifier,
                TokenKind::LeftParenthesis,
                TokenKind::Float,
                TokenKind::RightParenthesis,
                TokenKind::Semicolon,
                TokenKind::RightBrace,
            ],
        );
    }
}
