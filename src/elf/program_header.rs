use std::marker::PhantomData;

use crate::bits::{Bitwidth, PtrType};
use crate::endian::Endianness;

use super::elf_bitwidth::ElfBitwidth;
use super::ElfParseError;

#[derive(Debug, Clone)]
pub struct ProgramHeader<B: ElfBitwidth> {
    _bitwidth: PhantomData<B>,

    pub type_: ProgramHeaderType,
    // Maybe read flags
    pub file_offset: <B as Bitwidth>::Ptr,
    pub virtual_address: <B as Bitwidth>::Ptr,
    pub physical_address: <B as Bitwidth>::Ptr,
    pub size: <B as Bitwidth>::Ptr,
    pub memory_size: <B as Bitwidth>::Ptr,
    // Maybe read alignment?
}

impl <B: ElfBitwidth> ProgramHeader<B> {
    pub fn parse(inp: &[u8], endianness: Endianness) -> Result<ProgramHeader<B>, ElfParseError> {
        let mut at = 0;
        let type_ = ProgramHeaderType::from_u32(endianness.read_u32(&inp));
        at += 4;

        // Skip p_flags
        at += 4;

        let file_offset = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let virtual_address = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let physical_address = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let size = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let memory_size = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);

        Ok(ProgramHeader {
            _bitwidth: PhantomData,
            type_,
            file_offset,
            virtual_address,
            physical_address,
            size,
            memory_size,
        })
    }

    pub fn get_content<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        &bytes[self.file_offset.to_usize()..self.file_offset.to_usize() + self.size.to_usize()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused, non_camel_case_types)]
pub enum ProgramHeaderType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
    Loos,
    Hios,
    LoProc,
    HiProc,
    Other(u32),
}

impl ProgramHeaderType {
    fn from_u32(value: u32) -> ProgramHeaderType {
        use ProgramHeaderType::*;
        match value {
            0x00000000 => Null,
            0x00000001 => Load,
            0x00000002 => Dynamic,
            0x00000003 => Interp,
            0x00000004 => Note,
            0x00000005 => Shlib,
            0x00000006 => Phdr,
            0x00000007 => Tls,
            0x60000000 => Loos,
            0x6FFFFFFF => Hios,
            0x70000000 => LoProc,
            0x7FFFFFFF => HiProc,
            _ => Other(value)
        }
    }
}
