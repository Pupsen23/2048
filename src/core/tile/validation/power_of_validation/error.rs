#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberError {
    pub(super) number: f64,
}

impl NumberError {
    pub fn number(&self) -> f64 {
        self.number
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseError {
    EqualsOne,
    LesserEqualsZero,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    NumberError(NumberError),
    BaseError(BaseError),
    NotPowerOf { number: f64, base: f64 },
}

impl From<NumberError> for Error {
    fn from(err: NumberError) -> Self {
        Error::NumberError(err)
    }
}

impl From<BaseError> for Error {
    fn from(err: BaseError) -> Self {
        Error::BaseError(err)
    }
}
