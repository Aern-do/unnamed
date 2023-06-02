use llvm_sys::{
    core::LLVMAddIncoming,
    prelude::{LLVMBasicBlockRef, LLVMValueRef},
};

use crate::{impl_as_raw, impl_get_context, impl_value_downcast, types::Type, AsRaw};

use super::{base_value::BaseValue, basic_block::BasicBlock, TypeOf, Value};

pub type Incomming<'ctx> = (Value<'ctx>, BasicBlock<'ctx>);

#[derive(Debug, Clone)]
pub struct PhiNodeValue<'ctx>(pub(crate) BaseValue<'ctx>);

impl<'ctx> PhiNodeValue<'ctx> {
    pub fn add_incomming(&self, incommings: &[Incomming<'ctx>]) {
        let (mut values, mut basic_blocks): (Vec<LLVMValueRef>, Vec<LLVMBasicBlockRef>) =
            incommings
                .iter()
                .map(|(value, basic_block)| (value.as_raw(), basic_block.as_raw()))
                .unzip();

        unsafe {
            LLVMAddIncoming(
                self.as_raw(),
                values.as_mut_ptr(),
                basic_blocks.as_mut_ptr(),
                values.len() as u32,
            );
        }
    }
}

impl<'ctx> TypeOf for PhiNodeValue<'ctx> {
    type Output<'ty> = Type<'ty>
    where
        Self: 'ty;

    fn type_of(&self) -> Self::Output<'_> {
        Type::from_base_type(self.0.type_of())
    }
}

impl_as_raw!(@downcast  PhiNodeValue<'ctx>.0 -> LLVMValueRef);
impl_value_downcast!(PhiNodeValue<'ctx> -> Value::PhiNode);
impl_get_context!(PhiNodeValue<'ctx>.0);
