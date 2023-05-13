use derive_macro::Parse;

use crate::parser::{expressions::Expression, primitive::ReturnKw};

#[derive(Parse, Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement<'source> {
    pub return_kw: ReturnKw,
    #[parse_if(cursor.test(Expression::POSSIBLE_TOKENS)?)]
    pub expression: Option<Expression<'source>>,
}

impl<'source> ReturnStatement<'source> {
    pub fn new(expression: Option<Expression<'source>>) -> Self {
        Self { return_kw: Default::default(), expression }
    }
}
