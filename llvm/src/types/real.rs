use llvm_sys::core::{LLVMFloatTypeInContext, LLVMDoubleTypeInContext, LLVMHalfTypeInContext};

use crate::context::Context;

use super::BaseType;

macro_rules! create_type {
    ($($llvm_func: ident -> $name: ident),*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $name<'ctx>(BaseType<'ctx>);
            
            impl<'ctx> $name<'ctx> {
                pub fn create(context: &'ctx Context) -> Self {
                    unsafe {
                        let base_type_ref = $llvm_func(context.into_raw());
                        let base_type = BaseType::new(base_type_ref);
                        Self(base_type)
                    }
                }
            }

            impl<'ctx> From<$name<'ctx>> for BaseType<'ctx> {
                fn from(ty: $name<'ctx>) -> Self {
                    ty.0
                }
            }
        )*
    };
}

create_type!(LLVMHalfTypeInContext -> Half, LLVMFloatTypeInContext -> Float, LLVMDoubleTypeInContext -> Double);
