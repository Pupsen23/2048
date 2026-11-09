use super::error::{MaxValueError, ValueError};
use super::rule::MIN_VALUE;

// None is a valid option
pub fn validate_value(value: Option<u32>, max_value: u32) -> Result<(), ValueError> {
    let value: u32 = match value {
        Some(some) => some,
        None => return Ok(()),
    };

    if !(MIN_VALUE..=max_value).contains(&value) {
        return Err(ValueError::NotInRangeMinMax {
            value,
            min: MIN_VALUE,
            max: max_value,
        });
    }

    if !value.is_power_of_two() {
        return Err(ValueError::NotAPowerOfTwo(value));
    }

    Ok(())
}

pub fn validate_max_value(max_value: u32) -> Result<(), MaxValueError> {
    if max_value < MIN_VALUE {
        Err(MaxValueError::LowerThanMin {
            min: MIN_VALUE,
            max_value,
        })
    } else if !max_value.is_power_of_two() {
        Err(MaxValueError::NotAPowerOfTwo(max_value))
    } else {
        Ok(())
    }
}
