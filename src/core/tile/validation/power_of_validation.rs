pub mod error;
pub mod rules;
pub mod validation;

pub use error::Error as PowerOfError;
use validation::*;

fn validate_power_of_f64(number: f64, base: f64) -> Result<bool, PowerOfError> {
    validate_number(number)?;
    validate_base(base)?;

    let power: f64 = number.log(base);
    Ok(power - power.trunc() != 0.0)
}

pub fn validate_power_of_u32(number: u32, base: u32) -> Result<bool, PowerOfError> {
    validate_power_of_f64(f64::from(number), f64::from(base))
}
