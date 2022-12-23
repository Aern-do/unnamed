pub mod error;

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
};

use self::error::Error;

use super::{instruction::Instruction, program::Program, value::Value};
#[derive(Clone, Debug, Default)]
pub struct Incremental<K, V> {
    mapping: BTreeMap<K, (V, usize)>,
    offset: usize,
}
impl<K: Ord, V> Incremental<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> usize {
        let entry_offset = self.offset;
        self.offset += 1;
        self.mapping.insert(key, (value, entry_offset));
        entry_offset
    }
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.mapping.get(key).map(|value| &value.0)
    }
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.mapping.get_mut(key).map(|value| &mut value.0)
    }
    pub fn get_id<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.mapping.get(key).map(|value| value.1)
    }
}
#[derive(Clone, Debug)]
pub enum PendingInstruction {
    Generated(Instruction),
    Jump(usize, usize),
    JumpIf(usize, usize),
}
#[derive(Clone, Debug, Default)]
pub struct Procedure {
    instructions: Vec<PendingInstruction>,
    markers: HashMap<String, usize>,
}
pub type ProcedureOrMarker = Vec<Instruction>;
#[derive(Clone, Debug, Default)]
pub struct Builder {
    procedures: Incremental<String, Procedure>,
    current_proc: Option<String>,
    natives: Incremental<String, ()>,
}
impl Builder {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_default_natives() -> Self {
        let mut natives = Incremental::default();
        natives.insert(String::from("print"), ());
        Self {
            natives,
            ..Default::default()
        }
    }
    pub fn current_procedure(&mut self) -> Result<&Procedure, Error> {
        Ok(self
            .procedures
            .get(
                self.current_proc
                    .as_ref()
                    .ok_or(Error::ProcedureNotSelected)?,
            )
            .unwrap())
    }
    pub fn current_procedure_id(&self) -> Result<usize, Error> {
        Ok(self
            .procedures
            .get_id(
                self.current_proc
                    .as_ref()
                    .ok_or(Error::ProcedureNotSelected)?,
            )
            .unwrap())
    }
    pub fn current_procedure_mut(&mut self) -> Result<&mut Procedure, Error> {
        Ok(self
            .procedures
            .get_mut(
                self.current_proc
                    .as_ref()
                    .ok_or(Error::ProcedureNotSelected)?,
            )
            .unwrap())
    }
    pub fn procedure<S>(&mut self, name: S)
    where
        String: From<S>,
    {
        let name = String::from(name);
        self.current_proc = Some(name.clone());
        if self.procedures.get(&name).is_none() {
            self.procedures.insert(name, Procedure::default());
        };
    }
    pub fn marker<S>(&mut self, name: S) -> Result<(), Error>
    where
        String: From<S>,
    {
        let name = String::from(name);
        let current_proc = self.current_procedure_mut()?;
        current_proc.markers.insert(
            name,
            current_proc
                .instructions
                .len()
                .checked_sub(1)
                .unwrap_or_default(),
        );
        Ok(())
    }
    pub fn call<S>(&mut self, name: S) -> Result<(), Error>
    where
        String: From<S>,
    {
        let name = String::from(name);
        let id = self
            .procedures
            .get_id(&name)
            .unwrap_or_else(|| self.procedures.insert(name, Procedure::default()));
        self.current_procedure_mut()?
            .instructions
            .push(PendingInstruction::Generated(Instruction::Call(id)));
        Ok(())
    }
    pub fn jump<S>(&mut self, name: S) -> Result<(), Error>
    where
        String: From<S>,
    {
        let name = String::from(name);
        let id = self.current_procedure_id()?;
        let current_proc = self.current_procedure_mut()?;
        let offset = current_proc
            .markers
            .get(&name)
            .ok_or(Error::UnknownMarker)?;
        current_proc
            .instructions
            .push(PendingInstruction::Jump(*offset, id));
        Ok(())
    }
    pub fn jump_if<S>(&mut self, name: S) -> Result<(), Error>
    where
        String: From<S>,
    {
        let name = String::from(name);
        let id = self.current_procedure_id()?;
        let current_proc = self.current_procedure_mut()?;
        let offset = current_proc
            .markers
            .get(&name)
            .ok_or(Error::UnknownMarker)?;
        current_proc
            .instructions
            .push(PendingInstruction::JumpIf(*offset, id));
        Ok(())
    }
    pub fn push<V>(&mut self, value: V) -> Result<(), Error>
    where
        Value: From<V>,
    {
        let value = Value::from(value);

        self.current_procedure_mut()?
            .instructions
            .push(PendingInstruction::Generated(Instruction::Push(value)));
        Ok(())
    }
    pub fn native<S>(&mut self, name: S) -> Result<(), Error>
    where
        String: From<S>,
    {
        let name = String::from(name);
        let id = self.natives.get_id(&name).ok_or(Error::UnknownNative)?;
        self.current_procedure_mut()?
            .instructions
            .push(PendingInstruction::Generated(Instruction::Native(id)));
        Ok(())
    }
    pub fn build(self) -> Result<Program, Error> {
        let mut procedures = vec![];
        let mut instructions = vec![];
        let entry_point = self.procedures.get_id("main").ok_or(Error::MissingMain)?;
        // Pregenerate instruction pointer mappings
        let mut no_lookup_len = None;
        for (.., (procedure_instructions, id)) in self.procedures.mapping.iter() {
            if *id == 0 {
                procedures.push(0);
                no_lookup_len = Some(procedure_instructions.instructions.len())
            } else if let Some(len) = no_lookup_len {
                procedures.push(len);
            } else {
                procedures.push(procedures[*id - 1] + procedure_instructions.instructions.len())
            }
        }
        // Expand instructions
        for (.., (procedure_instructions, ..)) in self.procedures.mapping {
            for instruction in procedure_instructions.instructions {
                match instruction {
                    PendingInstruction::Generated(instruction) => instructions.push(instruction),
                    PendingInstruction::Jump(offset, id) => {
                        instructions.push(Instruction::Jump(procedures[id] + offset))
                    }
                    PendingInstruction::JumpIf(offset, id) => {
                        instructions.push(Instruction::JumpIf(procedures[id] + offset))
                    }
                }
            }
        }
        Ok(Program {
            procedures,
            instructions,
            entry_point,
        })
    }
}
macro_rules! zero_op {
    ($name: ident -> $variant: ident) => {
        impl Builder {
            pub fn $name(&mut self) -> Result<(), Error> {
                self.current_procedure_mut()?.instructions.push(PendingInstruction::Generated(Instruction::$variant));
                Ok(())
            }
        }
    };
    ($($name: ident -> $variant: ident),*) => {
        $(zero_op!($name -> $variant);)*
    }
}
zero_op!(ret -> Ret, pop -> Pop, halt -> Halt, add -> Add, sub -> Sub, mul -> Mul, div -> Div, eq -> Eq, lt -> Lt, gt -> Gt, lt_eq -> LtEq, gt_eq -> GtEq, and -> And, or -> Or);

