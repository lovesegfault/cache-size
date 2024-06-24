use raw_cpuid::{self, CpuId, CpuIdReaderNative};

pub use raw_cpuid::CacheType;

/// Uses the CPUID family info to detect Zen architecture CPUs.
///
/// Data pulled from https://en.wikichip.org/wiki/amd/cpuid.
#[inline]
fn amd_is_zen(cpuid: &CpuId<CpuIdReaderNative>) -> Option<bool> {
    let info = cpuid.get_feature_info()?;
    match (info.base_family_id(), info.extended_family_id()) {
        (0xF, 0x8..=0xA) => Some(true),
        _ => Some(false),
    }
}

/// Uses cache parameters to get cache size at a given level with the provided cache type.
#[inline]
fn generic_cache_size(
    cpuid: CpuId<CpuIdReaderNative>,
    level: u8,
    cache_type: CacheType,
) -> Option<usize> {
    cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == level && c.cache_type() == cache_type)
        .map(|c| c.sets() * c.associativity() * c.coherency_line_size())
        .min()
}

/// This is computed using tlb info. The values come back in kilobytes, so they are multiplied by
/// 1024 to give the size in bytes to match the behaviour of other architectures.
#[inline]
fn amd_cache_size(
    cpuid: CpuId<CpuIdReaderNative>,
    level: u8,
    cache_type: CacheType,
) -> Option<usize> {
    match (level, cache_type) {
        (1, CacheType::Instruction) => cpuid
            .get_l1_cache_and_tlb_info()
            .map(|i| i.icache_size() as usize * 1024),
        (1, CacheType::Data) => cpuid
            .get_l1_cache_and_tlb_info()
            .map(|i| i.icache_size() as usize * 1024),
        (2, CacheType::Unified) => cpuid
            .get_l2_l3_cache_and_tlb_info()
            .map(|i| i.l2cache_size() as usize * 1024),
        (3, CacheType::Unified) => cpuid
            .get_l2_l3_cache_and_tlb_info()
            .map(|i| i.l3cache_size() as usize * 1024),
        _ => None,
    }
}

/// Returns the total size in bytes of `level` cache with type `cache_type`.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the selected cache level and/or type does not exist.
///
/// On an AMD Zen architecture this is computed using tlb info. The values come back in kilobytes,
/// so they are multiplied by 1024 to give the size in bytes to match the behaviour of other
/// architectures.
///
/// On other architectures this is computed as `associativity * line_size * sets`, and if there are multiple caches
/// available, it returns the size of the **smallest** cache.
#[inline]
pub fn cache_size(level: u8, cache_type: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info()?.as_str() {
        "AuthenticAMD" if amd_is_zen(&cpuid).unwrap_or(false) => {
            amd_cache_size(cpuid, level, cache_type)
        }
        _ => generic_cache_size(cpuid, level, cache_type),
    }
}

/// Uses cache parameters to get cache line size at a given level with the provided cache type.
#[inline]
fn generic_cache_line_size(
    cpuid: CpuId<CpuIdReaderNative>,
    level: u8,
    cache_type: CacheType,
) -> Option<usize> {
    cpuid
        .get_cache_parameters()?
        .filter(|cparams| cparams.level() == level && cparams.cache_type() == cache_type)
        .map(|cparams| cparams.coherency_line_size())
        .min()
}

/// This is computed using tlb info. Instruction and data cache line sizes
/// are available separately for the L1 cache, but only unified is available for L2 and L3 caches.
#[inline]
fn amd_cache_line_size(
    cpuid: CpuId<CpuIdReaderNative>,
    level: u8,
    cache_type: CacheType,
) -> Option<usize> {
    match (level, cache_type) {
        (1, CacheType::Instruction) => cpuid
            .get_l1_cache_and_tlb_info()
            .map(|i| i.icache_line_size() as usize),
        (1, CacheType::Data) => cpuid
            .get_l1_cache_and_tlb_info()
            .map(|i| i.dcache_line_size() as usize),
        (2, CacheType::Unified) => cpuid
            .get_l2_l3_cache_and_tlb_info()
            .map(|i| i.l2cache_line_size() as usize),
        (3, CacheType::Unified) => cpuid
            .get_l2_l3_cache_and_tlb_info()
            .map(|i| i.l3cache_line_size() as usize),
        _ => None,
    }
}

/// Returns the line size in bytes of `level` cache with type `cache_type`.
///
/// The only possibilities for this returning `None` are if the system does not support cache
/// parameters, in which case [`get_cache_parameters()`](raw_cpuid::CpuId::get_cache_parameters) will
/// fail, or if the selected cache level and/or type does not exist.
///
/// On an AMD Zen architecture this is computed using tlb info. Instruction and data cache line
/// sizes are available separately for the L1 cache, but only unified is available for L2 and L3
/// caches.
///
/// On other x86 architectures this is computed from
/// [`coherency_line_size()`](raw_cpuid::CacheParameter::coherency_line_size),
/// and if there are multiple caches available, it returns the size of the **smallest** cache.
#[inline]
pub fn cache_line_size(level: u8, cache_type: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info()?.as_str() {
        "AuthenticAMD" if amd_is_zen(&cpuid).unwrap_or(false) => {
            amd_cache_line_size(cpuid, level, cache_type)
        }
        _ => generic_cache_line_size(cpuid, level, cache_type),
    }
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
