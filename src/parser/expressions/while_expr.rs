use derive_macro::Parse;

use crate::parser::{primitive::WhileKw, Block};

use super::Expression;

#[derive(Parse, Debug, Clone, PartialEq, Eq)]
pub struct WhileExpression<'source> {
    while_kw: WhileKw,
    test: Box<Expression<'source>>,
    block: Block<'source>,
}

impl<'source> WhileExpression<'source> {
    pub fn new(test: Expression<'source>, block: Block<'source>) -> Self {
        Self { while_kw: Default::default(), test: Box::new(test), block }
    }
}
