pub mod error;
pub mod rule;
pub mod validation;

use super::Tile;
pub use error::Error as SpawnRuleError;
use rust_decimal::Decimal;
use validation::validate_probability;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpawnRule {
    tile: Tile,
    probability: Decimal,
}

impl SpawnRule {
    pub const DEFAULT_PROBABILITY: Decimal = Decimal::TEN;

    pub fn tile(&self) -> Tile {
        self.tile
    }

    pub fn probability(&self) -> Decimal {
        self.probability
    }

    pub fn set_tile(&mut self, tile: Tile) {
        self.tile = tile;
    }

    pub fn set_probability(&mut self, probability: Decimal) -> Result<(), SpawnRuleError> {
        validate_probability(probability)?;
        self.probability = probability;
        Ok(())
    }

    pub fn new(tile: Tile, probability: Decimal) -> Result<Self, SpawnRuleError> {
        validate_probability(probability)?;
        Ok(Self { tile, probability })
    }
}

impl Default for SpawnRule {
    fn default() -> Self {
        Self {
            tile: Tile::default(),
            probability: Self::DEFAULT_PROBABILITY,
        }
    }
}
