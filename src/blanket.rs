use crate::CacheType;

#[deprecated(
    note = "You are using a blanket implementation of cache-size because your architecture is not supported."
)]
#[inline]
fn cache_size(_level: u8, _cache_type: CacheType) -> Option<usize> {
    None
}

#[deprecated(
    note = "You are using a blanket implementation of cache-size because your architecture is not supported."
)]
#[inline]
fn cache_line_size(_level: u8, _cache_type: CacheType) -> Option<usize> {
    None
}

#[inline]
pub fn l1_cache_size() -> Option<usize> {
    None
}

#[inline]
pub fn l1_cache_line_size() -> Option<usize> {
    cache_line_size(1, CacheType::Data)
}

#[inline]
pub fn l2_cache_size() -> Option<usize> {
    cache_size(2, CacheType::Unified)
}

#[inline]
pub fn l2_cache_line_size() -> Option<usize> {
    cache_line_size(2, CacheType::Unified)
}

#[inline]
pub fn l3_cache_size() -> Option<usize> {
    cache_size(3, CacheType::Unified)
}

#[inline]
pub fn l3_cache_line_size() -> Option<usize> {
    cache_line_size(3, CacheType::Unified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blanket_l3_cache_line_size() {
        assert_eq!(l3_cache_line_size(), None);
    }
}
