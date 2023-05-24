mod float;
pub mod function;
pub mod integer;

use std::ffi::CStr;

use llvm_sys::{core::LLVMPrintTypeToString, prelude::LLVMTypeRef};

use crate::context::Context;

use self::{
    float::{DoubleType, FloatType, HalfType},
    function::FunctionType,
    integer::IntegerType,
};

#[macro_export]
macro_rules! impl_as_type_ref {
    ($ty: ident) => {
        impl<'ctx> $crate::types::AsTypeRef<'ctx> for $ty<'ctx> {
            fn as_type_ref(&'ctx self) -> llvm_sys::prelude::LLVMTypeRef {
                self.0.as_type_ref()
            }
        }
    };
}
#[macro_export]
macro_rules! impl_type_downcast {
    ($ty: ident) => {
        paste::paste! {
            impl<'ctx> From<[<$ty Type>]<'ctx>> for $crate::types::Type<'ctx> {
                fn from(ty: [<$ty Type>]<'ctx>) -> Self {
                    $crate::types::Type::$ty(ty)
                }
            }
        }
    };
}

pub trait AsTypeRef<'ctx> {
    fn as_type_ref(&'ctx self) -> LLVMTypeRef;
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BaseType<'ctx> {
    _context: &'ctx Context,
    inner: LLVMTypeRef,
}

impl<'ctx> BaseType<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, inner: LLVMTypeRef) -> Self {
        Self { _context, inner }
    }
}

impl<'ctx> AsTypeRef<'ctx> for BaseType<'ctx> {
    fn as_type_ref(&'ctx self) -> LLVMTypeRef {
        self.inner
    }
}

pub enum Type<'ctx> {
    Integer(IntegerType<'ctx>),
    Function(FunctionType<'ctx>),
    Half(HalfType<'ctx>),
    Float(FloatType<'ctx>),
    Double(DoubleType<'ctx>),
}

impl<'ctx> Type<'ctx> {
    pub fn print_to_string(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMPrintTypeToString(self.as_type_ref())) }
    }
}

impl<'ctx> AsTypeRef<'ctx> for Type<'ctx> {
    fn as_type_ref(&'ctx self) -> LLVMTypeRef {
        match self {
            Self::Integer(int) => int.as_type_ref(),
            Self::Function(func) => func.as_type_ref(),
            Self::Half(half) => half.as_type_ref(),
            Self::Float(float) => float.as_type_ref(),
            Self::Double(double) => double.as_type_ref(),
        }
    }
}
