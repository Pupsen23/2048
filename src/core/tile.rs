pub mod error;
pub mod rule;
pub mod validation;

pub use error::Error as TileError;
use validation::validate_value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    value: Option<u32>,
}

impl Tile {
    pub const DEFAULT_VALUE: Option<u32> = None;

    pub fn value(&self) -> Option<u32> {
        self.value
    }

    pub fn set_value(&mut self, value: Option<u32>) -> Result<(), TileError> {
        validate_value(value)?;
        self.value = value;
        Ok(())
    }

    pub fn new(value: Option<u32>) -> Result<Self, TileError> {
        validate_value(value)?;
        Ok(Self { value })
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            value: Self::DEFAULT_VALUE,
        }
    }
}
