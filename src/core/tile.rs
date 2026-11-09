pub mod error;
pub mod rule;
pub mod validation;

pub use error::Error as TileError;
use validation::{validate_max_value, validate_value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    value: Option<u32>,
    max_value: u32,
}

impl Tile {
    pub const DEFAULT_VALUE: Option<u32> = None;
    pub const DEFAULT_MAX_VALUE: u32 = 32;

    pub fn value(&self) -> Option<u32> {
        self.value
    }

    pub fn set_value(&mut self, value: Option<u32>) -> Result<(), TileError> {
        validate_value(value, self.max_value)?;
        self.value = value;
        Ok(())
    }

    pub fn new(value: Option<u32>, max_value: u32) -> Result<Self, TileError> {
        validate_max_value(max_value)?;
        validate_value(value, max_value)?;
        Ok(Self { value, max_value })
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            value: Self::DEFAULT_VALUE,
            max_value: Self::DEFAULT_MAX_VALUE,
        }
    }
}
