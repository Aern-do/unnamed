use llvm_sys::transforms::pass_builder::{
    LLVMCreatePassBuilderOptions, LLVMPassBuilderOptionsRef,
    LLVMPassBuilderOptionsSetCallGraphProfile, LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll,
    LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap, LLVMPassBuilderOptionsSetLicmMssaOptCap,
    LLVMPassBuilderOptionsSetLoopInterleaving, LLVMPassBuilderOptionsSetLoopUnrolling,
    LLVMPassBuilderOptionsSetLoopVectorization, LLVMPassBuilderOptionsSetMergeFunctions,
    LLVMPassBuilderOptionsSetSLPVectorization, LLVMPassBuilderOptionsSetVerifyEach,
};

macro_rules! option {
    ($name: ident($arg: ident: $ty: ty) -> $func: ident) => {
        pub fn $name(&self, $arg: $ty) {
            unsafe { $func(self.0, $arg) }
        }
    };
    ($name: ident($arg: ident: $ty: ty) -> $func: ident($transform: expr)) => {
        pub fn $name(&self, $arg: $ty) {
            unsafe { $func(self.0, $transform) }
        }
    };
}

pub struct PassManagerOptions(pub LLVMPassBuilderOptionsRef);

impl PassManagerOptions {
    pub fn create() -> Self {
        Self(unsafe { LLVMCreatePassBuilderOptions() })
    }

    option!(set_call_graph_profile(call_graph: bool) -> LLVMPassBuilderOptionsSetCallGraphProfile(call_graph as i32));
    option!(set_debug_logging(debug_logging: bool) -> LLVMPassBuilderOptionsSetCallGraphProfile(debug_logging as i32));
    option!(set_forget_all_scev_in_loop_unroll(forget_scev: bool) -> LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(forget_scev as i32));
    option!(set_licm_mssa_no_acc_for_promotion_cap(cap: u32) -> LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(cap));
    option!(set_licm_mssa_opt_cap(cap: u32) -> LLVMPassBuilderOptionsSetLicmMssaOptCap(cap));
    option!(set_loop_interleaving(enable: bool) -> LLVMPassBuilderOptionsSetLoopInterleaving(enable as i32));
    option!(set_loop_unrolling(enable: bool) -> LLVMPassBuilderOptionsSetLoopUnrolling(enable as i32));
    option!(set_loop_vectorization(enable: bool) -> LLVMPassBuilderOptionsSetLoopVectorization(enable as i32));
    option!(set_merge_functions(enable: bool) -> LLVMPassBuilderOptionsSetMergeFunctions(enable as i32));
    option!(set_slp_vectorization(enable: bool) -> LLVMPassBuilderOptionsSetSLPVectorization(enable as i32));
    option!(set_verify_each(enable: bool) -> LLVMPassBuilderOptionsSetVerifyEach(enable as i32));
}
