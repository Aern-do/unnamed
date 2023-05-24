use llvm::{context::Context, extra::AttributeKind};

pub mod common;
pub mod lexer;
pub mod parser;

fn main() {
    let context = Context::create();
    let module = context.module("test");
    let func = context.function(&[context.int::<16>().into()], context.int::<1>().into());
    let func = module.add_function("test", func);

    let no_return = context.attribute(AttributeKind::NoReturn);
    let cold = context.attribute(AttributeKind::Cold);
    func.add_attributes(&[no_return, cold]);

    println!("{}", module.print_to_string().to_string_lossy());
}
