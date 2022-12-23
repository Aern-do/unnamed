use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::value::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    Push(Value),
    Call(usize),
    Jump(usize),
    JumpIf(usize),
    Native(usize),
    Ret,
    Pop,
    Halt,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Push(operand) => write!(f, "push {}", operand),
            Instruction::Call(operand) => write!(f, "call {}", operand),
            Instruction::Jump(operand) => write!(f, "jump {}", operand),
            Instruction::JumpIf(operand) => write!(f, "jump_if {}", operand),
            Instruction::Native(operand) => write!(f, "native {}", operand),
            Instruction::Ret => write!(f, "ret"),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Halt => write!(f, "halt"),
            Instruction::Add => write!(f, "add"),
            Instruction::Sub => write!(f, "sub"),
            Instruction::Mul => write!(f, "mul"),
            Instruction::Div => write!(f, "div"),
            Instruction::Eq => write!(f, "eq"),
            Instruction::Lt => write!(f, "lt"),
            Instruction::Gt => write!(f, "gt"),
            Instruction::LtEq => write!(f, "lt_eq"),
            Instruction::GtEq => write!(f, "gt_eq"),
            Instruction::And => write!(f, "and"),
            Instruction::Or => write!(f, "or"),
        }
    }
}
