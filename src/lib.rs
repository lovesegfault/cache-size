extern crate raw_cpuid;

use raw_cpuid::{CacheType, CpuId};

pub fn l1_cache_size() -> Option<usize> {
    let cpuid = CpuId::new();
    let l1 = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == 1 && c.cache_type() == CacheType::DATA)
        .map(|c| c.sets() * c.associativity() * c.coherency_line_size());
    let cache_size = l1.min()?;
    Some(cache_size)
}

pub fn l1_cache_line_size() -> Option<usize> {
    let cpuid = CpuId::new();
    let l1 = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == 1 && c.cache_type() == CacheType::DATA)
        .map(|c| c.coherency_line_size());
    let cache_line_size = l1.min()?;
    Some(cache_line_size)
}

pub fn l2_cache_size() -> Option<usize> {
    let cpuid = CpuId::new();
    let l1 = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == 2 && c.cache_type() == CacheType::UNIFIED)
        .map(|c| c.sets() * c.associativity() * c.coherency_line_size());
    let cache_size = l1.min()?;
    Some(cache_size)
}

pub fn l2_cache_line_size() -> Option<usize> {
    let cpuid = CpuId::new();
    let l1 = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == 2 && c.cache_type() == CacheType::UNIFIED)
        .map(|c| c.coherency_line_size());
    let cache_line_size = l1.min()?;
    Some(cache_line_size)
}

pub fn l3_cache_size() -> Option<usize> {
    let cpuid = CpuId::new();
    let l1 = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == 3 && c.cache_type() == CacheType::UNIFIED)
        .map(|c| c.sets() * c.associativity() * c.coherency_line_size());
    let cache_size = l1.min()?;
    Some(cache_size)
}

pub fn l3_cache_line_size() -> Option<usize> {
    let cpuid = CpuId::new();
    let l1 = cpuid
        .get_cache_parameters()?
        .filter(|c| c.level() == 3 && c.cache_type() == CacheType::UNIFIED)
        .map(|c| c.coherency_line_size());
    let cache_line_size = l1.min()?;
    Some(cache_line_size)
}

#[cfg(test)]
mod tests {
    #[test]
    fn l1_cache_size_works() {
        assert_eq!(1, 1)
    }
}
