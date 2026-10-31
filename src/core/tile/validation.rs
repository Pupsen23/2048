pub mod number_validation;
use super::rules::*;
use number_validation::{is_in_range_include, is_power_of_u32, IsPowerOfError, RangeError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsPowerOfBaseError {
    RuleEqNone,
    IsPowerOfError(IsPowerOfError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsInRangeMinMaxError {
    RuleEqNone,
    RangeError(RangeError),
}

pub fn is_power_of_base(value: Option<u32>) -> Result<Option<bool>, IsPowerOfBaseError> {
    let power_base: u32 = POWER_BASE.ok_or(IsPowerOfBaseError::RuleEqNone)?;

    let value: u32 = match value {
        None => return Ok(None),
        Some(some) => some,
    };

    match is_power_of_u32(value, power_base) {
        Ok(ok) => Ok(Some(ok)),
        Err(err) => Err(IsPowerOfBaseError::IsPowerOfError(err)),
    }
}

pub fn is_in_range_min_max(value: Option<u32>) -> Result<Option<bool>, IsInRangeMinMaxError> {
    let min_value: u32 = MIN_VALUE.ok_or(IsInRangeMinMaxError::RuleEqNone)?;
    let max_value: u32 = MAX_VALUE.ok_or(IsInRangeMinMaxError::RuleEqNone)?;

    let value: u32 = match value {
        None => return Ok(None),
        Some(some) => some,
    };

    match is_in_range_include(value, min_value, max_value) {
        Ok(ok) => Ok(Some(ok)),
        Err(err) => Err(IsInRangeMinMaxError::RangeError(err)),
    }
}

pub fn is_default(value: Option<u32>) -> bool {
    value == DEFAULT_VALUE
}
