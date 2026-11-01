use super::error::Error;
use super::GridSize;

pub fn validate_x_y(x: usize, y: usize, size: GridSize) -> Result<(), Error> {
    if x >= size.x() {
        Err(Error::InvalidX {
            x,
            max: size.x() - 1,
        })
    } else if y >= size.y() {
        Err(Error::InvalidY {
            y,
            max: size.y() - 1,
        })
    } else {
        Ok(())
    }
}
