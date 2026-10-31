use super::validation::in_range_min_max_error::InRangeMinMaxError;
use super::validation::power_of_base_error::PowerOfBaseError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileError {
    PowerOfBaseError(PowerOfBaseError),
    InRangeMinMaxError(InRangeMinMaxError),
}

impl From<PowerOfBaseError> for TileError {
    fn from(err: PowerOfBaseError) -> Self {
        TileError::PowerOfBaseError(err)
    }
}

impl From<InRangeMinMaxError> for TileError {
    fn from(err: InRangeMinMaxError) -> Self {
        TileError::InRangeMinMaxError(err)
    }
}
