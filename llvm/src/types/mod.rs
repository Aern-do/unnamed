pub mod integer;
pub mod real;
pub mod function;
pub mod void;

use std::marker::PhantomData;

use llvm_sys::prelude::LLVMTypeRef;

#[derive(Debug, Clone, Copy)]
pub struct BaseType<'ctx> {
    inner: LLVMTypeRef,
    _context: PhantomData<&'ctx ()>,
}

impl<'ctx> BaseType<'ctx> {
    pub(crate) unsafe fn new(inner: LLVMTypeRef) -> Self {
        Self { inner, _context: Default::default() }
    }

    pub(crate) unsafe fn into_raw(&self) -> LLVMTypeRef {
        self.inner
    }
}