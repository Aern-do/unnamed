use llvm_sys::prelude::LLVMAttributeRef;

pub use crate::extra::{AttributeKind, AttributeLocation};
use crate::{context::Context, extra::LLVMCreateAttribute};

pub struct Attribute<'ctx> {
    _context: &'ctx Context,
    inner: LLVMAttributeRef,
}

impl<'ctx> Attribute<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, inner: LLVMAttributeRef) -> Self {
        Self { _context, inner }
    }

    pub(crate) fn as_attribute_ref(&self) -> LLVMAttributeRef {
        self.inner
    }

    pub fn create(context: &'ctx Context, kind: AttributeKind) -> Self {
        unsafe {
            let inner = LLVMCreateAttribute(context.as_context_ref(), kind);
            Self::new(context, inner)
        }
    }
}
