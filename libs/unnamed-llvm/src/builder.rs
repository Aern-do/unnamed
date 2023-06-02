use llvm_sys::{
    core::{
        LLVMBuildAdd, LLVMBuildBr, LLVMBuildCall2, LLVMBuildCondBr, LLVMBuildFAdd, LLVMBuildFCmp,
        LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFPToSI, LLVMBuildFPToUI, LLVMBuildFPTrunc,
        LLVMBuildFSub, LLVMBuildICmp, LLVMBuildMul, LLVMBuildPhi, LLVMBuildRet, LLVMBuildSDiv,
        LLVMBuildSIToFP, LLVMBuildSub, LLVMBuildTrunc, LLVMBuildUDiv, LLVMBuildUIToFP,
        LLVMBuildZExt, LLVMPositionBuilderAtEnd,
    },
    prelude::LLVMBuilderRef,
    LLVMIntPredicate, LLVMRealPredicate,
};

use crate::{
    context::Context,
    impl_as_raw, impl_get_context, to_c_str,
    types::{float::FloatType, function::FunctionType, integer::IntegerType, Type},
    values::{
        base_value::BaseValue, basic_block::BasicBlock, float::FloatValue, function::FunctionValue,
        integer::IntegerValue, phi_node::PhiNodeValue, TypeOf, Value,
    },
    AsRaw, GetContext,
};

macro_rules! int_op {
    ($name: ident($op: ident)) => {
        pub fn $name<L, R>(&self, lhs: L, rhs: R, name: &str) -> IntegerValue
        where
            IntegerValue<'ctx>: From<L>,
            IntegerValue<'ctx>: From<R>,
        {
            let lhs = IntegerValue::from(lhs);
            let rhs = IntegerValue::from(rhs);

            let name = to_c_str(name);

            let int_ref = unsafe { $op(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) };

            IntegerValue(BaseValue::new(self.get_context(), int_ref))
        }
    };
}

macro_rules! float_op {
    ($name: ident($op: ident)) => {
        pub fn $name<L, R>(&self, lhs: L, rhs: R, name: &str) -> FloatValue
        where
            FloatValue<'ctx>: From<L>,
            FloatValue<'ctx>: From<R>,
        {
            let lhs = FloatValue::from(lhs);
            let rhs = FloatValue::from(rhs);

            let name = to_c_str(name);

            let float_ref =
                unsafe { $op(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) };

            FloatValue::new(BaseValue::new(self.get_context(), float_ref), rhs.kind)
        }
    };
}

