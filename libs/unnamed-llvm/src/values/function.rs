use llvm_sys::core::{LLVMCountParams, LLVMAppendBasicBlockInContext};

use crate::{
    attribute::{Attribute, AttributeLocation},
    extra::LLVMAddFunctionAttributes,
    impl_as_value_ref, impl_value_downcast, to_c_str,
};

use super::{AsValueRef, BaseValue, basic_block::BasicBlock};

pub struct FunctionValue<'ctx>(pub(crate) BaseValue<'ctx>);

impl<'ctx> FunctionValue<'ctx> {
    fn add_any_attributes(&self, attributes: &[Attribute<'ctx>], location: AttributeLocation) {
        if let AttributeLocation::Argument(idx) = location {
            if idx > self.param_count() - 1 {
                panic!("idx > param_count")
            }
        }

        let attributes = attributes.iter().map(|attr| attr.as_attribute_ref()).collect::<Vec<_>>();
        
        unsafe {
            LLVMAddFunctionAttributes(
                self.as_value_ref(),
                location,
                attributes.as_ptr(),
                attributes.len(),
            )
        }
    }

    pub fn append_basic_block(&self, name: &str) -> BasicBlock<'ctx> {
        unsafe {
            let name = to_c_str(name);
            let basic_block_ref = LLVMAppendBasicBlockInContext(self.0.get_context().as_context_ref(), self.as_value_ref(), name.as_ptr());
            BasicBlock::new(self.0.get_context(), basic_block_ref)
        }
    }

    pub fn param_count(&self) -> u32 {
        unsafe {
            LLVMCountParams(self.as_value_ref())
        }
    }

    pub fn add_attributes(&self, attributes: &[Attribute<'ctx>]) {
        self.add_any_attributes(attributes, AttributeLocation::Function)
    }

    pub fn add_argument_attributes(&self, idx: u32, attributes: &[Attribute<'ctx>]) {
        self.add_any_attributes(attributes, AttributeLocation::Argument(idx))
    }
}

impl_as_value_ref!(FunctionValue);
impl_value_downcast!(Function);
