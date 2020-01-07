use super::ElfParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum ObjectType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    LoOs,
    HiOs,
    LoProc,
    HiProc,
}

impl ObjectType {
    pub fn from_u16(value: u16) -> Result<ObjectType, ElfParseError> {
        use ObjectType::*;

        match value {
            0 => Ok(None),
            1 => Ok(Rel),
            2 => Ok(Exec),
            3 => Ok(Dyn),
            4 => Ok(Core),
            0xfe00 => Ok(LoOs),
            0xfeff => Ok(HiOs),
            0xff00 => Ok(LoProc),
            0xffff => Ok(HiProc),
            _ => Err(ElfParseError::UnknownObjectType(value)),
        }
    }
}
