pub mod if_expr;
pub mod while_expr;

use std::ops::Index;

use crate::{
    check,
    common::error::Result,
    consume,
    lexer::token::{Token, TokenKind},
};

use self::{if_expr::IfExpression, while_expr::WhileExpression};

use super::{
    cursor::Cursor,
    primitive::{Comma, Float, Identifier, Integer, RightParenthesis, TrueKw, FalseKw},
    punctuated::Punctuated,
    Parse,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Division,
    Less,
    LessEq,
    Greeter,
    GreeterEq,
    Eq,
    Assignment,
}

impl Operator {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            Operator::Assignment => (0, 1),
            Operator::Plus | Operator::Minus => (2, 3),
            Operator::Multiply | Operator::Division => (4, 5),
            Operator::Less | Operator::LessEq | Operator::Greeter | Operator::GreeterEq => (6, 7),
            Operator::Eq => (8, 9),
        }
    }
}

impl<'source> Parse<'source> for Operator {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        Ok(consume!(cursor(_token) {
            Plus => Operator::Plus,
            Minus => Operator::Minus,
            Multiply => Operator::Multiply,
            Division => Operator::Division,
            Less => Operator::Less,
            LessEq => Operator::LessEq,
            Greeter => Operator::Greeter,
            GreeterEq => Operator::GreeterEq,
            Eq => Operator::Eq,
            Assignment => Operator::Assignment
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'source> {
    Integer(Integer<'source>),
    Float(Float<'source>),
    Identifier(Identifier<'source>),
    True,
    False,
}

impl<'source> Parse<'source> for Literal<'source> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        Ok(check!(cursor(_token) {
            Integer => Literal::Integer(cursor.parse()?),
            Float => Literal::Float(cursor.parse()?),
            Identifier => Literal::Identifier(cursor.parse()?),
            TrueKw => {
                cursor.parse::<TrueKw>()?;
                Literal::True
            },
            FalseKw => {
                cursor.parse::<FalseKw>()?;
                Literal::False
            }
        }))
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
    pub const POSSIBLE_TOKENS: &'static [TokenKind] = &[
        TokenKind::Identifier,
        TokenKind::IfKw,
        TokenKind::WhileKw,
        TokenKind::Float,
        TokenKind::Integer,
        TokenKind::LeftParenthesis,
    ];

    fn parse_bp<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
        min_bp: u8,
    ) -> Result<'source, Self> {
        let mut lhs = check!(cursor(_lhs) {
            Identifier => {
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
            },
            IfKw => Expression::If(cursor.parse()?),
            WhileKw => Expression::While(cursor.parse()?),
            Float | Integer | TrueKw | FalseKw => Expression::Literal(cursor.parse()?),
            LeftParenthesis => {
                cursor.next_token()?;
                let expression = cursor.parse::<Expression>()?;
                cursor.parse::<RightParenthesis>()?;
                expression
            }
        });

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

    #[macro_export]
    macro_rules! int {
        ($lit: literal) => {
            Expression::Literal(Literal::Integer(Integer(stringify!($lit))))
        };
    }

    #[macro_export]
    macro_rules! float {
        ($lit: literal) => {
            Expression::Literal(Literal::Float(Float(stringify!($lit))))
        };
    }

    #[macro_export]
    macro_rules! ident {
        ($lit: ident) => {
            Expression::Literal(Literal::Identifier(Identifier(stringify!($lit))))
        };
    }

    #[macro_export]
    macro_rules! infix {
        ($lhs: expr, $op: ident, $rhs: expr) => {
            Expression::Infix { lhs: Box::new($lhs), operator: Operator::$op, rhs: Box::new($rhs) }
        };
    }

    #[macro_export]
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
        test_true("true"): Expression::Literal(Literal::True);
        test_false("false"): Expression::Literal(Literal::False);
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
