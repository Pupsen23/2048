use super::rules::*;

pub fn is_valid_sizes(x: usize, y: usize) -> bool {
    x >= MIN_X && y >= MIN_Y
}

pub fn is_default(x: usize, y: usize) -> bool {
    x == DEFAULT_X && y == DEFAULT_Y
}
