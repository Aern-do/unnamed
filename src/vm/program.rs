use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::instruction::Instruction;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub procedures: Vec<usize>,
    pub entry_point: usize,
    pub instructions: Vec<Instruction>,
}
impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Header")?;
        writeln!(f, "   Procedures = {:?}", self.procedures)?;
        writeln!(f, "   Entry Point = {}", self.entry_point)?;
        writeln!(f, "Instructions:")?;
        for instruction in &self.instructions {
            writeln!(f, "   {}", instruction)?;
        }
        Ok(())
    }
}
