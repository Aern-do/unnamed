use self::{expression::Expression, statement::Statement};

pub mod expression;
pub mod program;
pub mod statement;
pub type BoxedNode<'a> = Box<Node<'a>>;
pub type Block<'a> = Vec<Node<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Integer(&'a str),
    Float(&'a str),
    Identifier(&'a str),
    Expression(Expression<'a>),
    Statement(Statement<'a>),
}
