pub mod spawn_rule;

use rand::rngs::StdRng;
use rand::{random, RngExt, SeedableRng};
use rust_decimal::Decimal;

use super::tile::rule::{MAX_VALUE, MIN_VALUE};
use super::Tile;
pub use spawn_rule::{SpawnRule as TileSpawnRule, SpawnRuleError as TileSpawnRuleError};

#[derive(Debug)]
pub struct TileGenerator {
    seed: u64,
    rng: StdRng,
}

impl TileGenerator {
    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: SeedableRng::seed_from_u64(seed),
        }
    }

    // somnitel'no, no okay. eto vryad li budet. and min/max const values import
    pub fn generate_random_or_default(&mut self) -> Tile {
        let random_value = self.rng.random_range(MIN_VALUE..=MAX_VALUE);

        match Tile::new(Some(random_value)) {
            Ok(ok) => ok,
            Err(_) => Tile::default(),
        }
    }

    // None indicates an out of luck
    pub fn generate_with_rule(&mut self, spawn_rule: TileSpawnRule) -> Option<Tile> {
        let random_value: u8 = self.rng.random_range(0..=100);
        let random_value: Decimal = Decimal::from(random_value);

        if spawn_rule.probability() > random_value {
            Some(spawn_rule.tile())
        } else {
            None
        }
    }

    pub fn generate_with_rule_or_default(&mut self, spawn_rule: TileSpawnRule) -> Tile {
        match self.generate_with_rule(spawn_rule) {
            Some(some) => some,
            None => Tile::default(),
        }
    }
}

impl Default for TileGenerator {
    fn default() -> Self {
        let seed: u64 = random();

        Self {
            seed,
            rng: SeedableRng::seed_from_u64(seed),
        }
    }
}
