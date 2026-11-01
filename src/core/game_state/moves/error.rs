#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidLimit { limit: u32, min: u32 },
    InvalidCount { count: u32, limit: u32 },
}
