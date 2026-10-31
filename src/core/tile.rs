pub mod rules;
pub mod validation;

use rules::DEFAULT_VALUE;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    pub value: Option<u32>,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            value: DEFAULT_VALUE,
        }
    }

    pub fn with(value: Option<u32>) -> Self {
        Self { value }
    }
}
