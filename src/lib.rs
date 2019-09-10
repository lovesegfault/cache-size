//! A library to quickly get the size and line size of your CPU caches.
//!
//! Currently this crate only supports x86 CPUs, since it relies on the `CPUID` instruction, via
//! the [`raw_cpuid`](raw_cpuid) crate. It is a goal to support other architectures; PRs are
//! welcome!
//!
//! Check the [Intel 64 and IA-32 Architectures Software Developers Manual](https://software.intel.com/sites/default/files/managed/39/c5/325462-sdm-vol-1-2abcd-3abcd.pdf)
//! for more information on the `CPUID` instruction.
pub use raw_cpuid::CacheType;
use raw_cpuid::{self, CpuId};

/// Returns the total size in bytes of `level` cache with type `ctype`.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the selected cache level and/or type does not exist.
///
/// This is computed as `associativity * line_size * sets`, and if there are multiple caches
/// available, it returns the size of the **smallest** cache.
#[inline]
pub fn cache_size(level: u8, ctype: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    let caches = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == level && c.cache_type() == ctype)
        .map(|c| c.sets() * c.associativity() * c.coherency_line_size());
    let cache_size = caches.min()?;
    Some(cache_size)
}

/// Returns the line size in bytes of `level` cache with type `ctype`.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the selected cache level and/or type does not exist.
///
/// This is computed from [`coherency_line_size()`](raw_cpuid::CacheParameter::coherency_line_size),
/// and if there are multiple caches available, it returns the size of the **smallest** cache.
#[inline]
pub fn cache_line_size(level: u8, ctype: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    let caches = cpuid
        .get_cache_parameters()?
        .filter(|cparams| cparams.level() == level && cparams.cache_type() == ctype)
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
