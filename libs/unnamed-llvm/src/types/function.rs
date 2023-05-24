use llvm_sys::core::LLVMIsFunctionVarArg;

use crate::{impl_as_type_ref, impl_type_downcast};

use super::{AsTypeRef, BaseType};

#[derive(Debug, Clone, Copy)]
pub struct FunctionType<'ctx>(pub(crate) BaseType<'ctx>);

impl<'ctx> FunctionType<'ctx> {
    pub fn is_variadic(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.0.as_type_ref()) != 0 }
    }
}

impl_as_type_ref!(FunctionType);
impl_type_downcast!(Function);
