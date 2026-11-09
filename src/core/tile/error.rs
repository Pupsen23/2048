#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ValueError(ValueError),
    MaxValueError(MaxValueError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueError {
    NotAPowerOfTwo(u32),
    NotInRangeMinMax { value: u32, min: u32, max: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxValueError {
    NotAPowerOfTwo(u32),
    LowerThanMin { min: u32, max_value: u32 },
}

impl From<ValueError> for Error {
    fn from(err: ValueError) -> Self {
        Error::ValueError(err)
    }
}

impl From<MaxValueError> for Error {
    fn from(err: MaxValueError) -> Self {
        Error::MaxValueError(err)
    }
}
