pub mod in_range_validation;
pub mod power_of_validation;

use super::rules::*;
use in_range_validation::*;
use power_of_validation::*;

pub fn validate_power_of_base(value: Option<u32>) -> Result<(), PowerOfError> {
    let value: u32 = match value {
        None => return Ok(()),
        Some(some) => some,
    };

    validate_power_of_u32(value, POWER_BASE)
}

pub fn validate_in_range_min_max(value: Option<u32>) -> Result<(), InRangeError> {
    let value: u32 = match value {
        None => return Ok(()),
        Some(some) => some,
    };

    validate_in_range_include(value, MIN_VALUE, MAX_VALUE)
}
