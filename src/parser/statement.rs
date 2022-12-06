use crate::lexer::token::TokenKind;

use super::{
    ast::{
        statement::{Alternate, Statement},
        Node,
    },
    error::Error,
    expression::DEFAULT_SKIP_STATES,
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> Result<Node<'a>, Error<'a>> {
        let token = self.cursor.peek()?;
        match token.kind {
            TokenKind::While => self.optional_semicolon(|parser| parser.parse_while()),
            TokenKind::If => self.optional_semicolon(|parser| parser.parse_if()),
            TokenKind::Let => self.semicolon(|parser| parser.parse_let()),
            TokenKind::Return => self.semicolon(|parser| parser.parse_return()),
            _ => self.semicolon(|parser| {
                parser.parse_expression(0, &[TokenKind::Semicolon, TokenKind::RightParenthesis])
            }),
        }
    }
    pub fn parse_let(&mut self) -> Result<Node<'a>, Error<'a>> {
        self.cursor.next_token()?;
        let mutable = self.cursor.test_and_next(&[TokenKind::Mut])?;
        let name = self.cursor.consume(&[TokenKind::Identifier])?;
        let init = if self.cursor.test_and_next(&[TokenKind::Assignment])? {
            Some(Box::new(self.parse_expression(0, &[TokenKind::Semicolon])?))
        } else {
            None
        };
        Ok(Node::Statement(Statement::Let {
            mutable,
            name: Box::new(Node::Identifier(name.chunk.slice)),
            init,
        }))
    }
    pub fn parse_if(&mut self) -> Result<Node<'a>, Error<'a>> {
        self.cursor.next_token()?;
        let test = self.parenthesized(|parser| parser.parse_expression(0, DEFAULT_SKIP_STATES))?;
        let consequent = self.parse_block()?;
        let alternate = if self.cursor.test(&[TokenKind::Else])? {
            self.cursor.next_token()?;
            if self.cursor.test(&[TokenKind::If])? {
                Some(Alternate::If(Box::new(self.parse_if()?)))
            } else {
                Some(Alternate::End(self.parse_block()?))
            }
        } else {
            None
        };
        Ok(Node::Statement(Statement::If {
            test: Box::new(test),
            consequent,
            alternate,
        }))
    }
    pub fn parse_return(&mut self) -> Result<Node<'a>, Error<'a>> {
        self.cursor.next_token()?;
        let expression = self.parse_expression(0, &[TokenKind::Semicolon])?;
        Ok(Node::Statement(Statement::Return {
            expression: Box::new(expression),
        }))
    }
    pub fn parse_while(&mut self) -> Result<Node<'a>, Error<'a>> {
        self.cursor.next_token()?;
        let test = self.parenthesized(|parser| parser.parse_expression(0, DEFAULT_SKIP_STATES))?;
        let body = self.parse_block()?;
        Ok(Node::Statement(Statement::While {
            test: Box::new(test),
            body,
        }))
    }
    pub fn parse_block(&mut self) -> Result<Vec<Node<'a>>, Error<'a>> {
        let mut statements = vec![];
        self.cursor.consume(&[TokenKind::LeftBrace])?;
        while !self.cursor.test(&[TokenKind::RightBrace])? {
            statements.push(self.parse_statement()?)
        }
        self.cursor.next_token()?;
        Ok(statements)
    }
}
