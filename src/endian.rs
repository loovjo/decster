use std::convert::TryInto;

use crate::error::GenericParseError;
use crate::parsable_file::ParsableFile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}


impl Endianness {
    #[allow(unused)]
    pub fn read_u8(&self, bytes: &mut ParsableFile<'_>) -> Result<u8, GenericParseError> {
        let read_bytes = bytes.read_n_bytes(1)?.try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(u8::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u8::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_u16(&self, bytes: &mut ParsableFile<'_>) -> Result<u16, GenericParseError> {
        let read_bytes = bytes.read_n_bytes(2)?.try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(u16::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u16::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_u32(&self, bytes: &mut ParsableFile<'_>) -> Result<u32, GenericParseError> {
        let read_bytes = bytes.read_n_bytes(4)?.try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(u32::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u32::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_u64(&self, bytes: &mut ParsableFile<'_>) -> Result<u64, GenericParseError> {
        let read_bytes = bytes.read_n_bytes(8)?.try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(u64::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u64::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_i16(&self, bytes: &mut ParsableFile<'_>) -> Result<i16, GenericParseError> {
        let read_bytes = bytes.read_n_bytes(2)?.try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(i16::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(i16::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_i32(&self, bytes: &mut ParsableFile<'_>) -> Result<i32, GenericParseError> {
        let read_bytes = bytes.read_n_bytes(4)?.try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(i32::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(i32::from_be_bytes(read_bytes)),
        }
    }

    #[allow(unused)]
    pub fn read_i64(&self, bytes: &mut ParsableFile<'_>) -> Result<i64, GenericParseError> {
        let read_bytes = bytes.read_n_bytes(8)?.try_into().unwrap();
        match *self {
            Endianness::LittleEndian => Ok(i64::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(i64::from_be_bytes(read_bytes)),
        }
    }
}
