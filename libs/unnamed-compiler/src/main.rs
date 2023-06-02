use llvm::{
    builder::IntPredicate,
    context::Context,
    pass_manager::PassManagerOptions,
    target::{OptimizationLevel, Target, TargetMachine, X86},
};

pub mod common;
pub mod lexer;
pub mod parser;

fn main() {
    let target = Target::<X86>::initialize();
    let machine: TargetMachine = TargetMachine::from_host(
        target,
        OptimizationLevel::None,
        Default::default(),
        Default::default(),
    );

    let options = PassManagerOptions::create();
    options.set_debug_logging(true);
    options.set_verify_each(true);

    let context = Context::create();
    let module = context.module("test");
    let builder = context.builder();

    let int_32 = context.int::<32>();

    let add_ty = context.function(&[int_32.into(), int_32.into()], int_32.into());
    let add = module.add_function("add", add_ty);
    let add_entry = add.append_basic_block("entry");

    let main = context.function(&[], context.int::<32>().into());
    let main = module.add_function("main", main);
    let main_entry = main.append_basic_block("entry");

    builder.position_at_end(&add_entry);
    let lhs = add.param(0);
    let rhs = add.param(1);

    let result = builder.add(lhs, rhs, "add");
    builder.ret(result);

    builder.position_at_end(&main_entry);
    let lhs = builder.call(
        add_ty,
        &add,
        &[int_32.constant(5).into(), int_32.constant(5).into()],
        "call_lhs",
    );
    let rhs = builder.call(
        add_ty,
        &add,
        &[int_32.constant(5).into(), int_32.constant(5).into()],
        "call_rhs",
    );

    let result = builder.icmp(IntPredicate::EQ, lhs, rhs, "cmp");

    let true_br = main.append_basic_block("true_br");
    let false_br = main.append_basic_block("false_br");
    let end_br = main.append_basic_block("end_br");

    builder.cond_br(result, &true_br, &false_br);

    builder.position_at_end(&true_br);
    let a = int_32.constant(1488);
    builder.br(&end_br);

    builder.position_at_end(&false_br);
    let b = int_32.constant(666);
    builder.br(&end_br);

    builder.position_at_end(&end_br);

    let phi_node = builder.phi(int_32, "select");
    phi_node.add_incomming(&[(a.into(), true_br), (b.into(), false_br)]);

    builder.ret(phi_node);

    println!("{:?}", module.run_passes("verify", machine, options));
    println!("{}", module.print_to_string().to_string_lossy());
}
