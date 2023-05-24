use llvm_sys::prelude::LLVMBasicBlockRef;

use crate::context::Context;

pub struct BasicBlock<'ctx> {
    _context: &'ctx Context,
    _inner: LLVMBasicBlockRef,
}

impl<'ctx> BasicBlock<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, _inner: LLVMBasicBlockRef) -> Self {
        Self { _context, _inner }
    }
}
