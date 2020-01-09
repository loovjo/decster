use crate::error::GenericParseError;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsableFile<'a> {
    data: &'a [u8],

    cursor: usize,
}

impl <'a> ParsableFile<'a> {
    pub fn new(data: &'a [u8]) -> ParsableFile<'a> {
        ParsableFile {
            data,
            cursor: 0,
        }
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<&'a [u8], GenericParseError> {
        if self.cursor + n <= self.data.len() {
            let start = self.cursor;
            self.cursor += n;
            Ok(&self.data[start..self.cursor])
        } else {
            Err(GenericParseError::EndOfFead)
        }
    }

    pub fn skip_n_bytes(&mut self, n: usize) -> Result<(), GenericParseError> {
        self.read_n_bytes(n).map(|_| ())
    }

    pub fn move_to(&mut self, position: usize) {
        self.cursor = position;
    }

    #[allow(unused)]
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }
}
