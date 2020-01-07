use crate::instruction_set::InstructionSet;
use super::error::ElfParseError;

pub fn instruction_set_from_u8(value: u8) -> Result<InstructionSet, ElfParseError> {
    use InstructionSet::*;
    match value {
        0x00 => Ok(NotSpecified),
        0x02 => Ok(SPARC),
        0x03 => Ok(X86),
        0x08 => Ok(MIPS),
        0x14 => Ok(PowerPC),
        0x16 => Ok(S390),
        0x28 => Ok(ARM),
        0x2A => Ok(SuperH),
        0x32 => Ok(IA_64),
        0x3E => Ok(X86_64),
        0xB7 => Ok(AArch64),
        0xF3 => Ok(RISC_V),
        _ => Err(ElfParseError::UnknownInstructionSet([value])),
    }
}
