use super::error::{BaseError, NumberError};
use super::rules::*;

fn is_valid_number(number: f64) -> bool {
    number != INVALID_NUMBER
}

fn is_valid_base(base: f64) -> bool {
    base != INVALID_BASE
}

fn is_positive_base(base: f64) -> bool {
    base > INVALID_BASE_LESSER_EQUALS
}

pub fn validate_number(number: f64) -> Result<(), NumberError> {
    is_valid_number(number).ok_or(NumberError { number })
}

pub fn validate_base(base: f64) -> Result<(), BaseError> {
    is_valid_base(base).ok_or(BaseError::EqualsOne)?;
    is_positive_base(base).ok_or(BaseError::LesserEqualsZero)?;
    Ok(())
}
