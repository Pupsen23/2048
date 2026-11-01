#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidX { x: usize, min: usize },
    InvalidY { y: usize, min: usize },
}
