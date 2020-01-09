#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GenericParseError {
    EndOfFead,
    PtrTooLarge(u64),
}
