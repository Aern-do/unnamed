use super::{Block, BoxedNode};
macro_rules! operator {
    ($operator: ident; $($token: ident),+) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $operator {
            $($token),+
        }
        impl<'a> TryFrom<crate::lexer::token::Token<'a>> for $operator {
            type Error = crate::parser::error::Error<'a>;
            fn try_from(token: crate::lexer::token::Token<'a>) -> Result<Self, Self::Error> {
                match token.kind {
                    $(crate::lexer::token::TokenKind::$token => Ok(Self::$token)),+,
                    _ => Err(
                        crate::parser::error::Error::new(crate::parser::error::ErrorKind::UnexpectedToken {
                            expected: &[
                                $(crate::lexer::token::TokenKind::$token),+
                            ],
                            received: Some(token.kind)
                        }, token.chunk)
                    )
                }
            }
        }
    };
}
operator!(InfixOperator; Add, Sub, Mul, Div, Greater, Less, GreaterEq, LessEq, And, Or, NotEq, Equal, Assignment);
impl InfixOperator {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            InfixOperator::Assignment => (2, 1),
            InfixOperator::Add | InfixOperator::Sub => (3, 4),
            InfixOperator::Mul | InfixOperator::Div => (5, 6),
            InfixOperator::Greater
            | InfixOperator::GreaterEq
            | InfixOperator::Less
            | InfixOperator::LessEq => (6, 7),
            InfixOperator::And | InfixOperator::Or | InfixOperator::NotEq => (8, 9),
            InfixOperator::Equal => (10, 11),
        }
    }
}
operator!(PrefixOperator; Add, Sub, Not);
impl PrefixOperator {
    pub fn binding_power(&self) -> ((), u8) {
        match self {
            PrefixOperator::Add | PrefixOperator::Sub => ((), 9),
            PrefixOperator::Not => ((), 10),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Infix {
        lhs: BoxedNode<'a>,
        rhs: BoxedNode<'a>,
        operator: InfixOperator,
    },
    Prefix {
        value: BoxedNode<'a>,
        operator: PrefixOperator,
    },
    Call {
        name: BoxedNode<'a>,
        arguments: Block<'a>,
    },
}
