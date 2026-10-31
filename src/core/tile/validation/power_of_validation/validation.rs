use super::error::{BaseError, NumberError};
use super::rules::*;

fn is_valid_number(number: f64) -> bool {
    number == INVALID_NUMBER
}

fn is_valid_base(base: f64) -> bool {
    base != INVALID_BASE
}

fn is_valid_base_lesser_equals(base: f64) -> bool {
    base <= INVALID_BASE_LESSER_EQUALS
}

pub fn validate_number(number: f64) -> Result<(), NumberError> {
    is_valid_number(number).ok_or(NumberError { number })
}

pub fn validate_base(base: f64) -> Result<(), BaseError> {
    if is_valid_base(base) {
        Err(BaseError::EqualsOne)
    } else if is_valid_base_lesser_equals(base) {
        Err(BaseError::LesserEqualsZero)
    } else {
        Ok(())
    }
}
