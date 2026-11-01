use twenty_forty_eight::core::tile::*;

pub fn test() {
    assert_eq!(Tile::with(Some(0)).ok(), None);
    assert_eq!(Tile::with(Some(3)).ok(), None);
    assert_ne!(Tile::with(Some(1)).ok(), None);
    assert_ne!(Tile::with(Some(32)).ok(), None);
    assert_eq!(Tile::with(Some(25)).ok(), None);
    
    let mut tile: Tile = Tile::new();
    assert_eq!(tile.value(), None);
    assert_eq!(tile.set_value(Some(0)).ok(), None);
    assert_eq!(tile.set_value(Some(3)).ok(), None);
    assert_ne!(tile.set_value(Some(1)).ok(), None);
    assert_eq!(tile.value(), Some(1));
    assert_ne!(tile.set_value(Some(32)).ok(), None);
    assert_eq!(tile.set_value(Some(25)).ok(), None);
}