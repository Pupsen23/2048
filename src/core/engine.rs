pub mod error;

use super::{Grid, GridError, Tile, TileError, TileGenerator, TileSpawnRule};
pub use error::Error as EngineError;

pub fn fill_grid(
    grid: &mut Grid,
    tile_generator: &mut TileGenerator,
    tile_spawn_rules: &Vec<TileSpawnRule>,
) -> Result<(), EngineError> {
    for y in 0..grid.size().y() {
        for x in 0..grid.size().x() {
            let grid_tile: &mut Tile = grid.get_tile_mut(x, y)?;

            for i in 0..tile_spawn_rules.len() {
                match tile_generator.generate_with_rule(tile_spawn_rules[i]) {
                    Some(some) => {
                        grid_tile.set_value(some.value())?;
                        break;
                    }
                    None => {}
                };
            }
        }
    }

    Ok(())
}
