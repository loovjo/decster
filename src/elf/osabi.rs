use super::error::ElfParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused, non_camel_case_types)]
pub enum OsABI {
    System_V,
    HP_UX,
    NetBSD,
    Linux,
    GNU_Hurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    Novell_Modesto,
    OpenBSD,
    OpenVMS,
    NonStop_Kernel,
    AROS,
    Fenix_OS,
    CloudABI,
    OpenVOS,
}

impl OsABI {
    pub fn from_u8(value: u8) -> Result<OsABI, ElfParseError> {
        use OsABI::*;

        match value {
            0x00 => Ok(System_V),
            0x01 => Ok(HP_UX),
            0x02 => Ok(NetBSD),
            0x03 => Ok(Linux),
            0x04 => Ok(GNU_Hurd),
            0x06 => Ok(Solaris),
            0x07 => Ok(AIX),
            0x08 => Ok(IRIX),
            0x09 => Ok(FreeBSD),
            0x0A => Ok(Tru64),
            0x0B => Ok(Novell_Modesto),
            0x0C => Ok(OpenBSD),
            0x0D => Ok(OpenVMS),
            0x0E => Ok(NonStop_Kernel),
            0x0F => Ok(AROS),
            0x10 => Ok(Fenix_OS),
            0x11 => Ok(CloudABI),
            0x12 => Ok(OpenVOS),
            _ => Err(ElfParseError::UnknownOsABI([value])),
        }
    }
}
