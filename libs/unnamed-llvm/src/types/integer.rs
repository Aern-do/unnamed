use llvm_sys::{
    core::{LLVMConstInt, LLVMGetIntTypeWidth},
    prelude::LLVMTypeRef,
};

use crate::{
    impl_as_raw, impl_get_context, impl_type_downcast,
    values::{base_value::BaseValue, integer::IntegerValue},
    AsRaw, GetContext,
};

use super::BaseType;

#[derive(Debug, Clone, Copy)]
pub struct IntegerType<'ctx>(pub(crate) BaseType<'ctx>);

impl<'ctx> IntegerType<'ctx> {
    pub fn width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.as_raw()) }
    }

    pub fn constant(&self, value: u64) -> IntegerValue {
        let int_ref = unsafe { LLVMConstInt(self.as_raw(), value, 0) };

        IntegerValue(BaseValue::new(self.get_context(), int_ref))
    }
}

impl_as_raw!(@downcast IntegerType<'ctx>.0 -> LLVMTypeRef);
impl_get_context!(IntegerType<'ctx>.0);
impl_type_downcast!(IntegerType<'ctx> -> Type::Integer);
