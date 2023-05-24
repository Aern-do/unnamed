pub mod function;
pub mod basic_block;

use llvm_sys::prelude::LLVMValueRef;

use crate::context::Context;

use self::function::FunctionValue;

#[macro_export]
macro_rules! impl_as_value_ref {
    ($ty: ident) => {
        impl<'ctx> $crate::values::AsValueRef<'ctx> for $ty<'ctx> {
            fn as_value_ref(&'ctx self) -> llvm_sys::prelude::LLVMValueRef {
                self.0.as_value_ref()
            }
        }
    };
}
#[macro_export]
macro_rules! impl_value_downcast {
    ($val: ident) => {
        paste::paste! {
            impl<'ctx> From<[<$val Value>]<'ctx>> for $crate::values::Value <'ctx> {
                fn from(val: [<$val Value>]<'ctx>) -> Self {
                    $crate::values::Value::$val(val)
                }
            }
        }
    };
}

pub trait AsValueRef<'ctx> {
    fn as_value_ref(&'ctx self) -> LLVMValueRef;
}

#[derive(Debug, Clone)]
pub(crate) struct BaseValue<'ctx> {
    context: &'ctx Context,
    inner: LLVMValueRef,
}

impl<'ctx> BaseValue<'ctx> {
    pub(crate) fn new(context: &'ctx Context, inner: LLVMValueRef) -> Self {
        Self { context, inner }
    }

    pub(crate) fn get_context(&self) -> &'ctx Context {
        self.context
    }
}

impl<'ctx> AsValueRef<'ctx> for BaseValue<'ctx> {
    fn as_value_ref(&'ctx self) -> LLVMValueRef {
        self.inner
    }
}

pub enum Value<'ctx> {
    Function(FunctionValue<'ctx>),
}

impl<'ctx> AsValueRef<'ctx> for Value<'ctx> {
    fn as_value_ref(&'ctx self) -> LLVMValueRef {
        match self {
            Value::Function(func) => func.as_value_ref(),
        }
    }
}
