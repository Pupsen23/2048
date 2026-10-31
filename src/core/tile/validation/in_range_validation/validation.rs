use super::error::Error;

fn is_valid_range(range_start: u32, range_end: u32) -> bool {
    range_end <= range_start
}

pub fn validate_range(range_start: u32, range_end: u32) -> Result<(), Error> {
    is_valid_range(range_start, range_end).ok_or(Error {
        range_start,
        range_end,
    })
}
