use std::fmt::Debug;
use std::convert::TryInto;

use crate::error::GenericParseError;
use crate::endian::Endianness;
use crate::parsable_file::ParsableFile;

pub trait Bitwidth: Debug + Clone + Copy + PartialEq + Eq {
    type Ptr: PtrType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SixtyfourBit {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThirtytwoBit {}

impl Bitwidth for SixtyfourBit { type Ptr = u64; }
impl Bitwidth for ThirtytwoBit { type Ptr = u32; }


pub trait PtrType: Debug + Clone {
    const N_BYTES: usize;
    fn to_u64(&self) -> u64;

    fn to_usize(&self) -> Result<usize, GenericParseError> {
        match self.to_u64().try_into() {
            Ok(x) => Ok(x),
            Err(_) => Err(GenericParseError::PtrTooLarge(self.to_u64()))
        }
    }

    fn read(endianness: Endianness, from: &mut ParsableFile<'_>) -> Result<Self, GenericParseError>;
}

impl PtrType for u32 {
    const N_BYTES: usize = 4;
    fn to_u64(&self) -> u64 { *self as u64 }

    fn read(endianness: Endianness, from: &mut ParsableFile<'_>) -> Result<Self, GenericParseError> {
        let read_bytes = from.read_n_bytes(Self::N_BYTES)?.try_into().unwrap();

        match endianness {
            Endianness::LittleEndian => Ok(u32::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u32::from_be_bytes(read_bytes)),
        }
    }
}

impl PtrType for u64 {
    const N_BYTES: usize = 8;
    fn to_u64(&self) -> u64 { *self }

    fn read(endianness: Endianness, from: &mut ParsableFile<'_>) -> Result<Self, GenericParseError> {
        let read_bytes = from.read_n_bytes(Self::N_BYTES)?.try_into().unwrap();

        match endianness {
            Endianness::LittleEndian => Ok(u64::from_le_bytes(read_bytes)),
            Endianness::BigEndian => Ok(u64::from_be_bytes(read_bytes)),
        }
    }
}
