use std::fmt::{self, Display};

use llvm_sys::core::{
    LLVMInt128TypeInContext, LLVMInt16TypeInContext, LLVMInt1TypeInContext,
    LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMInt8TypeInContext, LLVMIntTypeInContext, LLVMConstInt,
};

use crate::{context::Context, values::BaseValue};

use super::BaseType;

pub struct Assert<const EXPR: bool>;
pub trait True {}
impl True for Assert<true> {}

#[derive(Debug, Clone, Copy)]
pub struct IntegerType<'ctx, const WIDTH: u32>(BaseType<'ctx>);

impl<'ctx, const WIDTH: u32> Display for IntegerType<'ctx, WIDTH> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i{WIDTH}")
    }
}

impl<'ctx, const WIDTH: u32> IntegerType<'ctx, WIDTH>
where
    Assert<{ WIDTH > 0 }>: True,
{
    pub fn create(context: &'ctx Context) -> Self {
        unsafe {
            let base_type_ref = match WIDTH {
                1 => LLVMInt1TypeInContext(context.into_raw()),
                8 => LLVMInt8TypeInContext(context.into_raw()),
                16 => LLVMInt16TypeInContext(context.into_raw()),
                32 => LLVMInt32TypeInContext(context.into_raw()),
                64 => LLVMInt64TypeInContext(context.into_raw()),
                128 => LLVMInt128TypeInContext(context.into_raw()),
                bits => LLVMIntTypeInContext(context.into_raw(), bits),
            };
            let base_type = BaseType::new(base_type_ref);
            Self(base_type)
        }
    }

    pub fn make_const(&self, value: u64) -> BaseValue<'ctx> {
        unsafe {
            let int_const = LLVMConstInt(self.0.into_raw(), value, 0);
            BaseValue::new(int_const)
        }
    }
}

impl<'ctx, const WIDTH: u32> From<IntegerType<'ctx, WIDTH>> for BaseType<'ctx> {
    fn from(value: IntegerType<'ctx, WIDTH>) -> Self {
        value.0
    }
}

pub type Boolean<'ctx> = IntegerType<'ctx, 1>;
pub type Int8<'ctx> = IntegerType<'ctx, 8>;
pub type Int16<'ctx> = IntegerType<'ctx, 16>;
pub type Int32<'ctx> = IntegerType<'ctx, 32>;
pub type Int64<'ctx> = IntegerType<'ctx, 64>;
pub type Int128<'ctx> = IntegerType<'ctx, 128>;

pub trait GenericInt<'ctx> {
    
}