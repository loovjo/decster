use std::marker::PhantomData;

use crate::bits::{Bitwidth, PtrType};
use crate::endian::Endianness;

use super::elf_bitwidth::ElfBitwidth;
use super::ElfParseError;
use super::Elf;

#[derive(Debug, Clone)]
pub struct Symbol<B: ElfBitwidth> {
    _bitwidth: PhantomData<B>,

    name_strtab_offset: usize,
    value: <B as Bitwidth>::Ptr,
    size: usize,
    info: u8,
    other: u8,
    section_header_index: usize,
}

impl <B: ElfBitwidth> Symbol<B> {
    pub fn parse(inp: &[u8], endianness: Endianness) -> Result<Symbol<B>, ElfParseError> {
        let mut at = 0;
        let name_strtab_offset = endianness.read_u32(&inp[at..]) as usize;
        at += 4;

        // The layout is completely different for 32 and 64 bits
        if <B as Bitwidth>::Ptr::N_BYTES == 4 {
            let value = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
            at += <B as Bitwidth>::Ptr::N_BYTES;

            let size = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]).to_usize();
            at += <B as Bitwidth>::Ptr::N_BYTES;

            let info = inp[at];
            at += 1;

            let other = inp[at];
            at += 1;

            let section_header_index = endianness.read_u16(&inp[at..]) as usize;

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
            let info = inp[at];
            at += 1;

            let other = inp[at];
            at += 1;

            let section_header_index = endianness.read_u16(&inp[at..]) as usize;
            at += 2;

            let value = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]);
            at += <B as Bitwidth>::Ptr::N_BYTES;

            let size = <B as Bitwidth>::Ptr::read(endianness, &inp[at..]).to_usize();

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

    pub fn get_name<'a>(&self, bytes: &'a [u8], elf: &Elf<B>) -> Option<&'a [u8]> {
        let strtab_header = elf.section_headers
            .iter()
            .find(|x| {
                let name = x.get_name(bytes, elf);
                name == b".strtab"
            })?;

        let strtab = strtab_header.get_content(bytes);
        let name_start = &strtab[self.name_strtab_offset..];

        let mut end = 0;
        while name_start[end] != 0 {
            end += 1;
        }

        Some(&name_start[..end])
    }

}
