mod common;
mod bits;
mod endian;

mod instruction_set;

mod elf;

const EXAMPLE_BINARY: &[u8] = include_bytes!("../example_binaries/hello_elf.bin");

fn main() {
    match elf::Elf::<bits::SixtyfourBit>::parse(EXAMPLE_BINARY) {
        Ok(elf) => {
            println!("Parsed elf: {:#X?}", elf);
            process_generic_parsed(&elf);

            for section_header in &elf.section_headers {
                println!("Section header: {:?}", section_header);
                println!("Name: {:?}", String::from_utf8_lossy(section_header.get_name(EXAMPLE_BINARY, &elf)));
                if section_header.size < 64 {
                    println!("Content: {:?}", String::from_utf8_lossy(section_header.get_content(EXAMPLE_BINARY)));
                }
                println!();

            }

            match elf.symtab_tables(EXAMPLE_BINARY) {
                Ok(Some(symtab_tables)) => {
                    for table in symtab_tables {
                        println!("Table: {:#X?}", table);
                        println!("Name: {:?}", table.get_name(EXAMPLE_BINARY, &elf).map(String::from_utf8_lossy));
                        println!();
                    }
                }
                Ok(None) => {
                    eprintln!("No symbol tables found :(");
                }
                Err(e) => {
                    eprintln!("Error loading symbol table {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Could not parse elf: {:?}", e);
        }
    }
}

fn process_generic_parsed(x: &dyn common::ParsedExecutable) {
    println!("Instruction set: {:?}", x.get_instruction_set());
}
