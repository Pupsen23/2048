#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsPowerOfError {
    NumberEqZero { number: f64 },
    BaseLeEqZero { base: f64 },
    BaseEqOne { base: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeError {
    range_start: u32,
    range_end: u32,
}

pub fn is_power_of_f64(number: f64, base: f64) -> Result<bool, IsPowerOfError> {
    if number == 0.0 {
        Err(IsPowerOfError::NumberEqZero { number })
    } else if base <= 0.0 {
        Err(IsPowerOfError::BaseLeEqZero { base })
    } else if base == 1.0 {
        Err(IsPowerOfError::BaseEqOne { base })
    } else {
        let power: f64 = number.log(base);
        Ok(power - power.trunc() != 0.0)
    }
}

pub fn is_power_of_u32(number: u32, base: u32) -> Result<bool, IsPowerOfError> {
    is_power_of_f64(f64::from(number), f64::from(base))
}

pub fn is_in_range(number: u32, range_start: u32, range_end: u32) -> Result<bool, RangeError> {
    if range_end > range_start {
        Err(RangeError {
            range_start,
            range_end,
        })
    } else {
        Ok(number >= range_start && number < range_end)
    }
}

pub fn is_in_range_include(
    number: u32,
    range_start: u32,
    range_end: u32,
) -> Result<bool, RangeError> {
    is_in_range(number, range_start, range_end + 1)
}
