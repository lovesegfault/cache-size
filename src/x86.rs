use crate::CacheType;
use raw_cpuid::{self, CpuId};

impl From<CacheType> for raw_cpuid::CacheType {
    fn from(ct: CacheType) -> Self {
        match ct {
            CacheType::Null => raw_cpuid::CacheType::Null,
            CacheType::Data => raw_cpuid::CacheType::Data,
            CacheType::Instruction => raw_cpuid::CacheType::Instruction,
            CacheType::Unified => raw_cpuid::CacheType::Unified,
            CacheType::Reserved => raw_cpuid::CacheType::Reserved,
        }
    }
}

/// Returns the total size in bytes of `level` cache with type `cache_type`.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the selected cache level and/or type does not exist.
///
/// This is computed as `associativity * line_size * sets`, and if there are multiple caches
/// available, it returns the size of the **smallest** cache.
#[inline]
pub fn cache_size(level: u8, cache_type: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    let caches = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == level && c.cache_type() == cache_type.into())
        .map(|c| c.sets() * c.associativity() * c.coherency_line_size());
    let cache_size = caches.min()?;
    Some(cache_size)
}

/// Returns the line size in bytes of `level` cache with type `cache_type`.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the selected cache level and/or type does not exist.
///
/// This is computed from [`coherency_line_size()`](raw_cpuid::CacheParameter::coherency_line_size),
/// and if there are multiple caches available, it returns the size of the **smallest** cache.
#[inline]
pub fn cache_line_size(level: u8, cache_type: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    let caches = cpuid
        .get_cache_parameters()?
        .filter(|cparams| cparams.level() == level && cparams.cache_type() == cache_type.into())
        .map(|cparams| cparams.coherency_line_size());
    let cache_line_size = caches.min()?;
    Some(cache_line_size)
}

/// Returns the total size in bytes of the L1 data cache.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the system has no L1 data cache (if it's a unified L1 cache for example).
///
/// This is computed as `associativity * line_size * sets`, and if there are multiple caches
/// available, it returns the size of the **smallest** cache.
#[inline]
pub fn l1_cache_size() -> Option<usize> {
    cache_size(1, CacheType::Data)
}

/// Returns the line size in bytes of the L1 data cache.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the system has no L1 data cache (if it's a unified L1 cache for example).
///
/// This is computed from [`coherency_line_size()`](raw_cpuid::CacheParameter::coherency_line_size),
/// and if there are multiple caches available, it returns the size of the **smallest** cache.
#[inline]
pub fn l1_cache_line_size() -> Option<usize> {
    cache_line_size(1, CacheType::Data)
}

/// Returns the total size in bytes of the unified L2 cache.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the system has no unified L2 cache.
///
/// This is computed as `associativity * line_size * sets`, and if there are multiple caches
/// available, it returns the size of the **smallest** cache.
#[inline]
pub fn l2_cache_size() -> Option<usize> {
    cache_size(2, CacheType::Unified)
}

/// Returns the line size in bytes of the unified L2 cache.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the system has no unified L2 cache.
///
/// This is computed from [`coherency_line_size()`](raw_cpuid::CacheParameter::coherency_line_size),
/// and if there are multiple caches available, it returns the size of the **smallest** cache.
#[inline]
pub fn l2_cache_line_size() -> Option<usize> {
    cache_line_size(2, CacheType::Unified)
}

/// Returns the total size in bytes of the unified L3 cache.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the system has no unified L3 cache.
///
/// This is computed as `associativity * line_size * sets`, and if there are multiple caches
/// available, it returns the size of the **smallest** cache.
#[inline]
pub fn l3_cache_size() -> Option<usize> {
    cache_size(3, CacheType::Unified)
}

/// Returns the line size in bytes of the unified L3 cache.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the system has no unified L3 cache.
///
/// This is computed from [`coherency_line_size()`](raw_cpuid::CacheParameter::coherency_line_size),
/// and if there are multiple caches available, it returns the size of the **smallest** cache.
#[inline]
pub fn l3_cache_line_size() -> Option<usize> {
    cache_line_size(3, CacheType::Unified)
}

/// Tests
///
/// I'd like to figure out a way to test this crate, but it's behavior being entirely
/// hardware-dependant that seems hard, if not impossible.
#[cfg(test)]
mod tests {}
