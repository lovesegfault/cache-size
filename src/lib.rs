//! A library to quickly get the size and line size of your CPU caches.
//!
//! Currently this crate only supports x86 CPUs, since it relies on the `CPUID` instruction, via
//! the [`raw_cpuid`](raw_cpuid) crate. It is a goal to support other architectures; PRs are
//! welcome!
//!
//! Check the [Intel 64 and IA-32 Architectures Software Developers Manual](https://software.intel.com/sites/default/files/managed/39/c5/325462-sdm-vol-1-2abcd-3abcd.pdf)
//! for more information on the `CPUID` instruction.

#[cfg_attr(target_arch = "x86", path = "x86.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86.rs")]
#[cfg_attr(
    not(any(target_arch = "x86_64", target_arch = "x86")),
    path = "blanket.rs"
)]
mod imp;
pub use imp::*;
