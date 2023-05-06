use std::ops::Index;

use crate::{
    common::error::Result,
    lexer::token::{Token, TokenKind},
};

use super::{
    cursor::Cursor,
    primitive::{Float, Integer, RightParenthesis, Identifier},
    Parse,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Division,
}

impl Operator {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            Operator::Plus | Operator::Minus => (1, 2),
            Operator::Multiply | Operator::Division => (3, 4),
        }
    }
}

impl<'source> Parse<'source> for Operator {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        let token = cursor.consume(&[
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Multiply,
            TokenKind::Division,
        ])?;
        Ok(match token.kind {
            TokenKind::Plus => Operator::Plus,
            TokenKind::Minus => Operator::Minus,
            TokenKind::Multiply => Operator::Multiply,
            TokenKind::Division => Operator::Division,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'source> {
    Integer(Integer<'source>),
    Float(Float<'source>),
    Identifier(Identifier<'source>)
}

impl<'source> Parse<'source> for Literal<'source> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        let token = cursor.test_and_return(&[TokenKind::Integer, TokenKind::Float, TokenKind::Identifier])?;
        Ok(match token.kind {
            TokenKind::Integer => Self::Integer(cursor.parse()?),
            TokenKind::Float => Self::Float(cursor.parse()?),
            TokenKind::Identifier => Self::Identifier(cursor.parse()?),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'source> {
    Literal(Literal<'source>),
    Infix { lhs: Box<Expression<'source>>, operator: Operator, rhs: Box<Expression<'source>> },
}

impl<'source> Expression<'source> {
    fn parse_bp<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
        min_bp: u8,
    ) -> Result<'source, Self> {
        let lhs = cursor.test_and_return(&[
            TokenKind::Integer,
            TokenKind::Float,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier
        ])?;
        let mut lhs = match lhs.kind {
            TokenKind::Float | TokenKind::Integer | TokenKind::Identifier => Expression::Literal(cursor.parse()?),
            TokenKind::LeftParenthesis => {
                cursor.next_token()?;
                let expression = cursor.parse::<Expression>()?;
                cursor.parse::<RightParenthesis>()?;
                expression
            }
            _ => unreachable!(),
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
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        Self::parse_bp(cursor, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::primitive::{Float, Integer, Identifier},
        tests,
    };

    use super::{Expression, Literal, Operator};

    macro_rules! int {
        ($lit: literal) => {
            Expression::Literal(Literal::Integer(Integer(stringify!($lit))))
        };
    }

    macro_rules! float {
        ($lit: literal) => {
            Expression::Literal(Literal::Float(Float(stringify!($lit))))
        };
    }

    macro_rules! ident {
        ($lit: ident) => {
            Expression::Literal(Literal::Identifier(Identifier(stringify!($lit))))
        };
    }

    macro_rules! infix {
        ($lhs: expr, $op: ident, $rhs: expr) => {
            Expression::Infix { lhs: Box::new($lhs), operator: Operator::$op, rhs: Box::new($rhs) }
        };
    }

    tests! {
        test_integer("10"): int!(10);
        test_float("1.0"): float!(1.0);
        test_identifier("pi"): ident!(pi);
        test_infix("2 + pi"): infix!(int!(2), Plus, ident!(pi));
        test_parenthesis("(2 + 2) * 2"): infix!(infix!(int!(2), Plus, int!(2)), Multiply, int!(2));
    }
}
