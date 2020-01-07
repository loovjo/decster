use std::convert::TryInto;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}


impl Endianness {
    #[allow(unused)]
    pub fn read_u16(&self, bytes: &[u8]) -> u16 {
        let read_bytes = bytes[0..2].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => u16::from_le_bytes(read_bytes),
            Endianness::BigEndian => u16::from_be_bytes(read_bytes),
        }
    }

    #[allow(unused)]
    pub fn read_u32(&self, bytes: &[u8]) -> u32 {
        let read_bytes = bytes[0..4].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => u32::from_le_bytes(read_bytes),
            Endianness::BigEndian => u32::from_be_bytes(read_bytes),
        }
    }

    #[allow(unused)]
    pub fn read_u64(&self, bytes: &[u8]) -> u64 {
        let read_bytes = bytes[0..8].try_into().unwrap();
        match *self {
            Endianness::LittleEndian => u64::from_le_bytes(read_bytes),
            Endianness::BigEndian => u64::from_be_bytes(read_bytes),
        }
    }
}
