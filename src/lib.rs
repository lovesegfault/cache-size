//! A library to quickly get the size and line size of your CPU caches.
//!
//! Currently this crate only supports x86 CPUs, since it relies on the `CPUID` instruction, via
//! the [`raw_cpuid`](raw_cpuid) crate. It is a goal to support other architectures; PRs are
//! welcome!
//!
//! Note that the library will still compile and work on non x86 architectures, but the result of
//! all the cache queries will be `None`.
//!
//! Check the [Intel 64 and IA-32 Architectures Software Developers Manual](https://software.intel.com/sites/default/files/managed/39/c5/325462-sdm-vol-1-2abcd-3abcd.pdf)
//! for more information on the `CPUID` instruction.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::*;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
mod blanket;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub use blanket::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CacheType {
    /// Null - No more caches
    Null = 0,
    /// Data cache
    Data,
    /// Instruction cache
    Instruction,
    /// Data and Instruction cache
    Unified,
    Reserved,
}
