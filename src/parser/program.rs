use crate::lexer::token::TokenKind;

use super::{
    ast::program::{Argument, Function, Program},
    error::Error,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_program(&mut self) -> Result<Program<'a>, Error<'a>> {
        let mut functions = vec![];
        while !self.cursor.eof() {
            functions.push(self.parse_function()?);
        }
        Ok(Program { functions })
    }
    pub fn parse_function(&mut self) -> Result<Function<'a>, Error<'a>> {
        self.cursor.consume(&[TokenKind::Function])?;
        let name = self.cursor.consume(&[TokenKind::Identifier])?.chunk.slice;

        self.cursor.consume(&[TokenKind::LeftParenthesis])?;
        let arguments = self.arguments(|parser| {
            let name = parser.cursor.consume(&[TokenKind::Identifier])?.chunk.slice;
            parser.cursor.consume(&[TokenKind::Colon])?;
            let argument_type = parser.cursor.consume(&[TokenKind::Identifier])?.chunk.slice;
            Ok(Argument {
                name,
                argument_type,
            })
        })?;
        self.cursor.consume(&[TokenKind::RightParenthesis])?;
        self.cursor.consume(&[TokenKind::Arrow])?;
        let return_type = self.cursor.consume(&[TokenKind::Identifier])?.chunk.slice;
        let body = self.parse_block()?;
        Ok(Function {
            name,
            arguments,
            return_type,
            body,
        })
    }
}