macro_rules! int_cast {
    ($name: ident($cast: ident); assert(|$value: ident, $ty: ident| $assert: expr)) => {
        pub fn $name<V>(&self, $value: V, $ty: IntegerType<'ctx>, name: &str) -> IntegerValue
        where
            IntegerValue<'ctx>: From<V>,
        {
            let $value: IntegerValue<'ctx> = $value.into();
            let name = to_c_str(name);

            assert!($assert);

            let int_ref =
                unsafe { $cast(self.as_raw(), $value.as_raw(), $ty.as_raw(), name.as_ptr()) };

            IntegerValue(BaseValue::new(self.get_context(), int_ref))
        }
    };
}

macro_rules! float_cast {
    ($name: ident($cast: ident); assert(|$value: ident, $ty: ident| $assert: expr)) => {
        pub fn $name<V>(&self, $value: V, $ty: FloatType<'ctx>, name: &str) -> FloatValue
        where
            FloatValue<'ctx>: From<V>,
        {
            let $value = FloatValue::from($value);
            let name = to_c_str(name);

            assert!($assert);

            let float_ref =
                unsafe { $cast(self.as_raw(), $value.as_raw(), $ty.as_raw(), name.as_ptr()) };

            FloatValue::new(BaseValue::new(self.get_context(), float_ref), $ty.kind)
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntPredicate {
    EQ,
    NE,
    UGT,
    UGE,
    ULT,
    ULE,
    SGT,
    SGE,
    SLT,
    SLE,
}

impl From<IntPredicate> for LLVMIntPredicate {
    fn from(predicate: IntPredicate) -> Self {
        match predicate {
            IntPredicate::EQ => LLVMIntPredicate::LLVMIntEQ,
            IntPredicate::NE => LLVMIntPredicate::LLVMIntNE,
            IntPredicate::UGT => LLVMIntPredicate::LLVMIntUGT,
            IntPredicate::UGE => LLVMIntPredicate::LLVMIntUGE,
            IntPredicate::ULT => LLVMIntPredicate::LLVMIntULT,
            IntPredicate::ULE => LLVMIntPredicate::LLVMIntULE,
            IntPredicate::SGT => LLVMIntPredicate::LLVMIntSGT,
            IntPredicate::SGE => LLVMIntPredicate::LLVMIntSGE,
            IntPredicate::SLT => LLVMIntPredicate::LLVMIntSLT,
            IntPredicate::SLE => LLVMIntPredicate::LLVMIntSLE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatPredicate {
    False,
    OEQ,
    OGT,
    OGE,
    OLT,
    OLE,
    ONE,
    ORD,
    UNO,
    UEQ,
    UGT,
    UGE,
    ULT,
    ULE,
    UNE,
    True,
}

impl From<FloatPredicate> for LLVMRealPredicate {
    fn from(predicate: FloatPredicate) -> Self {
        match predicate {
            FloatPredicate::False => LLVMRealPredicate::LLVMRealPredicateFalse,
            FloatPredicate::OEQ => LLVMRealPredicate::LLVMRealOEQ,
            FloatPredicate::OGT => LLVMRealPredicate::LLVMRealOGT,
            FloatPredicate::OGE => LLVMRealPredicate::LLVMRealOGE,
            FloatPredicate::OLT => LLVMRealPredicate::LLVMRealOLT,
            FloatPredicate::OLE => LLVMRealPredicate::LLVMRealOLE,
            FloatPredicate::ONE => LLVMRealPredicate::LLVMRealONE,
            FloatPredicate::ORD => LLVMRealPredicate::LLVMRealORD,
            FloatPredicate::UNO => LLVMRealPredicate::LLVMRealUNO,
            FloatPredicate::UEQ => LLVMRealPredicate::LLVMRealUEQ,
            FloatPredicate::UGT => LLVMRealPredicate::LLVMRealUGT,
            FloatPredicate::UGE => LLVMRealPredicate::LLVMRealUGE,
            FloatPredicate::ULT => LLVMRealPredicate::LLVMRealULT,
            FloatPredicate::ULE => LLVMRealPredicate::LLVMRealULE,
            FloatPredicate::UNE => LLVMRealPredicate::LLVMRealUNE,
            FloatPredicate::True => LLVMRealPredicate::LLVMRealPredicateTrue,
        }
    }
}

#[derive(Debug)]
pub struct Builder<'ctx> {
    context: &'ctx Context,
    inner: LLVMBuilderRef,
}

impl<'ctx> Builder<'ctx> {
    pub(crate) fn new(context: &'ctx Context, inner: LLVMBuilderRef) -> Self {
        Self { context, inner }
    }

    pub fn position_at_end(&self, basic_block: &BasicBlock<'ctx>) {
        unsafe { LLVMPositionBuilderAtEnd(self.as_raw(), basic_block.as_raw()) }
    }

    int_op!(add(LLVMBuildAdd));
    int_op!(sub(LLVMBuildSub));
    int_op!(mul(LLVMBuildMul));
    int_op!(signed_div(LLVMBuildSDiv));
    int_op!(unsigned_div(LLVMBuildUDiv));

    float_op!(float_add(LLVMBuildFAdd));
    float_op!(float_sub(LLVMBuildFSub));
    float_op!(float_mul(LLVMBuildFMul));
    float_op!(float_div(LLVMBuildFDiv));

    int_cast!(trunc(LLVMBuildTrunc); assert(|value, ty| value.type_of().width() > ty.width()));
    int_cast!(zero_extend(LLVMBuildZExt); assert(|value, ty| value.type_of().width() < ty.width()));
    int_cast!(sign_extend(LLVMBuildZExt); assert(|value, ty| value.type_of().width() < ty.width()));

    float_cast!(float_trunc(LLVMBuildFPTrunc); assert(|value, ty| value.type_of().size() > ty.size()));
    float_cast!(float_extend(LLVMBuildFPTrunc); assert(|value, ty| value.type_of().size() < ty.size()));

    pub fn float_to_signed_int<F>(
        &self,
        float: F,
        ty: IntegerType<'ctx>,
        name: &str,
    ) -> IntegerValue
    where
        FloatValue<'ctx>: From<F>,
    {
        let float = FloatValue::from(float);
        let name = to_c_str(name);

        let int_value_ref =
            unsafe { LLVMBuildFPToSI(self.as_raw(), float.as_raw(), ty.as_raw(), name.as_ptr()) };

        IntegerValue(BaseValue::new(self.get_context(), int_value_ref))
    }

    pub fn float_to_unsigned_int<F>(
        &self,
        float: F,
        ty: IntegerType<'ctx>,
        name: &str,
    ) -> IntegerValue
    where
        FloatValue<'ctx>: From<F>,
    {
        let float = FloatValue::from(float);
        let name = to_c_str(name);

        let int_value_ref =
            unsafe { LLVMBuildFPToUI(self.as_raw(), float.as_raw(), ty.as_raw(), name.as_ptr()) };

        IntegerValue(BaseValue::new(self.get_context(), int_value_ref))
    }

    pub fn signed_int_to_float<I>(&self, integer: I, ty: FloatType<'ctx>, name: &str) -> FloatValue
    where
        IntegerValue<'ctx>: From<I>,
    {
        let float: IntegerValue<'ctx> = integer.into();
        let name = to_c_str(name);

        let float_value_ref =
            unsafe { LLVMBuildSIToFP(self.as_raw(), float.as_raw(), ty.as_raw(), name.as_ptr()) };

        FloatValue::new(BaseValue::new(self.get_context(), float_value_ref), ty.kind)
    }

    pub fn unsigned_int_to_float<I>(
        &self,
        integer: I,
        ty: FloatType<'ctx>,
        name: &str,
    ) -> FloatValue
    where
        IntegerValue<'ctx>: From<I>,
    {
        let integer = IntegerValue::from(integer);
        let name = to_c_str(name);

        let float_value_ref =
            unsafe { LLVMBuildUIToFP(self.as_raw(), integer.as_raw(), ty.as_raw(), name.as_ptr()) };

        FloatValue::new(BaseValue::new(self.get_context(), float_value_ref), ty.kind)
    }

    pub fn phi<T>(&self, ty: T, name: &str) -> PhiNodeValue
    where
        Type<'ctx>: From<T>,
    {
        let ty = Type::from(ty);
        let str = to_c_str(name);

        let phi_node_ref = unsafe { LLVMBuildPhi(self.as_raw(), ty.as_raw(), str.as_ptr()) };

        PhiNodeValue(BaseValue::new(self.get_context(), phi_node_ref))
    }

    pub fn icmp<L, R>(&self, op: IntPredicate, lhs: L, rhs: R, name: &str) -> IntegerValue
    where
        IntegerValue<'ctx>: From<L>,
        IntegerValue<'ctx>: From<R>,
    {
        let lhs = IntegerValue::from(lhs);
        let rhs = IntegerValue::from(rhs);

        let name = to_c_str(name);

        let bool_ref = unsafe {
            LLVMBuildICmp(self.as_raw(), op.into(), lhs.as_raw(), rhs.as_raw(), name.as_ptr())
        };

        IntegerValue(BaseValue::new(self.get_context(), bool_ref))
    }

    pub fn fcmp<L, R>(&self, op: FloatPredicate, lhs: L, rhs: R, name: &str) -> IntegerValue
    where
        FloatValue<'ctx>: From<L>,
        FloatValue<'ctx>: From<R>,
    {
        let lhs = FloatValue::from(lhs);
        let rhs = FloatValue::from(rhs);

        let name = to_c_str(name);

        let bool_ref = unsafe {
            LLVMBuildFCmp(self.as_raw(), op.into(), lhs.as_raw(), rhs.as_raw(), name.as_ptr())
        };

        IntegerValue(BaseValue::new(self.get_context(), bool_ref))
    }

    pub fn ret<V: Into<Value<'ctx>>>(&self, value: V) {
        let value: Value<'ctx> = value.into();
        unsafe {
            LLVMBuildRet(self.as_raw(), value.as_raw());
        }
    }

    pub fn cond_br<V>(&self, value: V, then_br: &BasicBlock<'ctx>, else_br: &BasicBlock<'ctx>)
    where
        Value<'ctx>: From<V>,
    {
        let value = Value::from(value);

        unsafe {
            LLVMBuildCondBr(self.as_raw(), value.as_raw(), then_br.as_raw(), else_br.as_raw());
        }
    }

    pub fn br(&self, to: &BasicBlock) {
        unsafe {
            LLVMBuildBr(self.as_raw(), to.as_raw());
        }
    }

    pub fn call(
        &self,
        func_ty: FunctionType<'ctx>,
        func: &FunctionValue<'ctx>,
        args: &[Value<'ctx>],
        name: &str,
    ) -> Value {
        let name = to_c_str(name);
        let mut args = args.iter().map(|arg| arg.as_raw()).collect::<Vec<_>>();

        let value_ref = unsafe {
            LLVMBuildCall2(
                self.as_raw(),
                func_ty.as_raw(),
                func.as_raw(),
                args.as_mut_ptr(),
                args.len() as u32,
                name.as_ptr(),
            )
        };

        Value::from_unknown(BaseValue::new(self.get_context(), value_ref))
    }
}

impl_as_raw!(Builder<'ctx>.inner -> LLVMBuilderRef);
impl_get_context!(Builder<'ctx>.context);
