use std::ffi::CStr;

use llvm_sys::{
    core::{LLVMAddFunction, LLVMPrintModuleToString},
    prelude::LLVMModuleRef,
};

use crate::{
    context::Context,
    to_c_str,
    types::{function::FunctionType, AsTypeRef},
    values::{function::FunctionValue, BaseValue},
};

pub struct Module<'ctx> {
    context: &'ctx Context,
    inner: LLVMModuleRef,
}

impl<'ctx> Module<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, inner: LLVMModuleRef) -> Self {
        Self { context: _context, inner }
    }

    pub fn add_function(&self, name: &str, function: FunctionType<'ctx>) -> FunctionValue<'ctx> {
        let name = to_c_str(name);
        unsafe {
            let inner = LLVMAddFunction(self.inner, name.as_ptr(), function.as_type_ref());
            let base_value = BaseValue::new(self.context, inner);
            FunctionValue(base_value)
        }
    }

    pub fn print_to_string(&self) -> &CStr {
        unsafe {
            let ptr = LLVMPrintModuleToString(self.inner);
            CStr::from_ptr(ptr)
        }
    }
}
