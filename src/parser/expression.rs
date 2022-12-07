use crate::lexer::token::{Token, TokenKind};

use super::{
    ast::{
        expression::{Expression, InfixOperator, PrefixOperator},
        Node,
    },
    error::{Error, ErrorKind},
    Parser,
};

pub const DEFAULT_SKIP_STATES: &[TokenKind; 1] = &[TokenKind::RightParenthesis];

impl<'a, T: Iterator<Item = Token<'a>>> Parser<'a, T> {
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
            TokenKind::Integer => Node::Integer(lhs.slice()),
            TokenKind::Float => Node::Float(lhs.slice()),
            TokenKind::Identifier => {
                if self.cursor.test(&[TokenKind::LeftParenthesis])? {
                    self.cursor.next_token()?;
                    let name = lhs.slice();
                    let arguments = self.arguments(|parser| {
                        parser.parse_expression(0, &[TokenKind::Comma, TokenKind::RightParenthesis])
                    })?;
                    self.cursor.consume(&[TokenKind::RightParenthesis])?;
                    Node::Expression(Expression::Call { name, arguments })
                } else {
                    Node::Identifier(lhs.slice())
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
#[cfg(test)]
mod tests {
    use crate::parser::{
        ast::{
            expression::{Expression, InfixOperator, PrefixOperator},
            Node,
        },
        test,
    };

    use super::DEFAULT_SKIP_STATES;

    #[test]
    fn integer() {
        test("12", Node::Integer("12"), |parser| {
            parser.parse_expression(0, DEFAULT_SKIP_STATES)
        });
    }
    #[test]
    fn float() {
        test("1.2", Node::Float("1.2"), |parser| {
            parser.parse_expression(0, DEFAULT_SKIP_STATES)
        })
    }
    #[test]
    fn identifier() {
        test("abcd", Node::Identifier("abcd"), |parser| {
            parser.parse_expression(0, DEFAULT_SKIP_STATES)
        })
    }
    #[test]
    fn infix_expression() {
        test(
            "2 + 2",
            Node::Expression(Expression::Infix {
                lhs: Box::new(Node::Integer("2")),
                rhs: Box::new(Node::Integer("2")),
                operator: InfixOperator::Add,
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        );
    }
    #[test]
    fn assigment() {
        test(
            "a = b = 5 * 2",
            Node::Expression(Expression::Infix {
                lhs: Box::new(Node::Identifier("a")),
                rhs: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("b")),
                    rhs: Box::new(Node::Expression(Expression::Infix {
                        lhs: Box::new(Node::Integer("5")),
                        rhs: Box::new(Node::Integer("2")),
                        operator: InfixOperator::Mul,
                    })),
                    operator: InfixOperator::Assignment,
                })),
                operator: InfixOperator::Assignment,
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        )
    }
    #[test]
    fn preifx_expression() {
        test(
            "-2",
            Node::Expression(Expression::Prefix {
                value: Box::new(Node::Integer("2")),
                operator: PrefixOperator::Sub,
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        );
    }
    #[test]
    fn function_calls_without_arguments() {
        test(
            "print()",
            Node::Expression(Expression::Call {
                name: "print",
                arguments: vec![],
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        );
    }
    #[test]
    fn function_calls() {
        test(
            "cos(pi / 2)",
            Node::Expression(Expression::Call {
                name: "cos",
                arguments: vec![Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("pi")),
                    rhs: Box::new(Node::Integer("2")),
                    operator: InfixOperator::Div,
                })],
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        );
    }
    #[test]
    fn function_calls_with_multiple_arguments() {
        test(
            "mul(pi, 2)",
            Node::Expression(Expression::Call {
                name: "mul",
                arguments: vec![Node::Identifier("pi"), Node::Integer("2")],
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        );
    }
    #[test]
    fn parenthesized() {
        test(
            "(2 + 2) * 2",
            Node::Expression(Expression::Infix {
                lhs: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Integer("2")),
                    rhs: Box::new(Node::Integer("2")),
                    operator: InfixOperator::Add,
                })),
                rhs: Box::new(Node::Integer("2")),
                operator: InfixOperator::Mul,
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        )
    }
    #[test]
    fn complex_test() {
        test(
            "5 * f(x, y) + g(z) / 6",
            Node::Expression(Expression::Infix {
                lhs: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Integer("5")),
                    rhs: Box::new(Node::Expression(Expression::Call {
                        name: "f",
                        arguments: vec![Node::Identifier("x"), Node::Identifier("y")],
                    })),
                    operator: InfixOperator::Mul,
                })),
                rhs: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Expression(Expression::Call {
                        name: "g",
                        arguments: vec![Node::Identifier("z")],
                    })),
                    rhs: Box::new(Node::Integer("6")),
                    operator: InfixOperator::Div,
                })),
                operator: InfixOperator::Add,
            }),
            |parser| parser.parse_expression(0, DEFAULT_SKIP_STATES),
        )
    }
}
