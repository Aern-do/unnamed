use llvm_sys::{core::LLVMConstReal, prelude::LLVMTypeRef};

use crate::{
    impl_as_raw, impl_get_context, impl_type_downcast,
    values::{base_value::BaseValue, float::FloatValue},
    AsRaw, GetContext,
};

use super::BaseType;

#[derive(Debug, Clone, Copy)]
pub enum FloatKind {
    Half,
    Float,
    Double,
}

impl FloatKind {
    pub fn size(&self) -> u32 {
        match self {
            FloatKind::Half => 16,
            FloatKind::Float => 32,
            FloatKind::Double => 64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FloatType<'ctx> {
    base_type: BaseType<'ctx>,
    pub(crate) kind: FloatKind,
}

impl<'ctx> FloatType<'ctx> {
    pub(crate) fn new(base_type: BaseType<'ctx>, kind: FloatKind) -> Self {
        Self { base_type, kind }
    }

    pub fn size(&self) -> u32 {
        self.kind.size()
    }

    pub fn constant(&self, value: f64) -> FloatValue {
        let float_ref = unsafe { LLVMConstReal(self.as_raw(), value) };

        FloatValue::new(BaseValue::new(self.get_context(), float_ref), self.kind)
    }
}

impl_as_raw!(@downcast FloatType<'ctx>.base_type -> LLVMTypeRef);
impl_get_context!(FloatType<'ctx>.base_type);
impl_type_downcast!(FloatType<'ctx> -> Type::Float);
