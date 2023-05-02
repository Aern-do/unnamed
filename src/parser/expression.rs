use crate::{
    common::error::Result,
    group,
    lexer::token::{Token, TokenKind},
};

use super::{
    cursor::Cursor,
    primitive::{Division, Integer, LeftParenthesis, Minus, Multiply, Plus, RightParenthesis},
    Parse,
};

group!(Operator: Plus, Minus, Multiply, Division);

impl Operator {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            Operator::Plus(..) | Operator::Minus(..) => (1, 2),
            Operator::Multiply(..) | Operator::Division(..) => (3, 4),
        }
    }
}

group!(Lhs<'source>: Integer<'source>, LeftParenthesis);

#[derive(Debug, Clone)]
pub enum Expression<'source> {
    Integer(Integer<'source>),
    Infix { lhs: Box<Expression<'source>>, operator: Operator, rhs: Box<Expression<'source>> },
}

impl<'source> Expression<'source> {
    fn parse_bp<I: Iterator<Item = Token<'source>> + Clone>(
        cursor: &mut Cursor<'source, I>,
        min_bp: u8,
    ) -> Result<'source, Self> {
        let mut lhs = match cursor.parse::<Lhs>()? {
            Lhs::Integer(integer) => Expression::Integer(integer),
            Lhs::LeftParenthesis(..) => {
                let expression = cursor.parse::<Expression>()?;
                cursor.parse::<RightParenthesis>()?;
                expression
            }
        };

        loop {
            if cursor.test(&[TokenKind::RightParenthesis])? {
                break;
            }

            let operator = match cursor.parse_without_consume::<Operator>() {
                Ok(op) => op,
                Err(err) if err.kind.is_eof() => break,
                Err(err) => return Err(err),
            };

            let (l_bp, r_bp) = operator.binding_power();
            if l_bp < min_bp {
                break;
            }

            cursor.next_token()?;
            let rhs = Expression::parse_bp(cursor, r_bp)?;

            lhs = Expression::Infix { lhs: Box::new(lhs), operator, rhs: Box::new(rhs) }
        }
        Ok(lhs)
    }
}

impl<'source> Parse<'source> for Expression<'source> {
    fn parse<I: Iterator<Item = Token<'source>> + Clone>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        Self::parse_bp(cursor, 0)
    }
}
