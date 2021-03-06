use std::env::args;
use std::fs::File;
use std::io::Read;

mod common;
mod bits;
mod endian;

mod error;

mod instruction_set;

mod elf;

mod parsable_file;
use parsable_file::ParsableFile;

fn main() {
    let path = args().skip(1).next();
    let path = path.unwrap_or("example_binaries/hello_elf.bin".to_string());

    let mut file = if let Ok(file) = File::open(path.clone()) {
        file
    } else {
        eprintln!("Could not open file {}", path);
        return;
    };

    let mut contents = Vec::new();
    if let Err(e) = file.read_to_end(&mut contents) {
        eprintln!("Could not read file {}, error {}", path, e);
        return;
    }
    let contents = ParsableFile::new(&*contents);

    let mut parsed = None;
    match handle_elf_64bit(contents) {
        Ok(res) => {
            parsed = Some(Box::new(res));
        }
        Err(e) => {
            eprintln!("Error parsing ELF {}: {:?}", path, e);
        }
    }

    if let Some(parsed) = parsed {
        process_generic_parsed(&*parsed);
    } else {
        eprintln!("Could not find the format of {}", path);
    }
}

fn handle_elf_64bit(mut contents: ParsableFile<'_>) -> Result<impl common::ParsedExecutable, elf::ElfParseError> {
    let elf = elf::Elf::<bits::SixtyfourBit>::parse(&mut contents)?;
    println!("Parsed elf: {:#X?}", elf);

    for (i, section_header) in elf.section_headers.iter().enumerate() {
        println!("Section header #{:X}: {:X?}", i, section_header);
        println!("Name: {:?}", String::from_utf8_lossy(section_header.get_name(&mut contents, &elf)?));
        if section_header.size < 32 {
            println!("Content: {:?}", String::from_utf8_lossy(section_header.get_content(&mut contents)?));
        }
        println!();

    }

    match elf.symtab_index() {
        Some(idx) => {
            println!("Found strtab at 0x{:x}", idx);
            let symbols = elf.symbols(&mut contents, idx)?;
            for (i, symbol) in symbols.iter().enumerate() {
                println!("Symbol 0x{:X}: {:X?}", i, symbol);
                println!("Name: {:?}", symbol.get_name(&mut contents, &elf, &elf.section_headers[idx])?.map(String::from_utf8_lossy));
                println!();
            }
        }
        None => {
            eprintln!("No symbols found :(");
        }
    }

    for reloc_table_idx in elf.reloc_tables_inds() {
        println!("---\n");

        let table = &elf.section_headers[reloc_table_idx];
        println!("Found relocation table #0x{:X}: {:?}", reloc_table_idx, String::from_utf8_lossy(table.get_name(&mut contents, &elf)?));

        let relocations = elf.relocations(&mut contents, reloc_table_idx)?;
        for relocation in relocations {
            println!("Relocation: {:X?}", relocation);
            let symbol = relocation.get_symbol(&mut contents, &elf, &elf.section_headers[reloc_table_idx])?;
            println!("Name from symbol: {:?}", symbol.get_name(&mut contents, &elf, &elf.section_headers[table.link])?.map(String::from_utf8_lossy));

            println!();
        }
    }

    Ok(elf)
}

fn process_generic_parsed(x: &dyn common::ParsedExecutable) {
    println!("Instruction set: {:?}", x.get_instruction_set());
}
