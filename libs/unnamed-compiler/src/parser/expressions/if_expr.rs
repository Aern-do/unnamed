use std::ops::Index;

use derive_macro::Parse;

use crate::{
    common::error::Result,
    lexer::token::Token,
    parser::{
        cursor::Cursor,
        primitive::{ElseKw, IfKw},
        Block, Parse, SyntaxKind,
    },
};

use super::Expression;

#[derive(Parse, Debug, Clone, PartialEq, Eq)]
pub struct IfExpression<'source> {
    if_kw: IfKw,
    expression: Box<Expression<'source>>,
    block: Block<'source>,
    else_kw: Option<ElseKw>,
    #[parse_if(else_kw.is_some())]
    alternative: Option<Alternative<'source>>,
}

impl<'source> IfExpression<'source> {
    pub fn new(
        expression: Expression<'source>,
        block: Block<'source>,
        alternative: Option<Alternative<'source>>,
    ) -> Self {
        Self {
            if_kw: Default::default(),
            expression: Box::new(expression),
            block,
            else_kw: if alternative.is_some() { Some(Default::default()) } else { None },
            alternative,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alternative<'source> {
    End(Block<'source>),
    If(Box<IfExpression<'source>>),
}

impl<'source> Parse<'source> for Alternative<'source> {
    fn parse<I: Index<usize, Output = Token<'source>>>(
        cursor: &mut Cursor<'source, I>,
    ) -> Result<'source, Self> {
        if IfKw::test(cursor) {
            Ok(Alternative::If(Box::new(cursor.parse()?)))
        } else {
            Ok(Alternative::End(cursor.parse()?))
        }
    }
}
