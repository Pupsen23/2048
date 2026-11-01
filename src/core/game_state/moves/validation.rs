use super::error::Error;
use super::rule::MIN_LIMIT;

pub fn validate_limit(limit: u32) -> Result<(), Error> {
    if limit < MIN_LIMIT {
        Err(Error::InvalidLimit {
            limit,
            min: MIN_LIMIT,
        })
    } else {
        Ok(())
    }
}

pub fn validate_count(count: u32, limit: u32) -> Result<(), Error> {
    if count > limit {
        Err(Error::InvalidCount { count, limit })
    } else {
        Ok(())
    }
}
