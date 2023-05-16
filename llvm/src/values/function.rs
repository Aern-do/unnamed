use llvm_sys::core::{LLVMAppendBasicBlock, LLVMGetParam};

use crate::to_c_str;

use super::{basic_block::BasicBlockValue, BaseValue};

#[derive(Debug)]
pub struct FunctionValue<'ctx>(pub(crate) BaseValue<'ctx>);

impl<'ctx> FunctionValue<'ctx> {
    pub fn append_basic_block(&self, name: &str) -> BasicBlockValue<'ctx> {
        unsafe {
            let basic_block_ref = LLVMAppendBasicBlock(self.0.into_raw(), to_c_str(name).as_ptr());
            BasicBlockValue::new(basic_block_ref)
        }
    }
    pub fn get_param(&self, index: u32) -> BaseValue<'ctx> {
        unsafe { BaseValue::new(LLVMGetParam(self.0.into_raw(), index)) }
    }
}

impl<'ctx> From<FunctionValue<'ctx>> for BaseValue<'ctx> {
    fn from(value: FunctionValue<'ctx>) -> Self {
        value.0
    }
}
