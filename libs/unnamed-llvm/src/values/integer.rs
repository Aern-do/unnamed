use llvm_sys::{core::LLVMTypeOf, prelude::LLVMValueRef};

use crate::{
    impl_as_raw, impl_get_context, impl_type_of, impl_value_downcast,
    types::{base_type::BaseType, integer::IntegerType},
    AsRaw,
};

use super::base_value::BaseValue;

#[derive(Debug, Clone)]
pub struct IntegerValue<'ctx>(pub(crate) BaseValue<'ctx>);

impl<'ctx> IntegerValue<'ctx> {
    pub fn get_type(&self) -> IntegerType<'ctx> {
        unsafe {
            let type_ref = LLVMTypeOf(self.as_raw());
            let base_type = BaseType::new(self.0.get_context(), type_ref);
            IntegerType(base_type)
        }
    }
}

impl_as_raw!(@downcast IntegerValue<'ctx>.0 -> LLVMValueRef);
impl_get_context!(IntegerValue<'ctx>.0);
impl_type_of!(IntegerValue<'ctx> -> IntegerType);
impl_value_downcast!(IntegerValue<'ctx> -> Value::Integer);
