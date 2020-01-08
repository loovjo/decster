use crate::bits::{PtrType, Bitwidth};
use crate::common::ParsedExecutable;
use crate::instruction_set::InstructionSet;

mod error;
pub use self::error::ElfParseError;

mod elf_bitwidth;
use self::elf_bitwidth::ElfBitwidth;

mod osabi;
mod elf_instruction_set;
mod object_type;

mod header;
use self::header::Header;

mod program_header;
use self::program_header::ProgramHeader;

mod section_header;
use self::section_header::SectionHeader;

mod symbol;
use self::symbol::Symbol;

const ELF_MAGIC: [u8; 4] = [0x7F, 0x45, 0x4c, 0x46];


#[derive(Debug, Clone)]
pub struct Elf<B: ElfBitwidth> {
    pub header: Header<B>,
    pub program_headers: Vec<ProgramHeader<B>>,
    pub section_headers: Vec<SectionHeader<B>>,
}

impl <B: ElfBitwidth> ParsedExecutable for Elf<B> {
    fn get_instruction_set(&self) -> InstructionSet {
        self.header.instruction_set
    }
}


impl <B: ElfBitwidth> Elf<B> {
    pub fn parse(inp: &[u8]) -> Result<Elf<B>, ElfParseError> {
        if inp[0..4] != ELF_MAGIC {
            return Err(ElfParseError::WrongMagic([ inp[0], inp[1], inp[2], inp[3] ]))
        }

        let header = Header::<B>::parse(inp)?;

        // Read program headers
        let mut program_headers = Vec::with_capacity(header.program_header_n_entries as usize);

        for program_header_idx in 0..header.program_header_n_entries {
            let base_offset = header.program_header_offset.to_usize();
            let relative_offset = program_header_idx as usize * header.program_header_entry_size as usize;

            let bytes = &inp[base_offset + relative_offset..];
            let program_header = ProgramHeader::parse(bytes, header.endianness)?;

            program_headers.push(program_header);
        }

        // Read section headers
        let mut section_headers = Vec::with_capacity(header.section_header_n_entries as usize);
        for section_header_idx in 0..header.section_header_n_entries {
            let base_offset = header.section_header_offset.to_usize();
            let relative_offset = section_header_idx as usize * header.section_header_entry_size as usize;

            let bytes = &inp[base_offset + relative_offset..];
            let section_header = SectionHeader::parse(bytes, header.endianness)?;

            section_headers.push(section_header);
        }

        Ok(Elf {
            header,
            program_headers,
            section_headers,
        })
    }

    pub fn symtab_index(&self) -> Option<usize> {
        self.section_headers
            .iter()
            .position(|x| x.type_ == section_header::SectionHeaderType::SymTab)
    }

    #[inline(always)]
    fn symtab_entry_size(&self) -> usize {
        let ptr_size = <B as Bitwidth>::Ptr::N_BYTES;
        2 * ptr_size + 8
    }

    pub fn symbols(&self, inp: &[u8]) -> Result<Option<Vec<Symbol<B>>>, ElfParseError> {
        let symtab_index = if let Some(stidx) = self.symtab_index() {
            stidx
        } else {
            return Ok(None);
        };

        let symtab_header = &self.section_headers[symtab_index];

        if symtab_header.size.to_usize() % self.symtab_entry_size() != 0 {
            eprintln!("Error: Symbol table size is not a multiple of {}!", self.symtab_entry_size());
        }

        let n_symbol_tables = symtab_header.size.to_usize() / self.symtab_entry_size();

        let mut symbol_tables = Vec::with_capacity(n_symbol_tables);

        for symbol_table_idx in 0..n_symbol_tables {
            let base_offset = symtab_header.file_offset.to_usize();
            let relative_offset = symbol_table_idx * self.symtab_entry_size();

            let bytes = &inp[base_offset + relative_offset..];

            let symbol_table = Symbol::parse(bytes, self.header.endianness)?;

            symbol_tables.push(symbol_table);
        }

        Ok(Some(symbol_tables))
    }
}

