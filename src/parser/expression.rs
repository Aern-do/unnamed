use crate::lexer::token::TokenKind;

use super::{
    ast::{
        expression::{Expression, InfixOperator, PrefixOperator},
        Node,
    },
    error::{Error, ErrorKind},
    Parser,
};

pub const DEFAULT_SKIP_STATES: &[TokenKind; 1] = &[TokenKind::RightParenthesis];

impl<'a> Parser<'a> {
    pub fn parse_expression(
        &mut self,
        bp: u8,
        skip_states: &'static [TokenKind],
    ) -> Result<Node<'a>, Error<'a>> {
        let lhs = self.cursor.consume(&[
            TokenKind::Integer,
            TokenKind::Float,
            TokenKind::Identifier,
            TokenKind::Add,
            TokenKind::Sub,
            TokenKind::Not,
            TokenKind::LeftParenthesis,
        ])?;
        let mut lhs = match lhs.kind {
            TokenKind::Integer => Node::Integer(lhs.chunk.slice),
            TokenKind::Float => Node::Float(lhs.chunk.slice),
            TokenKind::Identifier => {
                if self.cursor.test(&[TokenKind::LeftParenthesis])? {
                    self.cursor.next_token()?;
                    let name = Node::Identifier(lhs.chunk.slice);
                    let arguments = self.arguments(|parser| {
                        parser.parse_expression(0, &[TokenKind::Comma, TokenKind::RightParenthesis])
                    })?;
                    self.cursor.consume(&[TokenKind::RightParenthesis])?;
                    Node::Expression(Expression::Call {
                        name: Box::new(name),
                        arguments,
                    })
                } else {
                    Node::Identifier(lhs.chunk.slice)
                }
            }
            TokenKind::Add | TokenKind::Sub | TokenKind::Not => {
                let operator = PrefixOperator::try_from(lhs)?;
                let value = self.parse_expression(operator.binding_power().1, skip_states)?;
                Node::Expression(Expression::Prefix {
                    value: Box::new(value),
                    operator,
                })
            }
            TokenKind::LeftParenthesis => {
                let value = self.parse_expression(0, skip_states)?;
                self.cursor.consume(&[TokenKind::RightParenthesis])?;
                value
            }
            _ => unreachable!(),
        };
        loop {
            let operator = match self.cursor.peek() {
                Ok(token) => token,
                Err(err) if err.kind == ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(err),
            };

            if skip_states.contains(&operator.kind) {
                break;
            }
            let operator = InfixOperator::try_from(operator)?;
            let (l_bp, r_bp) = operator.binding_power();
            if l_bp < bp {
                break;
            }
            self.cursor.next_token()?;
            let rhs = self.parse_expression(r_bp, skip_states)?;
            lhs = Node::Expression(Expression::Infix {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                operator,
            })
        }
        Ok(lhs)
    }
}
