#![allow(non_snake_case, clippy::missing_safety_doc)]

use llvm_sys::prelude::{LLVMAttributeRef, LLVMContextRef, LLVMValueRef};

pub unsafe fn LLVMAddFunctionAttributes(
    llfn: LLVMValueRef,
    idx: AttributeLocation,
    attr_ptr: *const LLVMAttributeRef,
    attr_len: usize,
) {
    LLVMRustAddFunctionAttributes(llfn, idx.into(), attr_ptr, attr_len);
}

pub unsafe fn LLVMAddCallSiteAttributes(
    llfn: LLVMValueRef,
    idx: AttributeLocation,
    attr_ptr: *const LLVMAttributeRef,
    attr_len: usize,
) {
    LLVMRustAddCallSiteAttributes(llfn, idx.into(), attr_ptr, attr_len);
}

pub enum AttributeLocation {
    Return,
    Argument(u32),
    Function,
}

impl From<AttributeLocation> for u32 {
    fn from(attribute_location: AttributeLocation) -> Self {
        match attribute_location {
            AttributeLocation::Return => 0,
            AttributeLocation::Argument(arg) => 1 + arg,
            AttributeLocation::Function => !0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum AttributeKind {
    AlwaysInline = 0,
    Cold = 1,
    Hot = 2,
    NoAlias = 3,
    NoCapture = 4,
    NoInline = 5,
    NoUnwind = 6,
    NoReturn = 7,
}

extern "C" {
    pub fn LLVMCreateAttribute(C: LLVMContextRef, attr: AttributeKind) -> LLVMAttributeRef;

    fn LLVMRustAddFunctionAttributes(
        Fn: LLVMValueRef,
        index: u32,
        Attrs: *const LLVMAttributeRef,
        AttrsLen: usize,
    );

    fn LLVMRustAddCallSiteAttributes(
        Instr: LLVMValueRef,
        index: u32,
        Attrs: *const LLVMAttributeRef,
        AttrsLen: usize,
    );
}
