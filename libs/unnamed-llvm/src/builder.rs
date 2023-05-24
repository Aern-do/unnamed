use llvm_sys::prelude::LLVMBuilderRef;

use crate::context::Context;

pub struct Builder<'ctx> {
    _context: &'ctx Context,
    _inner: LLVMBuilderRef,
}

impl<'ctx> Builder<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, _inner: LLVMBuilderRef) -> Self {
        Self { _context, _inner }
    }

    
}
