use std::marker::PhantomData;

use crate::bits::{Bitwidth, PtrType};
use crate::endian::Endianness;

use super::elf_bitwidth::ElfBitwidth;
use super::ElfParseError;
use super::Elf;

#[derive(Debug, Clone)]
pub struct SectionHeader<B: ElfBitwidth> {
    _bitwidth: PhantomData<B>,

    pub shstrtab_offset: usize,
    pub type_: SectionHeaderType,
    // TODO: Add this
    // pub flags: SectionHeaderFlags,
    pub virtual_address: <B as Bitwidth>::Ptr,
    pub file_offset: <B as Bitwidth>::Ptr,
    pub size: <B as Bitwidth>::Ptr,
    // TODO: Maybe add link, info, align, entsize
}

impl <B: ElfBitwidth> SectionHeader<B> {
    pub fn parse(inp: &[u8], endianness: Endianness) -> Result<SectionHeader<B>, ElfParseError> {
        let mut at = 0x0;

        let shstrtab_offset = endianness.read_u32(&inp[at..]) as usize;
        at += 4;

        let type_ = SectionHeaderType::from_u32(endianness.read_u32(&inp[at..]));
        at += 4;

        // TODO: sh_flags
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let virtual_address = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let file_offset = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
        at += <B as Bitwidth>::Ptr::N_BYTES;

        let size = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);

        Ok(SectionHeader {
            _bitwidth: PhantomData,
            shstrtab_offset,
            type_,
            virtual_address,
            file_offset,
            size,
        })
    }

    pub fn get_content<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        &bytes[self.file_offset.to_usize()..self.file_offset.to_usize() + self.size.to_usize()]
    }

    pub fn get_name<'a>(&self, bytes: &'a [u8], elf: &Elf<B>) -> &'a [u8] {
        let section_header_shstrtab = &elf.section_headers[elf.header.section_header_shstrtab_index];

        let shstrtab = section_header_shstrtab.get_content(bytes);
        let name_start = &shstrtab[self.shstrtab_offset..];

        let mut end = 0;
        while name_start[end] != 0 {
            end += 1;
        }

        &name_start[..end]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused, non_camel_case_types)]
pub enum SectionHeaderType {
    Null,
    Progbits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    Init_ARRAY,
    Fini_ARRAY,
    Preinit_ARRAY,
    Group,
    SymTab_SHNDX,
    Num,
    Loos,
    Other
}

impl SectionHeaderType {
    fn from_u32(value: u32) -> SectionHeaderType {
        use SectionHeaderType::*;

        match value {
            0x0 => Null,
            0x1 => Progbits,
            0x2 => SymTab,
            0x3 => StrTab,
            0x4 => Rela,
            0x5 => Hash,
            0x6 => Dynamic,
            0x7 => Note,
            0x8 => Nobits,
            0x9 => Rel,
            0x0A => Shlib,
            0x0B => Dynsym,
            0x0E => Init_ARRAY,
            0x0F => Fini_ARRAY,
            0x10 => Preinit_ARRAY,
            0x11 => Group,
            0x12 => SymTab_SHNDX,
            0x13 => Num,
            0x60000000 => Loos,
            _ => Other,
        }
    }
}
