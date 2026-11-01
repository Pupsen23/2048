use super::error::Error;
use super::rule::{MAX_PROBABILITY, MIN_PROBABILITY};
use rust_decimal::Decimal;

pub fn validate_probability(probability: Decimal) -> Result<(), Error> {
    if probability < MIN_PROBABILITY || probability > MAX_PROBABILITY {
        Err(Error::InvalidProbability {
            probability,
            min: MIN_PROBABILITY,
            max: MAX_PROBABILITY,
        })
    } else {
        Ok(())
    }
}
