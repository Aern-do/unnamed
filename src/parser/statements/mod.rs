use std::ops::Index;

use crate::{
    common::error::Result,
    lexer::token::{Token, TokenKind},
};

use self::{let_stmt::LetStatement, return_stmt::ReturnStatement};

use super::{cursor::Cursor, expressions::Expression, Parse};

pub mod let_stmt;
pub mod return_stmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'source> {
    Let(LetStatement<'source>),
    Return(ReturnStatement<'source>),
    Expression(Expression<'source>),
}

impl<'source> Parse<'source> for Statement<'source> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        let token = cursor.peek()?;
        Ok(match token.kind {
            TokenKind::LetKw => Statement::Let(cursor.parse()?),
            TokenKind::ReturnKw => Statement::Return(cursor.parse()?),
            _ => Statement::Expression(cursor.parse()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        int,
        parser::{
            expressions::{Expression, Literal},
            primitive::{Identifier, Integer},
        },
        tests,
    };

    use super::{let_stmt::LetStatement, return_stmt::ReturnStatement};

    tests! {
        test_empty_return("return"): ReturnStatement::new(None);
        test_return("return 42"): ReturnStatement::new(Some(int!(42)));
        test_empty_let("let test"): LetStatement::new(Identifier("test"), false, None);
        test_let_with_init("let test = 42"): LetStatement::new(Identifier("test"), false, Some(int!(42)));
        test_empty_let_mut("let mut test"): LetStatement::new(Identifier("test"), true, None);
        test_let_mut_with_init("let mut test = 42"): LetStatement::new(Identifier("test"), true, Some(int!(42)));
    }
}
