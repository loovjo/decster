use crate::bits::{PtrType, Bitwidth};
use crate::common::ParsedExecutable;
use crate::instruction_set::InstructionSet;
use crate::parsable_file::ParsableFile;

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
use self::section_header::{SectionHeader, SectionHeaderType};

mod symbol;
use self::symbol::Symbol;

mod reloc;
use reloc::Relocation;

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
    pub fn parse(inp: &mut ParsableFile<'_>) -> Result<Elf<B>, ElfParseError> {
        let magic = inp.read_n_bytes(4)?;
        if magic != &ELF_MAGIC {
            return Err(ElfParseError::WrongMagic(magic.to_vec()))
        }

        let header = Header::<B>::parse(inp)?;

        // Read program headers
        let mut program_headers = Vec::with_capacity(header.program_header_n_entries as usize);

        let mut inp_for_ph = inp.clone();
        inp_for_ph.move_to(header.program_header_offset.to_usize()?);

        for _ in 0..header.program_header_n_entries {
            let program_header = ProgramHeader::parse(&mut inp_for_ph, header.endianness)?;

            program_headers.push(program_header);
        }

        // Read section headers
        let mut section_headers = Vec::with_capacity(header.section_header_n_entries as usize);

        let mut inp_for_sh = inp.clone();
        inp_for_sh.move_to(header.section_header_offset.to_usize()?);

        for _ in 0..header.section_header_n_entries {
            let section_header = SectionHeader::parse(&mut inp_for_sh, header.endianness)?;

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
            .position(|x| x.type_ == SectionHeaderType::SymTab)
    }

    #[inline(always)]
    fn symtab_entry_size(&self) -> usize {
        let ptr_size = <B as Bitwidth>::Ptr::N_BYTES;
        2 * ptr_size + 8
    }

    pub fn symbols(&self, inp: &mut ParsableFile<'_>, symtab_index: usize) -> Result<Vec<Symbol<B>>, ElfParseError> {
        let symtab_header = &self.section_headers[symtab_index];
        assert!(symtab_header.type_ == SectionHeaderType::SymTab || symtab_header.type_ == SectionHeaderType::DynSym);

        if symtab_header.size.to_usize()? % self.symtab_entry_size() != 0 {
            eprintln!("Error: Symbol table size is not a multiple of {}!", self.symtab_entry_size());
        }

        let n_symbol_tables = symtab_header.size.to_usize()? / self.symtab_entry_size();

        let mut symbol_tables = Vec::with_capacity(n_symbol_tables);

        let mut inp_for_symbol_table = inp.clone();
        inp_for_symbol_table.move_to(symtab_header.file_offset.to_usize()?);

        for _ in 0..n_symbol_tables {
            let symbol_table = Symbol::parse(&mut inp_for_symbol_table, self.header.endianness)?;

            symbol_tables.push(symbol_table);
        }

        Ok(symbol_tables)
    }

    // Maybe we should make this function return the sections?
    pub fn reloc_tables_inds(&self) -> Vec<usize> {
        self.section_headers
            .iter()
            .enumerate()
            .filter(|(_i, x)| x.type_ == SectionHeaderType::Rel || x.type_ == SectionHeaderType::Rela)
            .map(|(i, _x)| i)
            .collect()
    }

    pub fn relocations(&self, inp: &mut ParsableFile<'_>) -> Result<Vec<Relocation<B>>, ElfParseError> {
        let mut relocs = Vec::new();

        for reloc_table_idx in self.reloc_tables_inds() {
            let reloc_table = &self.section_headers[reloc_table_idx];

            let has_addend = match reloc_table.type_ {
                SectionHeaderType::Rel => false,
                SectionHeaderType::Rela => true,
                _ => unreachable!(),
            };

            if reloc_table.size.to_usize()? % reloc_table.entry_size.to_usize()? != 0 {
                return Err(ElfParseError::InvalidRelocationTableSize(reloc_table_idx));
            }

            let mut inp_for_reloc = inp.clone();
            inp_for_reloc.move_to(reloc_table.file_offset.to_usize()?);

            for reloc_entry_idx in 0..reloc_table.size.to_usize()? / reloc_table.entry_size.to_usize()? {
                let ptr_before = inp_for_reloc.get_cursor();
                let reloc = Relocation::parse(&mut inp_for_reloc, self.header.endianness, has_addend)?;
                let delta = inp_for_reloc.get_cursor() - ptr_before;

                if delta != reloc_table.entry_size.to_usize()? {
                    return Err(ElfParseError::InvalidRelocationEntrySize(reloc_table_idx, reloc_entry_idx));
                }
                relocs.push(reloc);
            }
        }

        Ok(relocs)
    }
}

