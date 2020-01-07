
use crate::instruction_set::InstructionSet;

pub trait ParsedExecutable {
    fn get_instruction_set(&self) -> InstructionSet;
}
