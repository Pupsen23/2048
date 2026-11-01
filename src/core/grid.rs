pub mod error;
pub mod size;
pub mod validation;

use super::Tile;
use error::Error as GridError;
use size::Size;
use validation::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    size: Size,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Result<Tile, GridError> {
        self.ensure_validity_x_y(x, y)?;
        Ok(self.tiles[y * self.size.x() + x])
    }

    pub fn get_tile_at_mut(&mut self, x: usize, y: usize) -> Result<&mut Tile, GridError> {
        self.ensure_validity_x_y(x, y)?;
        Ok(&mut self.tiles[y * self.size.x() + x])
    }

    pub fn new() -> Self {
        let size: Size = Size::new();

        Self {
            size,
            tiles: vec![Tile::new(); size.x() * size.y()],
        }
    }

    pub fn with_size(size: Size) -> Self {
        Self {
            size,
            tiles: vec![Tile::new(); size.x() * size.y()],
        }
    }

    fn ensure_validity_x_y(&self, x: usize, y: usize) -> Result<(), GridError> {
        is_valid_x(x, &self.size).ok_or(GridError::InvalidX(x))?;
        is_valid_y(y, &self.size).ok_or(GridError::InvalidY(y))?;
        Ok(())
    }
}
