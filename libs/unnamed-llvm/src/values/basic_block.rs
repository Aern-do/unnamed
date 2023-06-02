use llvm_sys::prelude::LLVMBasicBlockRef;

use crate::{context::Context, impl_as_raw, GetContext};

#[derive(Debug, Clone)]
pub struct BasicBlock<'ctx> {
    context: &'ctx Context,
    inner: LLVMBasicBlockRef,
}

impl<'ctx> BasicBlock<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, inner: LLVMBasicBlockRef) -> Self {
        Self { context: _context, inner }
    }
}

impl<'ctx> GetContext for BasicBlock<'ctx> {
    fn get_context(&self) -> &Context {
        self.context
    }
}

impl_as_raw!(BasicBlock<'ctx>.inner -> LLVMBasicBlockRef);
