#[inline]
pub fn cache_size(level: u8, cache_type: CacheType) -> Option<usize> {
    None
}

#[inline]
pub fn cache_line_size(level: u8, cache_type: CacheType) -> Option<usize> {
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

    test_blanket_l3_cache_line_size() {
        assert_eq!(l3_cache_line_size(), None);
    }

}
