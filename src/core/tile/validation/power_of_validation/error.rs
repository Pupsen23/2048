#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberError {
    pub number: f64,
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
