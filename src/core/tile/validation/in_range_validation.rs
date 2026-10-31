pub mod error;
pub mod validation;

pub use error::Error as InRangeError;
use validation::*;

fn validate_in_range(number: u32, range_start: u32, range_end: u32) -> Result<bool, InRangeError> {
    validate_range(range_start, range_end)?;
    Ok(number >= range_start && number < range_end)
}

pub fn validate_in_range_include(
    number: u32,
    range_start: u32,
    range_end: u32,
) -> Result<bool, InRangeError> {
    validate_in_range(number, range_start, range_end + 1)
}
