use self::{
    cursor::Cursor,
    error::{Error, Result},
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

    pub fn is_integer(&mut self) -> bool {
        self.cursor.peek().is_ascii_digit()
    }

    pub fn is_whitespace(&mut self) -> bool {
        self.cursor.peek().is_whitespace()
    }

    pub fn lex_integer(&mut self) -> Result<Token<'source>> {
        while !self.cursor.eof() && self.is_integer() {
            self.cursor.next_char();
        }

        Ok(Token::new(TokenKind::Integer, self.cursor.chunk()))
    }

    pub fn skip_whitespaces(&mut self) {
        while !self.cursor.eof() && self.is_whitespace() {
            self.cursor.next_char();
        }

        self.cursor.reset();
    }

    pub fn lex_special_symbols(&mut self) -> Result<Token<'source>> {
        let kind = match self.cursor.peek() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Division,
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
            _ => return Err(Error::UnexpectedToken),
        };
        self.cursor.next_char();

        Ok(Token::new(kind, self.cursor.chunk()))
    }

    pub fn lex(&mut self) -> Result<Token<'source>> {
        if self.is_integer() {
            return self.lex_integer();
        }

        self.lex_special_symbols()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token<'source>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        if self.cursor.eof() {
            return None;
        }
        Some(self.lex())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        cursor::Cursor,
        token::{Chunk, Span, Token, TokenKind},
        Lexer,
    };

    macro_rules! tests {
        ($($test_name: ident($input: literal) = $($kind: ident: $slice: literal at $start: literal..$end: literal),+);* $(;)*) => {
            $(
                #[test]
                fn $test_name() {
                    let cursor = Cursor::new($input);
                    let mut lexer = Lexer::new(cursor);
                    $(
                        assert_eq!(Token::new(TokenKind::$kind, Chunk::new(
                            Span::new($start, $end),
                            $slice
                        )), lexer.next().unwrap().unwrap());
                    )*
                }
            )*
        };
    }

    tests!(
        test_integer("123") = Integer: "123" at 0..3;
        test_plus("+") = Plus: "+" at 0..1;
        test_minus("-") = Minus: "-" at 0..1;
        test_multiply("*") = Multiply: "*" at 0..1;
        test_division("/") = Division: "/" at 0..1;
        test_left_parenthesis("(") = LeftParenthesis: "(" at 0..1;
        test_right_parenthesis(")") = RightParenthesis: ")" at 0..1;
        test_skip_whitespaces("  123  456  ") = Integer: "123" at 2..5, Integer: "456" at 7..10;
        test_complex("2 + 2 * 2") = Integer: "2" at 0..1, Plus: "+" at 2..3, Integer: "2" at 4..5, Multiply: "*" at 6..7, Integer: "2" at 8..9;
    );

    #[test]
    #[should_panic]
    fn test_unexpected_token() {
        let cursor = Cursor::new("`");
        let mut lexer = Lexer::new(cursor);
        lexer.next().unwrap().unwrap();
    }

    #[test]
    fn test_empty() {
        let cursor = Cursor::new("");
        let mut lexer = Lexer::new(cursor);
        assert!(lexer.next().is_none())
    }

    #[test]
    fn test_empty_whitespaces() {
        let cursor = Cursor::new("    \n\t");
        let mut lexer = Lexer::new(cursor);
        assert!(lexer.next().is_none())
    }
}
