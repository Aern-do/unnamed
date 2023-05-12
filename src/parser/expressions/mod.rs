pub mod if_expr;
pub mod while_expr;

use std::ops::Index;

use crate::{
    common::error::Result,
    lexer::token::{Token, TokenKind},
};

use self::{if_expr::IfExpression, while_expr::WhileExpression};

use super::{
    cursor::Cursor,
    primitive::{Comma, Float, Identifier, Integer, RightParenthesis},
    punctuated::Punctuated,
    Parse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'source> {
    Integer(Integer<'source>),
    Float(Float<'source>),
    Identifier(Identifier<'source>),
}

impl<'source> Parse<'source> for Literal<'source> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        let token = cursor.test_and_return(&[
            TokenKind::Integer,
            TokenKind::Float,
            TokenKind::Identifier,
        ])?;

        Ok(match token.kind {
            TokenKind::Integer => Self::Integer(cursor.parse()?),
            TokenKind::Float => Self::Float(cursor.parse()?),
            TokenKind::Identifier => Self::Identifier(cursor.parse()?),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'source> {
    Literal(Literal<'source>),
    If(IfExpression<'source>),
    While(WhileExpression<'source>),
    Call {
        ident: Identifier<'source>,
        arguments: Punctuated<'source, Expression<'source>, Comma, RightParenthesis>,
    },
    Infix {
        lhs: Box<Expression<'source>>,
        operator: Operator,
        rhs: Box<Expression<'source>>,
    },
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
            TokenKind::Identifier,
            TokenKind::IfKw,
            TokenKind::WhileKw,
        ])?;
        let mut lhs = match lhs.kind {
            TokenKind::Identifier => {
                let ident = cursor.parse()?;

                let expr = if cursor.test(&[TokenKind::LeftParenthesis])? {
                    cursor.next_token()?;
                    let call = Expression::Call { ident, arguments: cursor.parse()? };
                    cursor.parse::<RightParenthesis>()?;
                    call
                } else {
                    Expression::Literal(Literal::Identifier(ident))
                };

                expr
            }
            TokenKind::IfKw => Expression::If(cursor.parse()?),
            TokenKind::WhileKw => Expression::While(cursor.parse()?),
            TokenKind::Float | TokenKind::Integer => Expression::Literal(cursor.parse()?),
            TokenKind::LeftParenthesis => {
                cursor.next_token()?;
                let expression = cursor.parse::<Expression>()?;
                cursor.parse::<RightParenthesis>()?;
                expression
            }
            _ => unreachable!(),
        };

        loop {
            if cursor.test(&[
                TokenKind::RightParenthesis,
                TokenKind::RightBrace,
                TokenKind::Comma,
                TokenKind::Semicolon,
                TokenKind::LeftBrace,
            ])? {
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
        parser::{
            delimited::Delimited,
            primitive::{Float, Identifier, Integer},
            punctuated::Punctuated,
        },
        tests,
    };

    use super::{
        if_expr::{Alternative, IfExpression},
        while_expr::WhileExpression,
        Expression, Literal, Operator,
    };

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
    macro_rules! call {
        ($ident: ident($($arg: expr),*)) => {
            Expression::Call {
                ident: Identifier(stringify!($ident)),
                arguments: Punctuated::new(vec![$($arg),*])
            }
        };
    }

    macro_rules! empty_body {
        () => {
            Delimited::new(Punctuated::new(vec![]))
        };
    }

    tests! {
        test_integer("10"): int!(10);
        test_float("1.0"): float!(1.0);
        test_identifier("pi"): ident!(pi);
        test_infix("2 + pi"): infix!(int!(2), Plus, ident!(pi));
        test_call_no_args("test()"): call!(test());
        test_call_one_arg("test(1)"): call!(test(int!(1)));
        test_call_many_args("test(1, 2.0)"): call!(test(int!(1), float!(2.0)));
        test_parenthesis("(2 + 2) * 2"): infix!(infix!(int!(2), Plus, int!(2)), Multiply, int!(2));
        test_simple_if("if a {}"): IfExpression::new(ident!(a), empty_body!(), None);
        test_if_with_end_else("if a {} else {}"): IfExpression::new(ident!(a), empty_body!(), Some(Alternative::End(empty_body!())));
        test_if_with_if_else("if a {} else if b {}"): IfExpression::new(ident!(a), empty_body!(), Some(Alternative::If(Box::new(IfExpression::new(ident!(b), empty_body!(), None)))));
        test_while_expression("while 42 {}"): WhileExpression::new(int!(42), empty_body!())
    }
}
