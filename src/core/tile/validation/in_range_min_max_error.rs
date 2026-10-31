use super::number_validation::InRangeError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InRangeMinMaxError {
    InvalidRule,
    IsInRangeError(InRangeError),
}

impl From<InRangeError> for InRangeMinMaxError {
    fn from(err: InRangeError) -> Self {
        InRangeMinMaxError::IsInRangeError(err)
    }
}
