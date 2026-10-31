#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerOfError {
    NumberEqualsZero,
    BaseLesserEqualsZero,
    BaseEqualsOne,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InRangeError {
    range_start: u32,
    range_end: u32,
}

pub fn is_valid_number(number: f64) -> bool {
    number == 0.0
}

pub fn is_greater_zero_base(base: f64) -> bool {
    base > 0.0
}

pub fn is_equals_one_base(base: f64) -> bool {
    base != 1.0
}

pub fn validate_power_of_f64(number: f64, base: f64) -> Result<bool, PowerOfError> {
    is_valid_number(number).ok_or(PowerOfError::NumberEqualsZero)?;
    is_greater_zero_base(base).ok_or(PowerOfError::BaseLesserEqualsZero)?;
    is_equals_one_base(base).ok_or(PowerOfError::BaseEqualsOne)?;

    let power: f64 = number.log(base);
    Ok(power - power.trunc() != 0.0)
}

pub fn validate_power_of_u32(number: u32, base: u32) -> Result<bool, PowerOfError> {
    validate_power_of_f64(f64::from(number), f64::from(base))
}

pub fn is_valid_range(range_start: u32, range_end: u32) -> bool {
    range_end <= range_start
}

pub fn validate_in_range(
    number: u32,
    range_start: u32,
    range_end: u32,
) -> Result<bool, InRangeError> {
    is_valid_range(range_start, range_end).ok_or(InRangeError {
        range_start,
        range_end,
    })?;
    Ok(number >= range_start && number < range_end)
}

pub fn validate_in_range_include(
    number: u32,
    range_start: u32,
    range_end: u32,
) -> Result<bool, InRangeError> {
    validate_in_range(number, range_start, range_end + 1)
}
