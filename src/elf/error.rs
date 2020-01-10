use crate::error::GenericParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum ElfParseError {
    Generic(GenericParseError),
    WrongMagic(Vec<u8>),
    WrongBitwidth(u8),
    WrongEndianness(u8),
    InvalidRelocationTableSize(usize), // Index of relocation table
    InvalidRelocationEntrySize(usize, usize), // Index of relocation table, index of relocation entry in table
    UnknownOsABI([u8; 1]),
    UnknownInstructionSet(u16),
    UnknownObjectType(u16),
}

impl From<GenericParseError> for ElfParseError {
    fn from(err: GenericParseError) -> Self {
        ElfParseError::Generic(err)
    }
}
