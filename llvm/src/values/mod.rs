pub mod function;
pub mod basic_block;
pub mod integer;

use std::marker::PhantomData;

use llvm_sys::prelude::LLVMValueRef;

#[derive(Debug, Clone, Copy)]
pub struct BaseValue<'ctx> {
    inner: LLVMValueRef,
    _context: PhantomData<&'ctx ()>,
}

impl<'ctx> BaseValue<'ctx> {
    pub(crate) unsafe fn new(inner: LLVMValueRef) -> Self {
        Self { inner, _context: Default::default() }
    }

    pub(crate) unsafe fn into_raw(&self) -> LLVMValueRef {
        self.inner
    }
}
