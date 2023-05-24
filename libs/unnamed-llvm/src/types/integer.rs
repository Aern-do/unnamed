use llvm_sys::core::LLVMGetIntTypeWidth;

use crate::{impl_as_type_ref, impl_type_downcast};

use super::{AsTypeRef, BaseType};

#[derive(Debug, Clone, Copy)]
pub struct IntegerType<'ctx>(pub(crate) BaseType<'ctx>);

impl<'ctx> IntegerType<'ctx> {
    pub fn width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.0.as_type_ref()) }
    }
}

impl_as_type_ref!(IntegerType);
impl_type_downcast!(Integer);
