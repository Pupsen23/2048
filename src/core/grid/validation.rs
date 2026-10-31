use super::size::Size;

pub fn is_valid_x(x: usize, size: &Size) -> bool {
    x < size.x()
}

pub fn is_valid_y(y: usize, size: &Size) -> bool {
    y < size.y()
}
