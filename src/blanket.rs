use crate::CacheType;

/// Returns the total size in bytes of `level` cache with type `cache_type`.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn cache_size(_level: u8, _cache_type: CacheType) -> Option<usize> {
    None
}

/// Returns the line size in bytes of `level` cache with type `cache_type`.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn cache_line_size(_level: u8, _cache_type: CacheType) -> Option<usize> {
    None
}

/// Returns the total size in bytes of of the L1 data cache.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn l1_cache_size() -> Option<usize> {
    None
}

/// Returns the line size in bytes of the L1 data cache.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn l1_cache_line_size() -> Option<usize> {
    cache_line_size(1, CacheType::Data)
}

/// Returns the total size in bytes of the unified L2 cache.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn l2_cache_size() -> Option<usize> {
    cache_size(2, CacheType::Unified)
}

/// Returns the line size in bytes of the unified L2 cache.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn l2_cache_line_size() -> Option<usize> {
    cache_line_size(2, CacheType::Unified)
}

/// Returns the total size in bytes of the unified L3 cache.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn l3_cache_size() -> Option<usize> {
    cache_size(3, CacheType::Unified)
}

/// Returns the line size in bytes of the unified L3 cache.
///
/// This is the implementation for unsupported architectures, and always returns None.
#[inline]
pub fn l3_cache_line_size() -> Option<usize> {
    cache_line_size(3, CacheType::Unified)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_l1_cache_size() {
        assert_eq!(l1_cache_size(), None);
    }
    #[test]
    fn test_l1_cache_line_size() {
        assert_eq!(l1_cache_line_size(), None)
    }
    #[test]
    fn test_l2_cache_size() {
        assert_eq!(l2_cache_size(), None);
    }
    #[test]
    fn test_l2_cache_line_size() {
        assert_eq!(l2_cache_line_size(), None);
    }
    #[test]
    fn test_l3_cache_size() {
        assert_eq!(l3_cache_size(), None);
    }
    #[test]
    fn test_l3_cache_line_size() {
        assert_eq!(l3_cache_line_size(), None);
    }
}
