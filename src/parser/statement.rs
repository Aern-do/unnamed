use crate::lexer::token::{Token, TokenKind};

use super::{
    ast::{
        statement::{Alternate, Statement},
        Node,
    },
    error::Error,
    expression::DEFAULT_SKIP_STATES,
    Parser,
};

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
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
        let name = self.cursor.consume(&[TokenKind::Identifier])?.slice();
        let init = if self.cursor.test_and_next(&[TokenKind::Assignment])? {
            Some(Box::new(self.parse_expression(0, &[TokenKind::Semicolon])?))
        } else {
            None
        };
        Ok(Node::Statement(Statement::Let {
            mutable,
            name,
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
#[cfg(test)]
mod tests {
    use crate::parser::{
        ast::{
            expression::{Expression, InfixOperator},
            statement::{Alternate, Statement},
            Node,
        },
        test,
    };

    #[test]
    fn let_with_init() {
        test(
            "let a = 2;",
            Node::Statement(Statement::Let {
                mutable: false,
                name: "a",
                init: Some(Box::new(Node::Integer("2"))),
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn let_without_init() {
        test(
            "let a;",
            Node::Statement(Statement::Let {
                mutable: false,
                name: "a",
                init: None,
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn mutable_let_wuthout_init() {
        test(
            "let mut a;",
            Node::Statement(Statement::Let {
                mutable: true,
                name: "a",
                init: None,
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn mutable_let_with_init() {
        test(
            "let mut a = 2;",
            Node::Statement(Statement::Let {
                mutable: true,
                name: "a",
                init: Some(Box::new(Node::Integer("2"))),
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn return_statement() {
        test(
            "return 1;",
            Node::Statement(Statement::Return {
                expression: Box::new(Node::Integer("1")),
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn while_statement() {
        test(
            "while(a > 5) {}",
            Node::Statement(Statement::While {
                test: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("a")),
                    rhs: Box::new(Node::Integer("5")),
                    operator: InfixOperator::Greater,
                })),
                body: vec![],
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn optional_semicolon_while_statement() {
        test(
            "while(a > 5) {};",
            Node::Statement(Statement::While {
                test: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("a")),
                    rhs: Box::new(Node::Integer("5")),
                    operator: InfixOperator::Greater,
                })),
                body: vec![],
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn simple_if() {
        test(
            "if(a > 5) {}",
            Node::Statement(Statement::If {
                test: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("a")),
                    rhs: Box::new(Node::Integer("5")),
                    operator: InfixOperator::Greater,
                })),
                consequent: vec![],
                alternate: None,
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn optional_semicolon_simple_if() {
        test(
            "if(a > 5) {};",
            Node::Statement(Statement::If {
                test: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("a")),
                    rhs: Box::new(Node::Integer("5")),
                    operator: InfixOperator::Greater,
                })),
                consequent: vec![],
                alternate: None,
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn one_alternate_if() {
        test(
            "if(a > 5) {} else {}",
            Node::Statement(Statement::If {
                test: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("a")),
                    rhs: Box::new(Node::Integer("5")),
                    operator: InfixOperator::Greater,
                })),
                consequent: vec![],
                alternate: Some(Alternate::End(vec![])),
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn multiple_alternate_if() {
        test(
            "if(a > 5) {} else if(a < 5) {} else if(a == 2) {} else {}",
            Node::Statement(Statement::If {
                test: Box::new(Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Identifier("a")),
                    rhs: Box::new(Node::Integer("5")),
                    operator: InfixOperator::Greater,
                })),
                consequent: vec![],
                alternate: Some(Alternate::If(Box::new(Node::Statement(Statement::If {
                    test: Box::new(Node::Expression(Expression::Infix {
                        lhs: Box::new(Node::Identifier("a")),
                        rhs: Box::new(Node::Integer("5")),
                        operator: InfixOperator::Less,
                    })),
                    consequent: vec![],
                    alternate: Some(Alternate::If(Box::new(Node::Statement(Statement::If {
                        test: Box::new(Node::Expression(Expression::Infix {
                            lhs: Box::new(Node::Identifier("a")),
                            rhs: Box::new(Node::Integer("2")),
                            operator: InfixOperator::Equal,
                        })),
                        consequent: vec![],
                        alternate: Some(Alternate::End(vec![])),
                    })))),
                })))),
            }),
            |parser| parser.parse_statement(),
        )
    }
    #[test]
    fn empty_block() {
        test("{}", vec![], |parser| parser.parse_block())
    }
    #[test]
    fn block_with_expressions() {
        test(
            "{ 2 + 2; print(); }",
            vec![
                Node::Expression(Expression::Infix {
                    lhs: Box::new(Node::Integer("2")),
                    rhs: Box::new(Node::Integer("2")),
                    operator: InfixOperator::Add,
                }),
                Node::Expression(Expression::Call {
                    name: "print",
                    arguments: vec![],
                }),
            ],
            |parser| parser.parse_block(),
        )
    }
    #[test]
    fn block_with_statements() {
        test(
            "{ while(1) {}; let a = 2; }",
            vec![
                Node::Statement(Statement::While {
                    test: Box::new(Node::Integer("1")),
                    body: vec![],
                }),
                Node::Statement(Statement::Let {
                    mutable: false,
                    name: "a",
                    init: Some(Box::new(Node::Integer("2"))),
                }),
            ],
            |parser| parser.parse_block(),
        )
    }
}
