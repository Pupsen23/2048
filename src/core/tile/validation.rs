pub mod in_range_min_max_error;
pub mod number_validation;
pub mod power_of_base_error;

use super::rules::*;
use in_range_min_max_error::InRangeMinMaxError;
use number_validation::{validate_in_range_include, validate_power_of_u32};
use power_of_base_error::PowerOfBaseError;

pub fn validate_power_of_base(value: Option<u32>) -> Result<Option<bool>, PowerOfBaseError> {
    let power_base: u32 = POWER_BASE.ok_or(PowerOfBaseError::InvalidRule)?;

    let value: u32 = match value {
        None => return Ok(None),
        Some(some) => some,
    };

    Ok(Some(validate_power_of_u32(value, power_base)?))
}

pub fn validate_in_range_min_max(value: Option<u32>) -> Result<Option<bool>, InRangeMinMaxError> {
    let min_value: u32 = MIN_VALUE.ok_or(InRangeMinMaxError::InvalidRule)?;
    let max_value: u32 = MAX_VALUE.ok_or(InRangeMinMaxError::InvalidRule)?;

    let value: u32 = match value {
        None => return Ok(None),
        Some(some) => some,
    };

    Ok(Some(validate_in_range_include(
        value, min_value, max_value,
    )?))
}
