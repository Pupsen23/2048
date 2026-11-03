/*use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    boardSize: usize,
    movesLimit: u32,
    maxRewardCashback: u32,
    maxRewardAllowedRate: Decimal,
}

pub fn main() -> std::io::Result<()> {
    let file = File::open("/mnt/data/projects/rust/twenty_forty_eight/examples/config.json")?;
    let buf_reader = BufReader::new(file);
    
    let serde: Vec<String> = serde_json::from_reader(buf_reader)?;
    println!("{serde:#?}");
    Ok(())
}*/

/*pub struct Config {
    grid_size: GridSize,
    game_state_moves: GameStateMoves,
    tile_spawn_rules: Vec<TileSpawnRule>,
}*/

use rust_decimal::Decimal;
use twenty_forty_eight::core::*;
use twenty_forty_eight::core::engine::fill_grid;

pub fn main() {
     let mut grid: Grid = Grid::default();
     let mut tile_generator: TileGenerator = TileGenerator::default();
     let tile = match Tile::new(Some(2)) {
         Ok(ok) => ok,
         Err(err) => panic!("{err:#?}"),  
     };
     let probability: Decimal = Decimal::from(30);
     let tile_spawn_rule: TileSpawnRule =  match TileSpawnRule::new(tile, probability) {
         Ok(ok) => ok,
         Err(err) => panic!("{err:#?}"),
     };
     let tile_spawn_rules: Vec<TileSpawnRule> = vec![tile_spawn_rule; 2];
     
     match fill_grid(&mut grid, &mut tile_generator, &tile_spawn_rules) {
         Ok(_) => {},
         Err(err) => panic!("{err:#?}"),
     };
     
     println!("{grid:#?}");
}