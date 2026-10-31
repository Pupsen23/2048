use super::validation::in_range_validation::InRangeError;
use super::validation::power_of_validation::PowerOfError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    PowerOfBaseError(PowerOfError),
    InRangeMinMaxError(InRangeError),
}

impl From<PowerOfError> for Error {
    fn from(err: PowerOfError) -> Self {
        Error::PowerOfBaseError(err)
    }
}

impl From<InRangeError> for Error {
    fn from(err: InRangeError) -> Self {
        Error::InRangeMinMaxError(err)
    }
}
