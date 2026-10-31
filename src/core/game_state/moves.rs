pub mod rules;
pub mod validation;

use validation::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovesError {
    InvalidLimit(u32),
    InvalidCount(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
        is_valid_count(count, self.limit).ok_or(MovesError::InvalidCount(count))?;
        self.count = count;
        Ok(())
    }
    
    pub fn new() -> Self {
        Self {
            limit: Self::DEFAULT_LIMIT,
            count: Self::DEFAULT_COUNT,
        }
    }

    pub fn with(count: u32, limit: u32) -> Result<Self, MovesError> {
        is_valid_limit(limit).ok_or(MovesError::InvalidLimit(limit))?;
        is_valid_count(count, limit).ok_or(MovesError::InvalidCount(count))?;
        Ok(Self { limit, count })
    }
}
