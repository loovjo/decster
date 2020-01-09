use std::marker::PhantomData;

use crate::bits::{Bitwidth, PtrType};
use crate::endian::Endianness;
use crate::parsable_file::ParsableFile;

use super::elf_bitwidth::ElfBitwidth;
use super::ElfParseError;
use super::Elf;

#[derive(Debug, Clone)]
pub struct Symbol<B: ElfBitwidth> {
    _bitwidth: PhantomData<B>,

    name_strtab_offset: usize,
    value: <B as Bitwidth>::Ptr,
    size: <B as Bitwidth>::Ptr,
    info: u8,
    other: u8,
    section_header_index: usize,
}

impl <B: ElfBitwidth> Symbol<B> {
    pub fn parse(inp: &mut ParsableFile<'_>, endianness: Endianness) -> Result<Symbol<B>, ElfParseError> {
        let name_strtab_offset = endianness.read_u32(inp)? as usize;

        // The layout is completely different for 32 and 64 bits
        if <B as Bitwidth>::Ptr::N_BYTES == 4 {
            let value = <B as Bitwidth>::Ptr::read(endianness, inp)?;

            let size = <B as Bitwidth>::Ptr::read(endianness, inp)?;

            let info = endianness.read_u8(inp)?;

            let other = endianness.read_u8(inp)?;

            let section_header_index = endianness.read_u16(inp)? as usize;

            Ok(Symbol {
                _bitwidth: PhantomData,
                name_strtab_offset,
                value,
                size,
                info,
                other,
                section_header_index,
            })
        } else {
            // 64 bit
            let info = endianness.read_u8(inp)?;

            let other = endianness.read_u8(inp)?;

            let section_header_index = endianness.read_u16(inp)? as usize;

            let value = <B as Bitwidth>::Ptr::read(endianness, inp)?;

            let size = <B as Bitwidth>::Ptr::read(endianness, inp)?;

            Ok(Symbol {
                _bitwidth: PhantomData,
                name_strtab_offset,
                value,
                size,
                info,
                other,
                section_header_index,
            })

        }
    }

    pub fn get_name<'a>(&self, bytes: &mut ParsableFile<'a>, elf: &Elf<B>) -> Result<Option<&'a [u8]>, ElfParseError> {
        let mut strtab_header = None;
        for section_header in &elf.section_headers {
            let name = section_header.get_name(bytes, elf)?;
            if name == b".strtab" {
                strtab_header = Some(section_header);
            }
        }

        let strtab_header = if let Some(strtab_header) = strtab_header {
            strtab_header
        } else {
            return Ok(None);
        };

        let strtab = strtab_header.get_content(bytes)?;
        let name_start = &strtab[self.name_strtab_offset..];

        let mut end = 0;
        while name_start[end] != 0 {
            end += 1;
        }

        Ok(Some(&name_start[..end]))
    }

}
