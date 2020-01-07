use std::fmt::Debug;
use std::convert::TryInto;

use crate::endian::Endianness;

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
    fn to_usize(&self) -> usize;

    fn read(endianness: Endianness, bytes: &[u8]) -> Self;
}

impl PtrType for u32 {
    const N_BYTES: usize = 4;
    fn to_usize(&self) -> usize { *self as usize }

    fn read(endianness: Endianness, bytes: &[u8]) -> Self {
        let read_bytes = bytes[0..<Self as PtrType>::N_BYTES].try_into().unwrap();
        match endianness {
            Endianness::LittleEndian => u32::from_le_bytes(read_bytes),
            Endianness::BigEndian => u32::from_be_bytes(read_bytes),
        }
    }
}

impl PtrType for u64 {
    const N_BYTES: usize = 8;
    fn to_usize(&self) -> usize { *self as usize }

    fn read(endianness: Endianness, bytes: &[u8]) -> Self {
        let read_bytes = bytes[0..<Self as PtrType>::N_BYTES].try_into().unwrap();

        match endianness {
            Endianness::LittleEndian => u64::from_le_bytes(read_bytes),
            Endianness::BigEndian => u64::from_be_bytes(read_bytes),
        }
    }
}
