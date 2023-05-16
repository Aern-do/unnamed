use std::marker::PhantomData;

use llvm_sys::{
    prelude::{LLVMBasicBlockRef},
};

#[derive(Debug)]
pub struct BasicBlockValue<'ctx> {
    pub(crate) inner: LLVMBasicBlockRef,
    _context: PhantomData<&'ctx ()>,
}

impl<'ctx> BasicBlockValue<'ctx> {
    pub(crate) unsafe fn new(inner: LLVMBasicBlockRef) -> Self {
        Self { inner, _context: PhantomData }
    }
}
