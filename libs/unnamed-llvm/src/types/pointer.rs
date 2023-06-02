use llvm_sys::prelude::LLVMTypeRef;

use crate::{impl_as_raw, impl_get_context, impl_type_downcast};

use super::BaseType;

#[derive(Debug, Clone, Copy)]
pub struct PointerType<'ctx>(pub(crate) BaseType<'ctx>);

impl_as_raw!(@downcast PointerType<'ctx>.0 -> LLVMTypeRef);
impl_get_context!(PointerType<'ctx>.0);
impl_type_downcast!(PointerType<'ctx> -> Type::Pointer);
