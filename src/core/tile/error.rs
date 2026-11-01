#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NotAPowerOfTwo(u32),
    NotInRangeMinMax { value: u32, min: u32, max: u32 },
}
