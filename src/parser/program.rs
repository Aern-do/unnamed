use crate::lexer::token::{Token, TokenKind};

use super::{
    ast::program::{Argument, Function, Program},
    error::Error,
    Parser,
};

impl<'a, T: Iterator<Item = Token<'a>>> Parser<'a, T> {
    pub fn parse_program(&mut self) -> Result<Program<'a>, Error<'a>> {
        let mut functions = vec![];
        while !self.cursor.eof() {
            functions.push(self.parse_function()?);
        }
        Ok(Program { functions })
    }
    pub fn parse_function(&mut self) -> Result<Function<'a>, Error<'a>> {
        self.cursor.consume(&[TokenKind::Function])?;
        let name = self.cursor.consume(&[TokenKind::Identifier])?.slice();
        let arguments = self.parenthesized(|parser| {
            parser.arguments(|parser| {
                let name = parser.cursor.consume(&[TokenKind::Identifier])?.slice();
                parser.cursor.consume(&[TokenKind::Colon])?;
                let argument_type = parser.cursor.consume(&[TokenKind::Identifier])?.slice();
                Ok(Argument {
                    name,
                    argument_type,
                })
            })
        })?;
        self.cursor.consume(&[TokenKind::Arrow])?;
        let return_type = self.cursor.consume(&[TokenKind::Identifier])?.slice();
        let body = self.parse_block()?;
        Ok(Function {
            name,
            arguments,
            return_type,
            body,
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::parser::{
        ast::{
            expression::{Expression, InfixOperator},
            program::{Argument, Function, Program},
            statement::Statement,
            Node,
        },
        test,
    };

    #[test]
    fn empty_program() {
        test("", Program { functions: vec![] }, |parser| {
            parser.parse_program()
        })
    }
    #[test]
    fn function_without_argumetns() {
        test(
            "function main() -> void {}",
            Function {
                name: "main",
                arguments: vec![],
                return_type: "void",
                body: vec![],
            },
            |parser| parser.parse_function(),
        )
    }
    #[test]
    fn function_with_arguments() {
        test(
            "function add(a: int, b: int) -> void {}",
            Function {
                name: "add",
                arguments: vec![
                    Argument {
                        name: "a",
                        argument_type: "int",
                    },
                    Argument {
                        name: "b",
                        argument_type: "int",
                    },
                ],
                return_type: "void",
                body: vec![],
            },
            |parser| parser.parse_function(),
        )
    }
    #[test]
    fn complex_test() {
        test(
            r#"
            function main() -> void {
                print(mul(pi, 2) * pi);
            }
            function mul(a: float, b: float) -> float {
                return a * b;
            }
        "#,
            Program {
                functions: vec![
                    Function {
                        name: "main",
                        arguments: vec![],
                        return_type: "void",
                        body: vec![Node::Expression(Expression::Call {
                            name: "print",
                            arguments: vec![Node::Expression(Expression::Infix {
                                lhs: Box::new(Node::Expression(Expression::Call {
                                    name: "mul",
                                    arguments: vec![Node::Identifier("pi"), Node::Integer("2")],
                                })),
                                rhs: Box::new(Node::Identifier("pi")),
                                operator: InfixOperator::Mul,
                            })],
                        })],
                    },
                    Function {
                        name: "mul",
                        arguments: vec![
                            Argument {
                                name: "a",
                                argument_type: "float",
                            },
                            Argument {
                                name: "b",
                                argument_type: "float",
                            },
                        ],
                        return_type: "float",
                        body: vec![Node::Statement(Statement::Return {
                            expression: Box::new(Node::Expression(Expression::Infix {
                                lhs: Box::new(Node::Identifier("a")),
                                rhs: Box::new(Node::Identifier("b")),
                                operator: InfixOperator::Mul,
                            })),
                        })],
                    },
                ],
            },
            |parser| parser.parse_program(),
        )
    }
}
