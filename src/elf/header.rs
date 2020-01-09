use std::marker::PhantomData;

use crate::bits::{Bitwidth, PtrType};
use crate::endian::Endianness;
use crate::instruction_set::InstructionSet;
use crate::parsable_file::ParsableFile;

use super::elf_bitwidth::ElfBitwidth;
use super::osabi::OsABI;
use super::elf_instruction_set::instruction_set_from_u16;
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
    pub fn parse(inp: &mut ParsableFile<'_>) -> Result<Header<B>, ElfParseError> {
        let bitwidth_marker = inp.read_n_bytes(1)?[0];
        if bitwidth_marker != B::MARKER {
            return Err(ElfParseError::WrongBitwidth(bitwidth_marker))
        }

        let endianness_marker = inp.read_n_bytes(1)?[0];
        let endianness = match endianness_marker {
            1 => Endianness::LittleEndian,
            2 => Endianness::BigEndian,
            _ => return Err(ElfParseError::WrongEndianness(endianness_marker))
        };

        // Skip version

        inp.skip_n_bytes(1)?;

        let abi = OsABI::from_u8(endianness.read_u8(inp)?)?;

        let abi_version = endianness.read_u8(inp)?;

        // Skip padding
        inp.skip_n_bytes(7)?;

        let object_type = ObjectType::from_u16(endianness.read_u16(inp)?)?;

        let instruction_set = instruction_set_from_u16(endianness.read_u16(inp)?)?;

        // Maybe check e_version?
        inp.skip_n_bytes(4)?;

        let entry_offset = <B as Bitwidth>::Ptr::read(endianness, inp)?;

        let program_header_offset = <B as Bitwidth>::Ptr::read(endianness, inp)?;

        let section_header_offset = <B as Bitwidth>::Ptr::read(endianness, inp)?;

        // Skip e_flags
        inp.skip_n_bytes(4)?;

        // Skip e_ehsize
        inp.skip_n_bytes(2)?;

        let program_header_entry_size = endianness.read_u16(inp)?;

        let program_header_n_entries = endianness.read_u16(inp)?;

        let section_header_entry_size = endianness.read_u16(inp)?;

        let section_header_n_entries = endianness.read_u16(inp)?;

        let section_header_shstrtab_index = endianness.read_u16(inp)? as usize;

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

