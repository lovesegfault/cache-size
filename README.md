# cache-size
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause) [![Crates.io](https://img.shields.io/crates/v/cache-size.svg)](https://crates.io/crates/cache-size) [![Documentation](https://docs.rs/cache-size/badge.svg)](https://docs.rs/cache-size) ![Continuous Integration](https://github.com/lovesegfault/cache-size/workflows/Continuous%20Integration/badge.svg)

A library to quickly get the size and line size of your CPU caches.

Currently this crate only supports x86 CPUs, since it relies on the `CPUID` instruction, via
the [`raw_cpuid`][raw_cpuid] crate. It is a goal to support other architectures; PRs are
welcome!

Note that the library will still compile and work on non x86 architectures, but
the result of all the cache queries will be `None`.

---
Check the [Intel 64 and IA-32 Architectures Software Developers Manual](https://software.intel.com/sites/default/files/managed/39/c5/325462-sdm-vol-1-2abcd-3abcd.pdf)
for more information on the `CPUID` instruction.

[raw_cpuid]: https://github.com/gz/rust-cpuid
