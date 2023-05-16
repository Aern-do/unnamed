use std::marker::PhantomData;

use llvm_sys::{
    core::{
        LLVMBuildAdd, LLVMBuildCondBr, LLVMBuildMul, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildSub,
        LLVMCreateBuilderInContext, LLVMGetTypeKind, LLVMPositionBuilderAtEnd, LLVMTypeOf,
    },
    prelude::LLVMBuilderRef,
    LLVMTypeKind,
};

use crate::{
    context::Context,
    to_c_str,
    values::{basic_block::BasicBlockValue, BaseValue},
};

#[derive(Debug)]
pub struct Builder<'ctx> {
    inner: LLVMBuilderRef,
    _context: PhantomData<&'ctx ()>,
}

impl<'ctx> Builder<'ctx> {
    pub(crate) fn create(context: &Context) -> Self {
        Self {
            inner: unsafe { LLVMCreateBuilderInContext(context.into_raw()) },
            _context: Default::default(),
        }
    }
    pub fn at_end(&self, block: BasicBlockValue<'ctx>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.inner, block.inner);
        }
    }

    pub fn add(&self, lhs: BaseValue<'ctx>, rhs: BaseValue<'ctx>, name: &str) -> BaseValue<'ctx> {
        unsafe {
            BaseValue::new(LLVMBuildAdd(
                self.inner,
                lhs.into_raw(),
                rhs.into_raw(),
                to_c_str(name).as_ptr(),
            ))
        }
    }

    pub fn sub(&self, lhs: BaseValue<'ctx>, rhs: BaseValue<'ctx>, name: &str) -> BaseValue<'ctx> {
        unsafe {
            BaseValue::new(LLVMBuildSub(
                self.inner,
                lhs.into_raw(),
                rhs.into_raw(),
                to_c_str(name).as_ptr(),
            ))
        }
    }

    pub fn mul(&self, lhs: BaseValue<'ctx>, rhs: BaseValue<'ctx>, name: &str) -> BaseValue<'ctx> {
        unsafe {
            BaseValue::new(LLVMBuildMul(
                self.inner,
                lhs.into_raw(),
                rhs.into_raw(),
                to_c_str(name).as_ptr(),
            ))
        }
    }

    pub fn cond_br(
        &self,
        cond: BaseValue<'ctx>,
        then_br: BasicBlockValue<'ctx>,
        else_br: BasicBlockValue<'ctx>,
    ) -> BaseValue<'ctx> {
        unsafe { BaseValue::new(LLVMBuildCondBr(self.inner, cond.into_raw(), then_br.inner, else_br.inner)) }
    }

    pub fn ret(&self, value: BaseValue<'ctx>) {
        unsafe {
            if LLVMGetTypeKind(LLVMTypeOf(value.into_raw())) == LLVMTypeKind::LLVMVoidTypeKind {
                LLVMBuildRetVoid(self.inner);
            }
            LLVMBuildRet(self.inner, value.into_raw());
        }
    }
}
