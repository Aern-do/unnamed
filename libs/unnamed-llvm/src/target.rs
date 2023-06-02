use std::marker::PhantomData;

use llvm_sys::{
    target::{LLVMInitializeX86Target, LLVMInitializeX86TargetInfo, LLVMInitializeX86TargetMC},
    target_machine::{
        LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetMachine, LLVMGetFirstTarget,
        LLVMRelocMode, LLVMTargetMachineRef, LLVMTargetRef,
    },
};

use crate::{impl_as_raw, AsRaw};

#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    None,
    Less,
    Default,
    Aggressive,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Default
    }
}

impl From<OptimizationLevel> for LLVMCodeGenOptLevel {
    fn from(value: OptimizationLevel) -> Self {
        match value {
            OptimizationLevel::None => Self::LLVMCodeGenLevelNone,
            OptimizationLevel::Less => Self::LLVMCodeGenLevelLess,
            OptimizationLevel::Default => Self::LLVMCodeGenLevelDefault,
            OptimizationLevel::Aggressive => Self::LLVMCodeGenLevelAggressive,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CodeModel {
    Default,
    JitDefault,
    Tiny,
    Small,
    Kernel,
    Medium,
    Large,
}

impl Default for CodeModel {
    fn default() -> Self {
        Self::Default
    }
}

impl From<CodeModel> for LLVMCodeModel {
    fn from(value: CodeModel) -> Self {
        match value {
            CodeModel::Default => Self::LLVMCodeModelDefault,
            CodeModel::JitDefault => Self::LLVMCodeModelJITDefault,
            CodeModel::Tiny => Self::LLVMCodeModelTiny,
            CodeModel::Small => Self::LLVMCodeModelSmall,
            CodeModel::Kernel => Self::LLVMCodeModelKernel,
            CodeModel::Medium => Self::LLVMCodeModelMedium,
            CodeModel::Large => Self::LLVMCodeModelLarge,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RelocMode {
    Default,
    Static,
    Pic,
    DynamicNoPic,
    Ropi,
    Rwpi,
    RopiRwpi,
}

impl Default for RelocMode {
    fn default() -> Self {
        Self::Default
    }
}

impl From<RelocMode> for LLVMRelocMode {
    fn from(value: RelocMode) -> Self {
        match value {
            RelocMode::Default => Self::LLVMRelocDefault,
            RelocMode::Static => Self::LLVMRelocStatic,
            RelocMode::Pic => Self::LLVMRelocPIC,
            RelocMode::DynamicNoPic => Self::LLVMRelocDynamicNoPic,
            RelocMode::Ropi => Self::LLVMRelocROPI,
            RelocMode::Rwpi => Self::LLVMRelocRWPI,
            RelocMode::RopiRwpi => Self::LLVMRelocROPI_RWPI,
        }
    }
}

pub trait InitTarget {
    fn init();
}

macro_rules! create_target {
    ($name: ident : $($stage: ident),* $(,)? ) => {
        #[derive(Default)]
        pub struct $name;
        impl InitTarget for $name {
            fn init() {
                unsafe {
                    $($stage());*
                }
            }
        }
    };
}

create_target! { X86: LLVMInitializeX86Target, LLVMInitializeX86TargetInfo, LLVMInitializeX86TargetMC }

pub struct Target<I: InitTarget>(LLVMTargetRef, PhantomData<I>);

impl<I: InitTarget> Target<I> {
    pub fn initialize() -> Self {
        I::init();
        unsafe { Self(LLVMGetFirstTarget(), Default::default()) }
    }
}

impl<I: InitTarget> AsRaw for Target<I> {
    type Raw = LLVMTargetRef;

    fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

pub struct TargetMachine(LLVMTargetMachineRef);

impl TargetMachine {
    pub fn from_host<I: InitTarget>(
        target: Target<I>,
        opt_level: OptimizationLevel,
        reloc_mode: RelocMode,
        code_model: CodeModel,
    ) -> Self {
        unsafe {
            let triple = host::triple().as_ptr();
            let cpu = host::cpu().as_ptr();
            let cpu_features = host::cpu_features().as_ptr();

            Self(LLVMCreateTargetMachine(
                target.0,
                triple,
                cpu,
                cpu_features,
                opt_level.into(),
                reloc_mode.into(),
                code_model.into(),
            ))
        }
    }
}

impl_as_raw!(TargetMachine.0 -> LLVMTargetMachineRef);

pub mod host {
    use std::ffi::CStr;

    use llvm_sys::target_machine::{
        LLVMGetDefaultTargetTriple, LLVMGetHostCPUFeatures, LLVMGetHostCPUName,
    };

    pub fn triple<'llvm>() -> &'llvm CStr {
        unsafe {
            let ptr = LLVMGetDefaultTargetTriple();
            CStr::from_ptr(ptr)
        }
    }

    pub fn cpu<'llvm>() -> &'llvm CStr {
        unsafe {
            let ptr = LLVMGetHostCPUName();
            CStr::from_ptr(ptr)
        }
    }

    pub fn cpu_features<'llvm>() -> &'llvm CStr {
        unsafe {
            let ptr = LLVMGetHostCPUFeatures();
            CStr::from_ptr(ptr)
        }
    }
}
