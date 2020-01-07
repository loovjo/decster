#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused, non_camel_case_types)]
pub enum InstructionSet {
    NotSpecified,
    SPARC,
    X86,
    MIPS,
    PowerPC,
    S390,
    ARM,
    SuperH,
    IA_64,
    X86_64,
    AArch64,
    RISC_V,
}

