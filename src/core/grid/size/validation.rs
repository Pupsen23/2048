use super::error::Error;
use super::rule::{MIN_X, MIN_Y};

pub fn validate_x(x: usize) -> Result<(), Error> {
    if x < MIN_X {
        Err(Error::InvalidX { x, min: MIN_X })
    } else {
        Ok(())
    }
}

pub fn validate_y(y: usize) -> Result<(), Error> {
    if y < MIN_Y {
        Err(Error::InvalidY { y, min: MIN_Y })
    } else {
        Ok(())
    }
}
