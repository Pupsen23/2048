use twenty_forty_eight::core::grid::{Grid, GridSize};

pub fn test_size() {
    assert_ne!(GridSize::new(2, 2).ok(), None);
    assert_ne!(GridSize::new(3, 4).ok(), None);
    assert_ne!(GridSize::new(4, 3).ok(), None);
    
    assert_eq!(GridSize::new(1, 1).ok(), None);
    assert_eq!(GridSize::new(2, 1).ok(), None);
    assert_eq!(GridSize::new(1, 2).ok(), None);
    
    let mut grid_size: GridSize = GridSize::default();
    
    assert_eq!(grid_size.x(), GridSize::DEFAULT_X);
    assert_ne!(grid_size.set_x(2).ok(), None);
    assert_ne!(grid_size.set_x(3).ok(), None);
    assert_ne!(grid_size.set_x(4).ok(), None);
    assert_eq!(grid_size.x(), 4);
    
    assert_eq!(grid_size.set_x(1).ok(), None);
    assert_eq!(grid_size.set_x(0).ok(), None);
    assert_ne!(grid_size.x(), 0);
    
    assert_eq!(grid_size.y(), GridSize::DEFAULT_Y);
    assert_ne!(grid_size.set_y(2).ok(), None);
    assert_ne!(grid_size.set_y(3).ok(), None);
    assert_ne!(grid_size.set_y(4).ok(), None);
    assert_eq!(grid_size.y(), 4);
    
    assert_eq!(grid_size.set_y(1).ok(), None);
    assert_eq!(grid_size.set_y(0).ok(), None);
    assert_ne!(grid_size.y(), 0);
}

pub fn test() {
    test_size();
    
    //...
}