use super::error::RangeError;

fn is_valid_range(range_start: u32, range_end: u32) -> bool {
    range_start < range_end
}

pub fn validate_range(range_start: u32, range_end: u32) -> Result<(), RangeError> {
    is_valid_range(range_start, range_end).ok_or(RangeError {
        range_start,
        range_end,
    })
}
