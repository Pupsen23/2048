use twenty_forty_eight::core::tile::Tile;

pub fn test() {
    assert_ne!(Tile::new(Some(1)).ok(), None);
    assert_ne!(Tile::new(Some(2)).ok(), None);
    assert_ne!(Tile::new(Some(8)).ok(), None);
    assert_ne!(Tile::new(Some(32)).ok(), None);
    
    assert_eq!(Tile::new(Some(0)).ok(), None);
    assert_eq!(Tile::new(Some(3)).ok(), None);
    assert_eq!(Tile::new(Some(7)).ok(), None);
    assert_eq!(Tile::new(Some(33)).ok(), None);
    
    let mut tile: Tile = Tile::default();
    
    assert_eq!(tile.value(), Tile::DEFAULT_VALUE);
    assert_ne!(tile.set_value(Some(1)).ok(), None);
    assert_ne!(tile.set_value(Some(2)).ok(), None);
    assert_ne!(tile.set_value(Some(8)).ok(), None);
    assert_ne!(tile.set_value(Some(32)).ok(), None);
    assert_eq!(tile.value(), Some(32));
    
    assert_eq!(tile.set_value(Some(0)).ok(), None);
    assert_eq!(tile.set_value(Some(3)).ok(), None);
    assert_eq!(tile.set_value(Some(7)).ok(), None);
    assert_eq!(tile.set_value(Some(33)).ok(), None);
    assert_ne!(tile.value(), Some(33));
}