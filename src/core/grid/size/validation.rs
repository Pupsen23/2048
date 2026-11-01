use super::error::Error;
use super::rules::*;

fn is_valid_x(x: usize) -> bool {
    x >= MIN_X
}

fn is_valid_y(y: usize) -> bool {
    y >= MIN_Y
}

pub fn validate_x(x: usize) -> Result<(), Error> {
    is_valid_x(x).ok_or(Error::InvalidX(x))
}

pub fn validate_y(y: usize) -> Result<(), Error> {
    is_valid_y(y).ok_or(Error::InvalidX(y))
}
