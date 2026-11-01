pub mod error;
pub mod size;
pub mod validation;

use super::Tile;
pub use error::Error as GridError;
pub use size::Size as GridSize;
use validation::validate_x_y;

pub struct Grid {
    size: GridSize,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn size(&self) -> GridSize {
        self.size
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Result<Tile, GridError> {
        validate_x_y(x, y, self.size)?;
        Ok(self.tiles[y * self.size.x() + x])
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Result<&mut Tile, GridError> {
        validate_x_y(x, y, self.size)?;
        Ok(&mut self.tiles[y * self.size.x() + x])
    }

    pub fn new(size: GridSize) -> Self {
        Self {
            size,
            tiles: vec![Tile::default(); size.x() * size.y()],
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        let size: GridSize = GridSize::default();

        Self {
            size,
            tiles: vec![Tile::default(); size.x() * size.y()],
        }
    }
}
