use llvm_sys::{
    core::{
        LLVMContextCreate, LLVMCreateBuilderInContext, LLVMFunctionType, LLVMInt16TypeInContext,
        LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMInt64TypeInContext,
        LLVMInt8TypeInContext, LLVMIntTypeInContext, LLVMModuleCreateWithNameInContext,
    },
    prelude::LLVMContextRef,
};

use crate::{
    attribute::{Attribute, AttributeKind},
    builder::Builder,
    module::Module,
    to_c_str,
    types::{function::FunctionType, integer::IntegerType, AsTypeRef, BaseType, Type},
    Assert, True,
};
#[derive(Debug)]
pub struct Context(LLVMContextRef);

impl Context {
    pub fn create() -> Self {
        Self(unsafe { LLVMContextCreate() })
    }

    pub fn as_context_ref(&self) -> LLVMContextRef {
        self.0
    }

    pub fn module(&self, name: &str) -> Module {
        unsafe {
            let name = to_c_str(name);
            let module_ref =
                LLVMModuleCreateWithNameInContext(name.as_ptr(), self.as_context_ref());
            Module::new(self, module_ref)
        }
    }

    pub fn builder(&self) -> Builder {
        unsafe {
            let builder_ref = LLVMCreateBuilderInContext(self.as_context_ref());
            Builder::new(self, builder_ref)
        }
    }

    pub fn attribute(&self, kind: AttributeKind) -> Attribute {
        Attribute::create(self, kind)
    }

    pub fn int<const WIDTH: u32>(&self) -> IntegerType
    where
        Assert<{ WIDTH > 0 }>: True,
    {
        unsafe {
            let type_ref = match WIDTH {
                1 => LLVMInt1TypeInContext(self.as_context_ref()),
                8 => LLVMInt8TypeInContext(self.as_context_ref()),
                16 => LLVMInt16TypeInContext(self.as_context_ref()),
                32 => LLVMInt32TypeInContext(self.as_context_ref()),
                64 => LLVMInt64TypeInContext(self.as_context_ref()),
                width => LLVMIntTypeInContext(self.as_context_ref(), width),
            };
            let base_type = BaseType::new(self, type_ref);

            IntegerType(base_type)
        }
    }

    pub fn function(&self, params: &[Type], return_ty: Type) -> FunctionType {
        unsafe {
            let mut params = params.iter().map(|param| param.as_type_ref()).collect::<Vec<_>>();
            let type_ref = LLVMFunctionType(
                return_ty.as_type_ref(),
                params.as_mut_ptr(),
                params.len() as u32,
                0,
            );
            let base_type = BaseType::new(self, type_ref);

            FunctionType(base_type)
        }
    }
}
