use derive_macro::Parse;

use crate::parser::{
    expressions::Expression,
    primitive::{Assignment, Identifier, LetKw, MutKw},
};

#[derive(Parse, Debug, Clone, PartialEq, Eq)]
pub struct LetStatement<'source> {
    pub let_kw: LetKw,
    pub mut_kw: Option<MutKw>,
    pub name: Identifier<'source>,
    pub assignment_token: Option<Assignment>,
    #[parse_if(assignment_token.is_some())]
    pub init: Option<Expression<'source>>,
}

impl<'source> LetStatement<'source> {
    pub fn new(name: Identifier<'source>, is_mut: bool, init: Option<Expression<'source>>) -> Self {
        Self {
            let_kw: Default::default(),
            mut_kw: if is_mut { Some(Default::default()) } else { None },
            name,
            assignment_token: if init.is_some() { Some(Default::default()) } else { None },
            init,
        }
    }
}
