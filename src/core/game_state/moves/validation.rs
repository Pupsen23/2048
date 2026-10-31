use super::rules::MIN_LIMIT;

pub fn is_valid_limit(limit: u32) -> bool {
    limit >= MIN_LIMIT
}

pub fn is_valid_count(count: u32, limit: u32) -> bool {
    count <= limit
}
