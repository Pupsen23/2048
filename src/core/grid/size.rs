pub mod rules;
pub mod validation;

use validation::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeError {
    InvalidX(usize),
    InvalidY(usize),
}

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
        is_valid_x(x).ok_or(SizeError::InvalidX(x))?;
        self.x = x;
        Ok(())
    }

    pub fn set_y(&mut self, y: usize) -> Result<(), SizeError> {
        is_valid_y(y).ok_or(SizeError::InvalidY(y))?;
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
        is_valid_x(x).ok_or(SizeError::InvalidX(x))?;
        is_valid_y(y).ok_or(SizeError::InvalidY(y))?;
        Ok(Self { x, y })
    }
}
