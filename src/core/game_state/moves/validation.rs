use super::error::Error;
use super::rules::MIN_LIMIT;

fn is_valid_limit(limit: u32) -> bool {
    limit >= MIN_LIMIT
}

fn is_valid_count(count: u32, limit: u32) -> bool {
    count <= limit
}

pub fn validate_limit(limit: u32) -> Result<(), Error> {
    is_valid_limit(limit).ok_or(Error::InvalidLimit {
        limit,
        min_limit: MIN_LIMIT,
    })
}

pub fn validate_count(count: u32, limit: u32) -> Result<(), Error> {
    is_valid_count(count, limit).ok_or(Error::InvalidCount { count, limit })
}
