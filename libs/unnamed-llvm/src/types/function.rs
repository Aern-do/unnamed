use llvm_sys::{
    core::{LLVMGetReturnType, LLVMIsFunctionVarArg},
    prelude::LLVMTypeRef,
};

use crate::{impl_as_raw, impl_get_context, impl_type_downcast, AsRaw, GetContext};

use super::{BaseType, Type};

#[derive(Debug, Clone, Copy)]
pub struct FunctionType<'ctx>(pub(crate) BaseType<'ctx>);

impl<'ctx> FunctionType<'ctx> {
    pub fn is_variadic(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.as_raw()) != 0 }
    }

    pub fn return_ty(&self) -> Type {
        let ty_ref = unsafe { LLVMGetReturnType(self.as_raw()) };

        Type::from_base_type(BaseType::new(self.get_context(), ty_ref))
    }
}

impl_as_raw!(@downcast FunctionType<'ctx>.0 -> LLVMTypeRef);
impl_get_context!(FunctionType<'ctx>.0);
impl_type_downcast!(FunctionType<'ctx> -> Type::Function);
