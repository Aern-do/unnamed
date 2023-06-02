use std::ffi::CStr;

use llvm_sys::{
    core::{LLVMAddFunction, LLVMPrintModuleToString},
    prelude::LLVMModuleRef,
    transforms::pass_builder::LLVMRunPasses,
};

use crate::{
    context::Context,
    error::Error,
    impl_as_raw, impl_get_context,
    pass_manager::PassManagerOptions,
    target::TargetMachine,
    to_c_str,
    types::function::FunctionType,
    values::{base_value::BaseValue, function::FunctionValue},
    AsRaw,
};

pub struct Module<'ctx> {
    context: &'ctx Context,
    inner: LLVMModuleRef,
}

impl<'ctx> Module<'ctx> {
    pub(crate) fn new(_context: &'ctx Context, inner: LLVMModuleRef) -> Self {
        Self { context: _context, inner }
    }

    pub fn run_passes(
        &self,
        passes: &str,
        target_machine: TargetMachine,
        options: PassManagerOptions,
    ) -> Result<(), Error> {
        let passes = to_c_str(passes);

        let result_ref = unsafe {
            LLVMRunPasses(self.as_raw(), passes.as_ptr(), target_machine.as_raw(), options.0)
        };

        let error = Error(result_ref);
        if error.is_failure() {
            return Err(error);
        }

        Ok(())
    }

    pub fn add_function(&self, name: &str, function: FunctionType<'ctx>) -> FunctionValue<'ctx> {
        let name = to_c_str(name);

        let function_ref = unsafe { LLVMAddFunction(self.inner, name.as_ptr(), function.as_raw()) };

        FunctionValue(BaseValue::new(self.context, function_ref))
    }

    pub fn print_to_string(&self) -> &CStr {
        unsafe {
            let ptr = LLVMPrintModuleToString(self.inner);
            CStr::from_ptr(ptr)
        }
    }
}

impl_as_raw!(Module<'ctx>.inner -> LLVMModuleRef);
impl_get_context!(Module<'ctx>.context);
