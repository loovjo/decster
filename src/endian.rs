use std::convert::TryInto;

use crate::error::GenericParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}


impl Endianness {
    #[allow(unused)]
    pub fn read_u16(&self, bytes: &[u8]) -> Result<u16, GenericParseError> {
        if bytes.len() < 2 {
            return Err(GenericParseError::EndOfFead);
        }
        let read_bytes = bytes[0..2].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(u16::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u16::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_u32(&self, bytes: &[u8]) -> Result<u32, GenericParseError> {
        if bytes.len() < 4 {
            return Err(GenericParseError::EndOfFead);
        }
        let read_bytes = bytes[0..4].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(u32::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u32::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_u64(&self, bytes: &[u8]) -> Result<u64, GenericParseError> {
        if bytes.len() < 8 {
            return Err(GenericParseError::EndOfFead);
        }
        let read_bytes = bytes[0..8].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(u64::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u64::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_i16(&self, bytes: &[u8]) -> Result<i16, GenericParseError> {
        if bytes.len() < 2 {
            return Err(GenericParseError::EndOfFead);
        }
        let read_bytes = bytes[0..2].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(i16::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(i16::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_i32(&self, bytes: &[u8]) -> Result<i32, GenericParseError> {
        if bytes.len() < 4 {
            return Err(GenericParseError::EndOfFead);
        }
        let read_bytes = bytes[0..4].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(i32::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(i32::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_i64(&self, bytes: &[u8]) -> Result<i64, GenericParseError> {
        if bytes.len() < 8 {
            return Err(GenericParseError::EndOfFead);
        }
        let read_bytes = bytes[0..8].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(i64::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(i64::from_be_bytes(read_bytes)),
        }
    }
}
