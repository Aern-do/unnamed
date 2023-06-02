use llvm_sys::prelude::LLVMAttributeRef;

pub use crate::extra::{AttributeKind, AttributeLocation};
use crate::{context::Context, impl_as_raw, impl_get_context};

#[derive(Debug, Clone)]
pub struct Attribute<'ctx> {
    context: &'ctx Context,
    inner: LLVMAttributeRef,
}

impl<'ctx> Attribute<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, inner: LLVMAttributeRef) -> Self {
        Self { context: _context, inner }
    }
}

impl_as_raw!(Attribute<'ctx>.inner -> LLVMAttributeRef);
impl_get_context!(Attribute<'ctx>.context);
