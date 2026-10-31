use super::number_validation::PowerOfError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerOfBaseError {
    InvalidRule,
    PowerOfError(PowerOfError),
}

impl From<PowerOfError> for PowerOfBaseError {
    fn from(err: PowerOfError) -> Self {
        PowerOfBaseError::PowerOfError(err)
    }
}
