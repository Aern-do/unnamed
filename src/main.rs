#![forbid(missing_debug_implementations)]

use std::{
    env,
    io::{stdin, stdout, Write},
    result,
    time::Instant,
};

use lexer::{cursor::Cursor, Lexer};
use parser::{
    ast::{
        expression::{Expression, InfixOperator},
        Node,
    },
    expression::DEFAULT_SKIP_STATES,
    Parser,
};
use vm::{
    builder::{self, Builder},
    program::Program,
    ExecutionState, Machine, Result,
};

pub mod lexer;
pub mod parser;
pub mod shared;
pub mod vm;

fn build_program(input: String) -> result::Result<Program, builder::error::Error> {
    let cursor = Cursor::new(input.as_str());
    let lexer = Lexer::new(cursor);
    let tokens = lexer
        .collect::<result::Result<Vec<_>, _>>()
        .unwrap()
        .into_iter();
    let mut parser = Parser::new(parser::cursor::Cursor::new(tokens));
    let expression = parser.parse_expression(0, DEFAULT_SKIP_STATES).unwrap();
    let mut builder = Builder::with_default_natives();
    builder.procedure("main");
    compile(&mut builder, expression);
    builder.halt()?;
    Ok(builder.build())
}
pub fn compile(builder: &mut Builder, node: Node) {
    match node {
        Node::Integer(int) => builder.push(int.parse::<i32>().unwrap()).unwrap(),
        Node::Float(float) => builder.push(float.parse::<f64>().unwrap()).unwrap(),
        Node::Identifier(_) => panic!("Unknown variable"),
        Node::Expression(expression) => match expression {
            Expression::Infix { lhs, rhs, operator } => {
                compile(builder, *rhs);
                compile(builder, *lhs);
                match operator {
                    InfixOperator::Add => builder.add().unwrap(),
                    InfixOperator::Sub => builder.sub().unwrap(),
                    InfixOperator::Mul => builder.mul().unwrap(),
                    InfixOperator::Div => builder.div().unwrap(),
                    _ => panic!("Unsupported operator"),
                }
            }
            Expression::Prefix { .. } => panic!("Prefix expressions are not implemented"),
            Expression::Call { name, arguments } => match name {
                "print" => {
                    for argument in arguments {
                        compile(builder, argument)
                    }
                    builder.native("print").unwrap();
                }
                _ => panic!("Unsupported function"),
            },
        },
        Node::Statement(_) => panic!("Statements are not yet implemented"),
    };
}
fn main() {
    let debug = env::args().nth(1).is_some();

    println!("A simple compiler implementation for arithmetic expressions");
    println!("Supported functions: print");
    println!("To exit, type .exit");
    stdout().write_all(b"> ").unwrap();
    stdout().flush().unwrap();
    for line in stdin().lines() {
        let line = line.unwrap();
        if line == ".exit" {
            break;
        }
        let start = Instant::now();
        let program = build_program(line).unwrap();
        if debug {
            println!("Compiled in {:#?}", start.elapsed());
            println!("{}", program);
        }
        let mut machine = Machine::from_program(program, 128, 16);
        machine.load_native(print);
        let start = Instant::now();
        loop {
            match machine.execute_instruction() {
                Ok(ExecutionState::Running) => machine.increment_if_required(),
                Ok(ExecutionState::Done) => break,
                Err(err) => {
                    eprintln!("Virtual Machine Errored: {:?}", err);
                    break;
                }
            }
        }
        if debug {
            println!("Executed in {:#?}", start.elapsed());
        }
        stdout().write_all(b"> ").unwrap();
        stdout().flush().unwrap();
    }
}
fn print(machine: &mut Machine) -> Result<()> {
    println!("{}", machine.pop()?);
    Ok(())
}
