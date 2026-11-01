pub mod error;
pub mod rule;
pub mod validation;

pub use error::Error as MovesError;
use validation::{validate_count, validate_limit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moves {
    limit: u32,
    count: u32,
}

impl Moves {
    pub const DEFAULT_LIMIT: u32 = 5;
    pub const DEFAULT_COUNT: u32 = 0;

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn set_count(&mut self, count: u32) -> Result<(), MovesError> {
        validate_count(count, self.limit)?;
        self.count = count;
        Ok(())
    }

    pub fn new(limit: u32, count: u32) -> Result<Self, MovesError> {
        validate_limit(limit)?;
        validate_count(count, limit)?;
        Ok(Self { limit, count })
    }
}

impl Default for Moves {
    fn default() -> Self {
        Self {
            limit: Self::DEFAULT_LIMIT,
            count: Self::DEFAULT_COUNT,
        }
    }
}
