use llvm_sys::core::{LLVMVoidTypeInContext};

use crate::context::Context;

use super::BaseType;

#[derive(Debug)]
pub struct VoidType<'ctx>(BaseType<'ctx>);

impl<'ctx> VoidType<'ctx> {
    pub fn create(context: &'ctx Context) -> Self {
        unsafe {
            let base_type_ref = LLVMVoidTypeInContext(context.into_raw());
            let base_type = BaseType::new(base_type_ref);
            VoidType(base_type)
        }
    }
}

impl<'ctx> From<VoidType<'ctx>> for BaseType<'ctx> {
    fn from(value: VoidType<'ctx>) -> Self {
        value.0
    }
}