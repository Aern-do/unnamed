pub(crate) mod base_type;
pub mod float;
pub mod function;
pub mod integer;
pub mod pointer;

use std::ffi::CStr;

use llvm_sys::{core::LLVMPrintTypeToString, prelude::LLVMTypeRef, LLVMTypeKind};

use crate::{context::Context, AsRaw, GetContext};

use self::{
    base_type::BaseType,
    float::{FloatKind, FloatType},
    function::FunctionType,
    integer::IntegerType,
    pointer::PointerType,
};

#[macro_export]
macro_rules! impl_type_downcast {
    ($ty: ident<$lt: lifetime> -> Type::$variant: ident) => {
        impl<$lt> From<$ty<$lt>> for $crate::types::Type<$lt> {
            fn from(ty: $ty<$lt>) -> Self {
                $crate::types::Type::$variant(ty)
            }
        }
    };
}

macro_rules! unwrap_type {
    ($variant: ident($expected: ident)) => {
        impl<'ctx> From<$crate::types::Type<'ctx>> for $expected<'ctx> {
            fn from(value: $crate::types::Type<'ctx>) -> Self {
                match value {
                    $crate::types::Type::$variant(val) => val,
                    _ => panic!("bad value"),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum TypeKind {
    Void,
    Half,
    Float,
    Double,
    X86Fp80,
    Fp128,
    PpcFp128,
    Label,
    Integer,
    Function,
    Struct,
    Array,
    Pointer,
    Vector,
    Metadata,
    X86Mmx,
    Token,
    ScalableVector,
    BFloat,
    X86Amx,
    TargetExt,
}

impl From<LLVMTypeKind> for TypeKind {
    fn from(kind: LLVMTypeKind) -> Self {
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => Self::Void,
            LLVMTypeKind::LLVMHalfTypeKind => Self::Half,
            LLVMTypeKind::LLVMFloatTypeKind => Self::Float,
            LLVMTypeKind::LLVMDoubleTypeKind => Self::Double,
            LLVMTypeKind::LLVMX86_FP80TypeKind => Self::X86Fp80,
            LLVMTypeKind::LLVMFP128TypeKind => Self::Fp128,
            LLVMTypeKind::LLVMPPC_FP128TypeKind => Self::PpcFp128,
            LLVMTypeKind::LLVMLabelTypeKind => Self::Label,
            LLVMTypeKind::LLVMIntegerTypeKind => Self::Integer,
            LLVMTypeKind::LLVMFunctionTypeKind => Self::Function,
            LLVMTypeKind::LLVMStructTypeKind => Self::Struct,
            LLVMTypeKind::LLVMArrayTypeKind => Self::Array,
            LLVMTypeKind::LLVMPointerTypeKind => Self::Pointer,
            LLVMTypeKind::LLVMVectorTypeKind => Self::Vector,
            LLVMTypeKind::LLVMMetadataTypeKind => Self::Metadata,
            LLVMTypeKind::LLVMX86_MMXTypeKind => Self::X86Mmx,
            LLVMTypeKind::LLVMTokenTypeKind => Self::Token,
            LLVMTypeKind::LLVMScalableVectorTypeKind => Self::ScalableVector,
            LLVMTypeKind::LLVMBFloatTypeKind => Self::BFloat,
            LLVMTypeKind::LLVMX86_AMXTypeKind => Self::X86Amx,
            LLVMTypeKind::LLVMTargetExtTypeKind => Self::TargetExt,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type<'ctx> {
    Integer(IntegerType<'ctx>),
    Function(FunctionType<'ctx>),
    Float(FloatType<'ctx>),
    Pointer(PointerType<'ctx>),
}

unwrap_type!(Integer(IntegerType));
unwrap_type!(Function(FunctionType));
unwrap_type!(Float(FloatType));
unwrap_type!(Pointer(PointerType));

impl<'ctx> Type<'ctx> {
    pub fn print_to_string(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMPrintTypeToString(self.as_raw())) }
    }

    pub(crate) fn from_base_type(base_type: BaseType<'ctx>) -> Self {
        match base_type.kind() {
            TypeKind::Integer => Self::Integer(IntegerType(base_type)),
            TypeKind::Double => Self::Float(FloatType::new(base_type, FloatKind::Double)),
            TypeKind::Float => Self::Float(FloatType::new(base_type, FloatKind::Float)),
            TypeKind::Half => Self::Float(FloatType::new(base_type, FloatKind::Half)),
            TypeKind::Function => Self::Function(FunctionType(base_type)),
            TypeKind::Pointer => Self::Pointer(PointerType(base_type)),
            ty => panic!("unexpected {:?}", ty),
        }
    }
}

impl<'ctx> AsRaw for Type<'ctx> {
    type Raw = LLVMTypeRef;

    fn as_raw(&self) -> Self::Raw {
        match self {
            Self::Integer(int) => int.as_raw(),
            Self::Function(func) => func.as_raw(),
            Self::Float(float) => float.as_raw(),
            Self::Pointer(ptr) => ptr.as_raw(),
        }
    }
}

impl<'ctx> GetContext for Type<'ctx> {
    fn get_context(&self) -> &Context {
        match self {
            Type::Integer(ty) => ty.get_context(),
            Type::Function(ty) => ty.get_context(),
            Type::Float(ty) => ty.get_context(),
            Type::Pointer(ty) => ty.get_context(),
        }
    }
}
