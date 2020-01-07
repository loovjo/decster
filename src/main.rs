mod common;
mod bits;
mod endian;

mod instruction_set;

const EXAMPLE_BINARY: &[u8] = include_bytes!("../example_binaries/hello_elf.bin");

fn main() {
}

fn process_generic_parsed(x: &dyn common::ParsedExecutable) {
    println!("Instruction set: {:?}", x.get_instruction_set());
}
