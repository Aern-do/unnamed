use llvm_sys::{
    core::{LLVMAppendBasicBlockInContext, LLVMCountParams, LLVMGetParam, LLVMSetLinkage},
    prelude::LLVMValueRef,
    LLVMLinkage,
};

use crate::{
    attribute::{Attribute, AttributeLocation},
    extra::LLVMAddFunctionAttributes,
    impl_as_raw, impl_get_context, impl_type_of, impl_value_downcast, to_c_str,
    types::function::FunctionType,
    AsRaw, GetContext,
};

use super::{base_value::BaseValue, basic_block::BasicBlock, Value};

#[derive(Debug, PartialEq)]
pub enum Linkage {
    External,
    AvailableExternally,
    LinkOnceAny,
    LinkOnceODR,
    LinkOnceODRAutoHide,
    WeakAny,
    WeakODR,
    Appending,
    Internal,
    Private,
    DLLImport,
    DLLExport,
    ExternalWeak,
    Ghost,
    Common,
    LinkerPrivate,
    LinkerPrivateWeak,
}

impl From<Linkage> for LLVMLinkage {
    fn from(value: Linkage) -> Self {
        match value {
            Linkage::External => Self::LLVMExternalLinkage,
            Linkage::AvailableExternally => Self::LLVMAvailableExternallyLinkage,
            Linkage::LinkOnceAny => Self::LLVMLinkOnceAnyLinkage,
            Linkage::LinkOnceODR => Self::LLVMLinkOnceODRLinkage,
            Linkage::LinkOnceODRAutoHide => Self::LLVMLinkOnceODRAutoHideLinkage,
            Linkage::WeakAny => Self::LLVMWeakAnyLinkage,
            Linkage::WeakODR => Self::LLVMWeakODRLinkage,
            Linkage::Appending => Self::LLVMAppendingLinkage,
            Linkage::Internal => Self::LLVMInternalLinkage,
            Linkage::Private => Self::LLVMPrivateLinkage,
            Linkage::DLLImport => Self::LLVMDLLImportLinkage,
            Linkage::DLLExport => Self::LLVMDLLExportLinkage,
            Linkage::ExternalWeak => Self::LLVMExternalWeakLinkage,
            Linkage::Ghost => Self::LLVMGhostLinkage,
            Linkage::Common => Self::LLVMCommonLinkage,
            Linkage::LinkerPrivate => Self::LLVMLinkerPrivateLinkage,
            Linkage::LinkerPrivateWeak => Self::LLVMLinkerPrivateWeakLinkage,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionValue<'ctx>(pub(crate) BaseValue<'ctx>);

impl<'ctx> FunctionValue<'ctx> {
    fn add_any_attributes(&self, attributes: &[Attribute<'ctx>], location: AttributeLocation) {
        if let AttributeLocation::Argument(idx) = location {
            assert!(idx > self.param_count() - 1, "idx > param.count")
        }

        let attributes = attributes.iter().map(|attr| attr.as_raw()).collect::<Vec<_>>();

        unsafe {
            LLVMAddFunctionAttributes(
                self.as_raw(),
                location,
                attributes.as_ptr(),
                attributes.len(),
            )
        }
    }

    pub fn set_linkage(&self, linkage: Linkage) {
        unsafe { LLVMSetLinkage(self.as_raw(), linkage.into()) }
    }

    pub fn param(&self, idx: u32) -> Value {
        let param_ref = unsafe { LLVMGetParam(self.as_raw(), idx) };

        Value::from_unknown(BaseValue::new(self.get_context(), param_ref))
    }

    pub fn append_basic_block(&self, name: &str) -> BasicBlock {
        let name = to_c_str(name);

        let basic_block_ref = unsafe {
            LLVMAppendBasicBlockInContext(
                self.0.get_context().as_raw(),
                self.as_raw(),
                name.as_ptr(),
            )
        };

        BasicBlock::new(self.get_context(), basic_block_ref)
    }

    pub fn param_count(&self) -> u32 {
        unsafe { LLVMCountParams(self.as_raw()) }
    }

    pub fn add_attributes(&self, attributes: &[Attribute<'ctx>]) {
        self.add_any_attributes(attributes, AttributeLocation::Function)
    }

    pub fn add_argument_attributes(&self, idx: u32, attributes: &[Attribute<'ctx>]) {
        self.add_any_attributes(attributes, AttributeLocation::Argument(idx))
    }
}

impl_type_of!(FunctionValue<'ctx> -> FunctionType);
impl_as_raw!(@downcast FunctionValue<'ctx>.0 -> LLVMValueRef);
impl_get_context!(FunctionValue<'ctx>.0);
impl_value_downcast!(FunctionValue<'ctx> -> Value::Function);
