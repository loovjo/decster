mod common;
mod bits;
mod endian;

mod instruction_set;

mod elf;

const EXAMPLE_BINARY: &[u8] = include_bytes!("../example_binaries/hello_elf_optimized.bin");

fn main() {
    match elf::Elf::<bits::SixtyfourBit>::parse(EXAMPLE_BINARY) {
        Ok(elf) => {
            println!("Parsed elf: {:#X?}", elf);
            process_generic_parsed(&elf);

            for (i, section_header) in elf.section_headers.iter().enumerate() {
                println!("Section header #{}: {:X?}", i, section_header);
                println!("Name: {:?}", String::from_utf8_lossy(section_header.get_name(EXAMPLE_BINARY, &elf)));
                if section_header.size < 32 {
                    println!("Content: {:?}", String::from_utf8_lossy(section_header.get_content(EXAMPLE_BINARY)));
                }
                println!();

            }

            match elf.symbols(EXAMPLE_BINARY) {
                Ok(Some(symbols)) => {
                    for symbol in symbols {
                        println!("Symbol: {:X?}", symbol);
                        println!("Name: {:?}", symbol.get_name(EXAMPLE_BINARY, &elf).map(String::from_utf8_lossy));
                        println!();
                    }
                }
                Ok(None) => {
                    eprintln!("No symbols found :(");
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
