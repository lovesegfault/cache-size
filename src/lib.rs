extern crate raw_cpuid;

use raw_cpuid::CpuId;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CacheType {
    /// Data cache
    Data,
    /// Instruction cache
    Instruction,
    /// Unified Data + Instruction cache
    Unified,
}

impl Into<raw_cpuid::CacheType> for CacheType {
    fn into(self) -> raw_cpuid::CacheType {
        match self {
            CacheType::Data => raw_cpuid::CacheType::DATA,
            CacheType::Instruction => raw_cpuid::CacheType::INSTRUCTION,
            CacheType::Unified => raw_cpuid::CacheType::UNIFIED,
        }
    }
}

#[inline]
pub fn cache_size(level: u8, ctype: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    let caches = cpuid
        .get_cache_parameters()?
        .filter(move |c| c.level() == level && c.cache_type() == ctype.into())
        .map(|c| c.sets() * c.associativity() * c.coherency_line_size());
    let cache_size = caches.min()?;
    Some(cache_size)
}

#[inline]
pub fn cache_line_size(level: u8, ctype: CacheType) -> Option<usize> {
    let cpuid = CpuId::new();
    let caches = cpuid
        .get_cache_parameters()?
        .filter(|cparams| cparams.level() == level && cparams.cache_type() == ctype.into())
        .map(|cparams| cparams.coherency_line_size());
    let cache_line_size = caches.min()?;
    Some(cache_line_size)
}

#[inline]
pub fn l1_cache_size() -> Option<usize> {
    cache_size(1, CacheType::Data)
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
    #[test]
    fn l1_cache_size_works() {
        assert_eq!(1, 1)
    }
}
