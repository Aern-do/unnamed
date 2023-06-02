pub(crate) mod base_value;
pub mod basic_block;
pub mod float;
pub mod function;
pub mod integer;
pub mod phi_node;

use llvm_sys::prelude::LLVMValueRef;

use crate::{
    context::Context,
    types::{float::FloatKind, Type, TypeKind},
    AsRaw, GetContext,
};

use self::{
    base_value::BaseValue, float::FloatValue, function::FunctionValue, integer::IntegerValue,
    phi_node::PhiNodeValue,
};

#[macro_export]
macro_rules! impl_value_downcast {
    ($val: ident$(<$lt: lifetime>)? -> Value::$variant: ident) => {
            impl$(<$lt>)? From<$val$(<$lt>)?> for $crate::values::Value$(<$lt>)? {
                fn from(val: $val$(<$lt>)?) -> Self {
                    $crate::values::Value::$variant(val)
                }
            }
    };
}

#[macro_export]
macro_rules! impl_type_of {
    ($value: ident $(<$lt: lifetime>)? -> $ty: ident) => {
        impl$(<$lt>)? $crate::values::TypeOf for $value$(<$lt>)? {
            type Output<'ty> = $ty<'ty> where Self: 'ty;
            fn type_of(&self) -> Self::Output<'_> {
                let type_ref = unsafe {
                    llvm_sys::core::LLVMTypeOf(self.as_raw())
                };

                $ty(
                    $crate::types::base_type::BaseType::new(self.0.get_context(), type_ref)
                )
            }
        }
    };
}

macro_rules! unwrap_value {
    ($variant: ident($expected: ident)) => {
        impl<'ctx> From<$crate::values::Value<'ctx>> for $expected<'ctx> {
            fn from(value: $crate::values::Value<'ctx>) -> Self {
                match value {
                    $crate::values::Value::$variant(val) => val,
                    _ => panic!("bad value"),
                }
            }
        }
    };
}

pub trait TypeOf {
    type Output<'ty>
    where
        Self: 'ty;

    fn type_of(&self) -> Self::Output<'_>;
}

#[derive(Debug, Clone)]
pub enum Value<'ctx> {
    Function(FunctionValue<'ctx>),
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    PhiNode(PhiNodeValue<'ctx>),
}

unwrap_value!(Function(FunctionValue));
unwrap_value!(Integer(IntegerValue));
unwrap_value!(Float(FloatValue));

impl<'ctx> Value<'ctx> {
    pub(crate) fn from_unknown(base_value: BaseValue<'ctx>) -> Self {
        let base_type = base_value.type_of();

        match base_type.kind() {
            TypeKind::Integer => Self::Integer(IntegerValue(base_value)),
            TypeKind::Double => Self::Float(FloatValue::new(base_value, FloatKind::Double)),
            TypeKind::Float => Self::Float(FloatValue::new(base_value, FloatKind::Float)),
            TypeKind::Half => Self::Float(FloatValue::new(base_value, FloatKind::Half)),
            TypeKind::Function => Self::Function(FunctionValue(base_value)),
            kind => panic!("unexpected {:?}", kind),
        }
    }
}

impl<'ctx> TypeOf for Value<'ctx> {
    type Output<'ty> = Type<'ty> where Self: 'ty;

    fn type_of(&self) -> Self::Output<'_> {
        match self {
            Value::Function(func) => Type::Function(func.type_of()),
            Value::Integer(int) => Type::Integer(int.type_of()),
            Value::Float(float) => Type::Float(float.type_of()),
            Value::PhiNode(node) => node.type_of(),
        }
    }
}

impl<'ctx> AsRaw for Value<'ctx> {
    type Raw = LLVMValueRef;

    fn as_raw(&self) -> Self::Raw {
        match self {
            Value::Function(value) => value.as_raw(),
            Value::Integer(value) => value.as_raw(),
            Value::Float(value) => value.as_raw(),
            Value::PhiNode(value) => value.as_raw(),
        }
    }
}

impl<'ctx> GetContext for Value<'ctx> {
    fn get_context(&self) -> &Context {
        match self {
            Value::Function(value) => value.get_context(),
            Value::Integer(value) => value.get_context(),
            Value::Float(value) => value.get_context(),
            Value::PhiNode(value) => value.get_context(),
        }
    }
}
