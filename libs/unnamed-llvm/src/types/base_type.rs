use llvm_sys::{core::LLVMGetTypeKind, prelude::LLVMTypeRef};

use crate::{context::Context, AsRaw, GetContext};

use super::TypeKind;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BaseType<'ctx> {
    context: &'ctx Context,
    inner: LLVMTypeRef,
}

impl<'ctx> BaseType<'ctx> {
    pub(crate) fn new(context: &'ctx Context, inner: LLVMTypeRef) -> Self {
        Self { context, inner }
    }

    pub(crate) fn kind(&self) -> TypeKind {
        unsafe { LLVMGetTypeKind(self.as_raw()).into() }
    }
}

impl<'ctx> GetContext for BaseType<'ctx> {
    fn get_context(&self) -> &Context {
        self.context
    }
}

impl<'ctx> AsRaw for BaseType<'ctx> {
    type Raw = LLVMTypeRef;

    fn as_raw(&self) -> Self::Raw {
        self.inner
    }
}
