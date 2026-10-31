pub mod rules;
pub mod validation;

use rules::{DEFAULT_X, DEFAULT_Y};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Size {
    pub fn new() -> Self {
        Self {
            x: DEFAULT_X,
            y: DEFAULT_Y,
        }
    }

    pub fn with(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
