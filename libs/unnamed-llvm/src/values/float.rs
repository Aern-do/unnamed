use llvm_sys::{core::LLVMTypeOf, prelude::LLVMValueRef};

use crate::{
    impl_as_raw, impl_get_context, impl_value_downcast,
    types::{
        base_type::BaseType,
        float::{FloatKind, FloatType},
    },
    AsRaw,
};

use super::{base_value::BaseValue, TypeOf};

#[derive(Debug, Clone)]
pub struct FloatValue<'ctx> {
    base_value: BaseValue<'ctx>,
    pub(crate) kind: FloatKind,
}

impl<'ctx> FloatValue<'ctx> {
    pub(crate) fn new(base_value: BaseValue<'ctx>, kind: FloatKind) -> Self {
        Self { base_value, kind }
    }
}

impl<'ctx> TypeOf for FloatValue<'ctx> {
    type Output<'ty> = FloatType<'ty> where Self: 'ty;

    fn type_of(&self) -> Self::Output<'_> {
        let float_ref = unsafe { LLVMTypeOf(self.as_raw()) };

        FloatType::new(BaseType::new(self.base_value.get_context(), float_ref), self.kind)
    }
}

impl_as_raw!(@downcast FloatValue<'ctx>.base_value -> LLVMValueRef);
impl_get_context!(FloatValue<'ctx>.base_value);
impl_value_downcast!(FloatValue<'ctx> -> Value::Float);
