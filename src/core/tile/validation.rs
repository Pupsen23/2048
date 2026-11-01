use super::error::Error;
use super::rule::{MAX_VALUE, MIN_VALUE};

// None is a valid option
pub fn validate_value(value: Option<u32>) -> Result<(), Error> {
    let value: u32 = match value {
        Some(some) => some,
        None => return Ok(()),
    };
    
    if !(MIN_VALUE..=MAX_VALUE).contains(&value) {
        return Err(Error::NotInRangeMinMax {
            value,
            min: MIN_VALUE,
            max: MAX_VALUE,
        });
    }

    if !value.is_power_of_two() {
        return Err(Error::NotAPowerOfTwo(value));
    }

    Ok(())
}
