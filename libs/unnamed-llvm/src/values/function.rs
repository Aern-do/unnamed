use crate::{
    attribute::{Attribute, AttributeLocation},
    extra::LLVMAddFunctionAttributes,
    impl_as_value_ref, impl_value_downcast,
};

use super::{AsValueRef, BaseValue};

pub struct FunctionValue<'ctx>(pub(crate) BaseValue<'ctx>);

impl<'ctx> FunctionValue<'ctx> {
    pub fn add_attributes(&self, attributes: &[Attribute<'ctx>]) {
        let attributes = attributes.iter().map(|attr| attr.as_attribute_ref()).collect::<Vec<_>>();
        unsafe {
            LLVMAddFunctionAttributes(
                self.as_value_ref(),
                AttributeLocation::Function,
                attributes.as_ptr(),
                attributes.len(),
            )
        }
    }

    pub fn add_argument_attributes(&self, idx: u32, attributes: &[Attribute<'ctx>]) {
        let attributes = attributes.iter().map(|attr| attr.as_attribute_ref()).collect::<Vec<_>>();
        unsafe {
            LLVMAddFunctionAttributes(
                self.as_value_ref(),
                AttributeLocation::Argument(idx),
                attributes.as_ptr(),
                attributes.len(),
            )
        }
    }
}

impl_as_value_ref!(FunctionValue);
impl_value_downcast!(Function);
