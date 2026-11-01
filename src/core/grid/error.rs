#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidX { x: usize, max: usize },
    InvalidY { y: usize, max: usize },
}
