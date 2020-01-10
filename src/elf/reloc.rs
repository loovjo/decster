use std::marker::PhantomData;

use crate::bits::{Bitwidth, PtrType};
use crate::endian::Endianness;
use crate::parsable_file::ParsableFile;

use super::elf_bitwidth::ElfBitwidth;
use super::ElfParseError;
use super::SectionHeader;
use super::Symbol;
use super::Elf;

#[derive(Debug, Clone)]
pub struct Relocation<B: ElfBitwidth> {
    _bitwidth: PhantomData<B>,

    virtual_address: <B as Bitwidth>::Ptr,
    info: <B as Bitwidth>::Ptr,
    addend: Option<i64>,
}

impl <B: ElfBitwidth> Relocation<B> {
    #[allow(unused)]
    pub fn parse(inp: &mut ParsableFile<'_>, endianness: Endianness, has_addend: bool) -> Result<Self, ElfParseError> {
        let virtual_address = <B as Bitwidth>::Ptr::read(endianness, inp)?;

        let info = <B as Bitwidth>::Ptr::read(endianness, inp)?;

        let addend = if has_addend {
            // Since this might be negative, we can't read it using Ptr.
            // We'll do it manually
            if <B as Bitwidth>::Ptr::N_BYTES == 4 {
                Some(endianness.read_i32(inp)? as i64)
            } else {
                Some(endianness.read_i64(inp)? as i64)
            }
        } else {
            None
        };

        Ok(Relocation {
            _bitwidth: PhantomData,
            virtual_address,
            info,
            addend
        })
    }

    pub fn get_symbol_idx(&self) -> usize {
        let info = self.info.to_u64();
        if <B as Bitwidth>::Ptr::N_BYTES == 4 {
            (info >> 8) as usize
        } else {
            (info >> 32) as usize
        }
    }

    pub fn get_symbol(&self, bytes: &mut ParsableFile<'_>, elf: &Elf<B>, reloc_table: &SectionHeader<B>) -> Result<Symbol<B>, ElfParseError> {
        let symbols = elf.symbols(bytes, reloc_table.link)?;

        symbols.get(self.get_symbol_idx()).ok_or(ElfParseError::InvalidSymbolReference(self.get_symbol_idx())).map(|x| x.clone())
    }

}
