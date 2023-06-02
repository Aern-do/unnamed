use llvm_sys::{
    core::{
        LLVMContextCreate, LLVMCreateBuilderInContext, LLVMFunctionType, LLVMInt16TypeInContext,
        LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMInt64TypeInContext,
        LLVMInt8TypeInContext, LLVMIntTypeInContext, LLVMModuleCreateWithNameInContext,
        LLVMPointerTypeInContext,
    },
    prelude::LLVMContextRef,
};

use crate::{
    attribute::{Attribute, AttributeKind},
    builder::Builder,
    extra::LLVMCreateAttribute,
    impl_as_raw,
    module::Module,
    to_c_str,
    types::{
        base_type::BaseType, function::FunctionType, integer::IntegerType, pointer::PointerType,
        Type,
    },
    AsRaw, Assert, GetContext, True,
};
#[derive(Debug)]
pub struct Context(LLVMContextRef);

impl Context {
    pub fn create() -> Self {
        Self(unsafe { LLVMContextCreate() })
    }

    pub fn module(&self, name: &str) -> Module {
        let name = to_c_str(name);

        let module_ref = unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), self.as_raw()) };

        Module::new(self, module_ref)
    }

    pub fn builder(&self) -> Builder {
        unsafe {
            let builder_ref = LLVMCreateBuilderInContext(self.as_raw());
            Builder::new(self, builder_ref)
        }
    }

    pub fn attribute(&self, kind: AttributeKind) -> Attribute {
        let attribute_ref = unsafe { LLVMCreateAttribute(self.as_raw(), kind) };
        Attribute::new(self, attribute_ref)
    }

    pub fn int<const WIDTH: u32>(&self) -> IntegerType
    where
        Assert<{ WIDTH > 0 }>: True,
    {
        let type_ref = unsafe {
            match WIDTH {
                1 => LLVMInt1TypeInContext(self.as_raw()),
                8 => LLVMInt8TypeInContext(self.as_raw()),
                16 => LLVMInt16TypeInContext(self.as_raw()),
                32 => LLVMInt32TypeInContext(self.as_raw()),
                64 => LLVMInt64TypeInContext(self.as_raw()),
                width => LLVMIntTypeInContext(self.as_raw(), width),
            }
        };

        IntegerType(BaseType::new(self, type_ref))
    }

    pub fn function(&self, params: &[Type], return_ty: Type) -> FunctionType {
        let mut params = params.iter().map(|param| param.as_raw()).collect::<Vec<_>>();

        let function_ref = unsafe {
            LLVMFunctionType(return_ty.as_raw(), params.as_mut_ptr(), params.len() as u32, 0)
        };

        FunctionType(BaseType::new(self, function_ref))
    }

    pub fn pointer(&self, address_space: u32) -> PointerType {
        let pointer_ref = unsafe { LLVMPointerTypeInContext(self.as_raw(), address_space) };

        PointerType(BaseType::new(self, pointer_ref))
    }
}

impl GetContext for Context {
    fn get_context(&self) -> &Context {
        self
    }
}

impl_as_raw!(Context.0 -> LLVMContextRef);
