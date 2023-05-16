use std::{ffi::CStr, marker::PhantomData};

use llvm_sys::{
    core::{LLVMCloneModule, LLVMGetModuleIdentifier, LLVMModuleCreateWithNameInContext, LLVMPrintModuleToString, LLVMAddFunction},
    prelude::LLVMModuleRef,
};

use crate::{context::Context, to_c_str, types::{function::Function, BaseType}, values::{BaseValue, function::FunctionValue}};

pub struct Module<'ctx> {
    inner: LLVMModuleRef,
    _context: PhantomData<&'ctx ()>,
}

impl<'ctx> Module<'ctx> {
    pub(crate) unsafe fn new(inner: LLVMModuleRef) -> Self {
        Self { inner, _context: Default::default() }
    }

    pub fn create(name: &str, context: &Context) -> Self {
        unsafe {
            Module::new(LLVMModuleCreateWithNameInContext(
                to_c_str(name).as_ptr(),
                context.into_raw(),
            ))
        }
    }

    pub fn get_name(&self) -> &'ctx CStr {
        unsafe {
            let mut len = 0;
            let str = LLVMGetModuleIdentifier(self.inner, &mut len);
            CStr::from_ptr(str)
        }
    }
    
    pub fn print_to_string(&self) -> &'ctx CStr {
        unsafe {
            CStr::from_ptr(LLVMPrintModuleToString(self.inner))
        }
    }

    pub fn add_function(&self, name: &str, func: Function<'ctx>) -> FunctionValue<'ctx> {
        unsafe {
            let base_type: BaseType = func.into();
            let base_value_ref = LLVMAddFunction(self.inner, to_c_str(name).as_ptr(), base_type.into_raw());
            let base_value = BaseValue::new(base_value_ref);
            FunctionValue(base_value)
        }
    }
}

impl<'ctx> Clone for Module<'ctx> {
    fn clone(&self) -> Self {
        let cloned_ref = unsafe { LLVMCloneModule(self.inner) };
        Self { inner: cloned_ref, _context: Default::default() }
    }
}