#[cfg(test)]
mod tests {
    use crate::vm::{instruction::Instruction, program::Program, value::Value};

    use super::{error::Error, Builder};

    fn test<F: FnOnce(&mut Builder) -> Result<(), Error>>(build: F, expected: Vec<Instruction>) {
        let mut builder = Builder::with_default_natives();
        builder.procedure("main");
        build(&mut builder).unwrap();
        assert_eq!(builder.build().unwrap().instructions, expected)
    }
    fn test_program<F: FnOnce(&mut Builder) -> Result<(), Error>>(build: F, expected: Program) {
        let mut builder = Builder::with_default_natives();
        build(&mut builder).unwrap();
        assert_eq!(builder.build().unwrap(), expected)
    }
    #[test]
    fn builds_zero_op() {
        test(|builder| builder.ret(), vec![Instruction::Ret]);
        test(|builder| builder.pop(), vec![Instruction::Pop]);
        test(|builder| builder.halt(), vec![Instruction::Halt]);
        test(|builder| builder.add(), vec![Instruction::Add]);
        test(|builder| builder.sub(), vec![Instruction::Sub]);
        test(|builder| builder.mul(), vec![Instruction::Mul]);
        test(|builder| builder.div(), vec![Instruction::Div]);
        test(|builder| builder.eq(), vec![Instruction::Eq]);
        test(|builder| builder.lt(), vec![Instruction::Lt]);
        test(|builder| builder.gt(), vec![Instruction::Gt]);
        test(|builder| builder.lt_eq(), vec![Instruction::LtEq]);
        test(|builder| builder.gt_eq(), vec![Instruction::GtEq]);
        test(|builder| builder.and(), vec![Instruction::And]);
        test(|builder| builder.or(), vec![Instruction::Or]);
    }
    #[test]
    fn push() {
        test(
            |builder| builder.push(0_i8),
            vec![Instruction::Push(Value::Byte(0))],
        );
        test(
            |builder| builder.push(0_i16),
            vec![Instruction::Push(Value::Short(0))],
        );
        test(
            |builder| builder.push(0),
            vec![Instruction::Push(Value::Integer(0))],
        );
        test(
            |builder| builder.push(0_i64),
            vec![Instruction::Push(Value::Long(0))],
        );
        test(
            |builder| builder.push(0.0_f32),
            vec![Instruction::Push(Value::Float(0.0))],
        );
        test(
            |builder| builder.push(0.0),
            vec![Instruction::Push(Value::Double(0.0))],
        );
        test(
            |builder| builder.push(false),
            vec![Instruction::Push(Value::Boolean(false))],
        );
        test(
            |builder| builder.push(Value::Short(0)),
            vec![Instruction::Push(Value::Short(0))],
        );
    }
    #[test]
    fn builds_correctly_mapped_calls() {
        test_program(
            |builder| {
                builder.procedure("add");
                builder.procedure("main");
                builder.push(5)?;
                builder.push(5)?;
                builder.add()?;
                builder.call("add")?;
                builder.halt()?;
                builder.procedure("add");
                builder.add()?;
                builder.ret()?;
                Ok(())
            },
            Program {
                procedures: vec![0, 2],
                entry_point: 1,
                instructions: vec![
                    Instruction::Add,
                    Instruction::Ret,
                    Instruction::Push(Value::Integer(5)),
                    Instruction::Push(Value::Integer(5)),
                    Instruction::Add,
                    Instruction::Call(0),
                    Instruction::Halt,
                ],
            },
        );
    }
    #[test]
    fn markers() {
        test_program(
            |builder| {
                builder.procedure("main");
                builder.marker("loop")?;
                builder.push(5)?;
                builder.push(5)?;
                builder.add()?;
                builder.native("print")?;
                builder.jump("loop")?;
                builder.halt()?;
                Ok(())
            },
            Program {
                procedures: vec![0],
                entry_point: 0,
                instructions: vec![
                    Instruction::Push(Value::Integer(5)),
                    Instruction::Push(Value::Integer(5)),
                    Instruction::Add,
                    Instruction::Native(0),
                    Instruction::Jump(0),
                    Instruction::Halt,
                ],
            },
        )
    }
}
