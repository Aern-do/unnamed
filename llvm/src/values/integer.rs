use super::{BaseValue};

#[derive(Debug)]
pub struct IntegerValue<'ctx>(pub(crate) BaseValue<'ctx>);

impl<'ctx> From<IntegerValue<'ctx>> for BaseValue<'ctx> {
    fn from(value: IntegerValue<'ctx>) -> Self {
        value.0
    }
}
