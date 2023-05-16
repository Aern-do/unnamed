

use llvm_sys::core::LLVMFunctionType;

use super::BaseType;

#[derive(Debug, Clone, Copy)]
pub struct Function<'ctx>(BaseType<'ctx>);

impl<'ctx> Function<'ctx> {
    pub fn create(params: &[BaseType<'ctx>], return_ty: BaseType<'ctx>) -> Self {
        unsafe {
            let mut params = params.iter().map(|param| param.into_raw()).collect::<Vec<_>>();
            let base_type_ref = LLVMFunctionType(
                return_ty.into_raw(),
                params.as_mut_ptr(),
                params.len() as u32,
                false as i32,
            );
            let base_type = BaseType::new(base_type_ref);
            Self(base_type)
        }
    }
}

impl<'ctx> From<Function<'ctx>> for BaseType<'ctx> {
    fn from(value: Function<'ctx>) -> Self {
        value.0
    }
}