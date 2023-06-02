use llvm_sys::{core::LLVMTypeOf, prelude::LLVMValueRef};

use crate::{context::Context, types::base_type::BaseType, AsRaw};

use super::TypeOf;

#[derive(Debug, Clone)]
pub(crate) struct BaseValue<'ctx> {
    context: &'ctx Context,
    inner: LLVMValueRef,
}

impl<'ctx> BaseValue<'ctx> {
    pub(crate) fn new(context: &'ctx Context, inner: LLVMValueRef) -> Self {
        Self { context, inner }
    }

    pub(crate) fn get_context(&self) -> &'ctx Context {
        self.context
    }
}

impl<'ctx> TypeOf for BaseValue<'ctx> {
    type Output<'ty> = BaseType<'ty>
    where
        Self: 'ty;

    fn type_of(&self) -> Self::Output<'_> {
        BaseType::new(self.get_context(), unsafe { LLVMTypeOf(self.as_raw()) })
    }
}

impl<'ctx> AsRaw for BaseValue<'ctx> {
    type Raw = LLVMValueRef;
    fn as_raw(&self) -> LLVMValueRef {
        self.inner
    }
}
