use super::{GridError, TileError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    GridError(GridError),
    TileError(TileError),
}

impl From<GridError> for Error {
    fn from(err: GridError) -> Self {
        Self::GridError(err)
    }
}

impl From<TileError> for Error {
    fn from(err: TileError) -> Self {
        Self::TileError(err)
    }
}
