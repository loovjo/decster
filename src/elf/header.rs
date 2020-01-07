use std::marker::PhantomData;

use crate::bits::{Bitwidth, PtrType};
use crate::endian::Endianness;
use crate::instruction_set::InstructionSet;

use super::elf_bitwidth::ElfBitwidth;
use super::osabi::OsABI;
use super::elf_instruction_set::instruction_set_from_u8;
use super::ElfParseError;
use super::object_type::ObjectType;

#[derive(Debug, Clone)]
pub struct Header<B: ElfBitwidth> {
    _bitwidth: PhantomData<B>,
    pub endianness: Endianness,
    pub abi: OsABI,
    pub abi_version: u8,
    pub object_type: ObjectType,
    pub instruction_set: InstructionSet,

    pub entry_offset: <B as Bitwidth>::Ptr,
    pub program_header_offset: <B as Bitwidth>::Ptr,
    pub section_header_offset: <B as Bitwidth>::Ptr,
    pub program_header_entry_size: u16,
    pub section_header_entry_size: u16,
    pub program_header_n_entries: u16,
    pub section_header_n_entries: u16,
    pub section_header_shstrtab_index: usize,
}

impl <B: ElfBitwidth> Header<B> {
    pub fn parse(inp: &[u8]) -> Result<Header<B>, ElfParseError> {
        let bitwidth_marker = inp[0x04];
        if bitwidth_marker != B::MARKER {
            return Err(ElfParseError::WrongBitwidth([bitwidth_marker]))
        }

        let endianness = match inp[0x05] {
            1 => Endianness::LittleEndian,
            2 => Endianness::BigEndian,
            _ => return Err(ElfParseError::WrongEndianness([inp[0x05]]))
        };

        // Maybe check EI_VERSION?

        let abi = OsABI::from_u8(inp[0x07])?;

        let abi_version = inp[0x08];

        let object_type = ObjectType::from_u16(endianness.read_u16(&inp[0x10..]))?;

        let instruction_set = instruction_set_from_u8(inp[0x12])?;

        // Maybe check e_version?

        let mut at = 0x18;

        let entry_offset = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        // TODO: Clean this up!
        // Preferably, we would alias some type to <B as Bitwidth>::Ptr, but this is wierd at the moment.
        let program_header_offset = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let section_header_offset = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        // Skip e_flags
        at += 4;
        // Skip eh_size
        at += 2;

        let program_header_entry_size = endianness.read_u16(&inp[at..]);
        at += 2;

        let program_header_n_entries = endianness.read_u16(&inp[at..]);
        at += 2;

        let section_header_entry_size = endianness.read_u16(&inp[at..]);
        at += 2;

        let section_header_n_entries = endianness.read_u16(&inp[at..]);
        at += 2;

        let section_header_shstrtab_index = endianness.read_u16(&inp[at..]) as usize;

        Ok(Header {
            _bitwidth: PhantomData,
            endianness,
            abi,
            abi_version,
            object_type,
            instruction_set,
            entry_offset,
            program_header_offset,
            section_header_offset,
            program_header_entry_size,
            section_header_entry_size,
            program_header_n_entries,
            section_header_n_entries,
            section_header_shstrtab_index,
        })
    }
}

