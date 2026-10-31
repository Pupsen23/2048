#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Error {
    pub range_start: u32,
    pub range_end: u32,
}
