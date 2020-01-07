use crate::bits::{Bitwidth, ThirtytwoBit, SixtyfourBit};

pub trait ElfBitwidth: Bitwidth {
    const MARKER: u8;
}

impl ElfBitwidth for ThirtytwoBit {
    const MARKER: u8 = 1;
}

impl ElfBitwidth for SixtyfourBit {
    const MARKER: u8 = 2;
}
