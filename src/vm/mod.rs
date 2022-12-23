use std::{fmt::Debug, result};

use self::{error::Error, instruction::Instruction, program::Program, value::Value};

pub mod builder;
pub mod error;
pub mod instruction;
pub mod program;
pub mod value;

macro_rules! operator {
    ($self: ident; $method: ident) => {{
        let lhs = $self.pop()?;
        let rhs = $self.pop()?;
        let result = lhs.$method(rhs)?;
        $self.stack.push(result);
    }};
}
pub type Native = fn(&mut Machine) -> Result<()>;

#[derive(Clone, Debug)]
pub enum ExecutionState {
    Done,
    Running,
}
pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug, Default)]
pub struct Frame {
    pub return_ip: usize,
    pub locals: Vec<Value>,
}

impl Frame {
    pub fn new(return_ip: usize) -> Self {
        Self {
            return_ip,
            locals: Vec::new(),
        }
    }
}

pub struct Machine {
    instructions: Vec<Instruction>,
    markers: Vec<usize>,
    natives: Vec<Native>,
    stack: Vec<Value>,
    frames: Vec<Frame>,
    ip: usize,
    pub halted: bool,
    pending_increment: bool,
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field("instructions", &self.instructions)
            .field("stack", &self.stack)
            .field("frames", &self.frames)
            .field("ip", &self.ip)
            .field("halted", &self.halted)
            .field("pending_increment", &self.pending_increment)
            .finish()
    }
}

impl Machine {
    pub fn new(
        instructions: Vec<Instruction>,
        markers: Vec<usize>,
        stack: usize,
        frames: usize,
    ) -> Self {
        Self {
            instructions,
            markers,
            natives: Vec::new(),
            stack: Vec::with_capacity(stack),
            frames: Vec::with_capacity(frames),
            halted: false,
            ip: 0,
            pending_increment: true,
        }
    }
    pub fn from_program(
        Program {
            instructions,
            procedures: markers,
        }: Program,
        stack: usize,
        frames: usize,
    ) -> Self {
        Self {
            instructions,
            markers,
            stack: Vec::with_capacity(stack),
            frames: Vec::with_capacity(frames),
            pending_increment: true,
            natives: Default::default(),
            ip: Default::default(),
            halted: Default::default(),
        }
    }
    pub fn create_frame(&mut self, frame: Frame) {
        self.frames.push(frame)
    }
    pub fn set_ip(&mut self, ip: usize) {
        self.ip = ip;
    }
    pub fn load_native(&mut self, native: Native) {
        self.natives.push(native);
    }
    pub fn increment_if_required(&mut self) {
        if self.pending_increment {
            self.ip += 1;
        }
    }
    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::Underflow)
    }
    pub fn current_instruction(&self) -> &Instruction {
        &self.instructions[self.ip]
    }
    pub fn execute_instruction(&mut self) -> Result<ExecutionState> {
        if self.halted || self.ip == self.instructions.len() {
            return Ok(ExecutionState::Done);
        }
        self.pending_increment = true;
        match &self.instructions[self.ip] {
            Instruction::Push(value) => self.stack.push(*value),
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::Call(id) => {
                self.frames.push(Frame::new(self.ip));
                self.ip = self.markers[*id];
                self.pending_increment = false;
            }
            Instruction::Jump(id) => {
                self.ip = self.markers[*id];
                self.pending_increment = false;
            }
            Instruction::JumpIf(id) => {
                let ip = self.markers[*id];
                if let Value::Boolean(bool) = self.pop()? {
                    if bool {
                        self.ip = ip;
                        self.pending_increment = false;
                    }
                } else {
                    return Err(Error::IncompatibleValues);
                }
            }
            Instruction::Ret => {
                self.ip = self.frames.pop().ok_or(Error::Underflow)?.return_ip + 1;
                self.pending_increment = false;
            }
            Instruction::Native(identifier) => {
                (self.natives.get(*identifier).ok_or(Error::UnknownNative)?)(self)?;
            }
            Instruction::Halt => self.halted = true,
            Instruction::Add => operator!(self; try_add),
            Instruction::Sub => operator!(self; try_sub),
            Instruction::Mul => operator!(self; try_mul),
            Instruction::Div => operator!(self; try_div),
            Instruction::Eq => operator!(self; try_eq),
            Instruction::Lt => operator!(self; try_lt),
            Instruction::Gt => operator!(self; try_gt),
            Instruction::LtEq => operator!(self; try_lteq),
            Instruction::GtEq => operator!(self; try_gt),
            Instruction::And => operator!(self; try_and),
            Instruction::Or => operator!(self; try_or),
        }
        Ok(ExecutionState::Running)
    }

    pub fn frames(&self) -> &[Frame] {
        self.frames.as_ref()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_ref()
    }
}
