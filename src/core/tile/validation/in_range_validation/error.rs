#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeError {
    pub(super) range_start: u32,
    pub(super) range_end: u32,
}

impl RangeError {
    pub fn range_start(&self) -> u32 {
        self.range_start
    }

    pub fn range_end(&self) -> u32 {
        self.range_end
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    RangeError(RangeError),
    NotInRange {
        number: u32,
        range_start: u32,
        range_end: u32,
    },
}

impl From<RangeError> for Error {
    fn from(err: RangeError) -> Self {
        Error::RangeError(err)
    }
}
