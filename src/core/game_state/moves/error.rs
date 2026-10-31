#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    InvalidLimit { limit: u32, min_limit: u32 },
    InvalidCount { count: u32, limit: u32 },
}
