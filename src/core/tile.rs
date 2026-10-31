pub mod error;
pub mod rules;
pub mod validation;

pub use error::Error as TileError;
use validation::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    value: Option<u32>,
}

impl Tile {
    pub const DEFAULT_VALUE: Option<u32> = None;

    pub fn value(&self) -> Option<u32> {
        self.value
    }

    pub fn set_value(&mut self, value: Option<u32>) -> Result<(), TileError> {
        Self::ensure_validity(value)?;
        self.value = value;
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            value: Self::DEFAULT_VALUE,
        }
    }

    pub fn with(value: Option<u32>) -> Result<Self, TileError> {
        Self::ensure_validity(value)?;
        Ok(Self { value })
    }

    fn ensure_validity(value: Option<u32>) -> Result<(), TileError> {
        validate_in_range_min_max(value)?;
        validate_power_of_base(value)?;
        Ok(())
    }
}
