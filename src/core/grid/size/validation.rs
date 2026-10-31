use super::rules::*;

pub fn is_valid_x(x: usize) -> bool {
    x >= MIN_X
}

pub fn is_valid_y(y: usize) -> bool {
    y >= MIN_Y
}
