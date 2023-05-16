use llvm_sys::{core::LLVMContextCreate, prelude::LLVMContextRef};

use crate::{module::Module, builder::Builder};

#[derive(Debug)]
pub struct Context(LLVMContextRef);

impl Context {
    pub(crate) unsafe fn new(context_ref: LLVMContextRef) -> Self {
        Self(context_ref)
    }
    pub fn create() -> Self {
        unsafe {
            let context_ref = LLVMContextCreate();
            Self::new(context_ref)
        }
    }

    pub fn module(&self, name: &str) -> Module {
        Module::create(name, self)
    }

    pub fn builder(&self) -> Builder {
        Builder::create(self)
    }

    pub(crate) unsafe fn into_raw(&self) -> LLVMContextRef {
        self.0
    }
}
