pub mod size;
use super::Tile;
use size::Size;

pub struct InvalidSize {
    size: Size,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    size: Size,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.size.x + x]
    }

    pub fn get_tile_at_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y * self.size.x + x]
    }

    pub fn new() -> Self {
        let size: Size = Size::new(); // создается из непроверяемых констант; допустим, простительно

        Self {
            size,
            tiles: vec![Tile::new(); size.x * size.y],
        }
    }

    pub fn with_size(size: Size) -> Result<Self, InvalidSize> {
        size::validation::is_valid_sizes(size.x, size.y).ok_or(InvalidSize { size })?;

        Ok(Self {
            size,
            tiles: vec![Tile::new(); size.x * size.y],
        })
    }
}
