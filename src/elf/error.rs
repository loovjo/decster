#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElfParseError {
    WrongMagic([u8; 4]),
    WrongBitwidth([u8; 1]),
    WrongEndianness([u8; 1]),
    UnknownOsABI([u8; 1]),
    UnknownInstructionSet([u8; 1]),
    UnknownObjectType(u16),
}
