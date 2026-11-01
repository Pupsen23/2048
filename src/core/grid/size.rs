pub mod error;
pub mod rules;
pub mod validation;

pub use error::Error as SizeError;
use validation::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    x: usize,
    y: usize,
}

impl Size {
    pub const DEFAULT_X: usize = 4;
    pub const DEFAULT_Y: usize = 4;

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn set_x(&mut self, x: usize) -> Result<(), SizeError> {
        validate_x(x)?;
        self.x = x;
        Ok(())
    }

    pub fn set_y(&mut self, y: usize) -> Result<(), SizeError> {
        validate_y(y)?;
        self.y = y;
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            x: Self::DEFAULT_X,
            y: Self::DEFAULT_Y,
        }
    }

    pub fn with(x: usize, y: usize) -> Result<Self, SizeError> {
        validate_x(x)?;
        validate_y(y)?;
        Ok(Self { x, y })
    }
}
