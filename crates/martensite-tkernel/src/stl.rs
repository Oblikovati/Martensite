//! Exact reimplementations of the libstdc++ 13 algorithms whose unstable element order is
//! observable in kernel results on Linux/GCC.
//!
//! `std::sort` and `std::partition` do not preserve the order of equivalent
//! elements. Callers such as BVH sorters apply the
//! resulting permutation in place, so the order of ties is part of the observable
//! output. Rust's own sorts use other algorithms and resolve ties differently.
//!
//! The comparator must be a strict weak ordering. Out-of-contract
//! comparators, which have no defined result, panic on a bounds check here.
//!
//! The `<random>` engine and distribution the kernel draws from are here too: `std::mt19937`
//! (fixed by the standard) and libstdc++'s `std::uniform_real_distribution<double>`
//! (its `generate_canonical` is library-specific).
mod heap;
mod mt19937;
mod partition;
mod sort;
mod uniform_real;
pub use mt19937::MersenneTwister;
pub use partition::partition;
pub use sort::{sort_by, try_sort_by};
pub use uniform_real::{UniformRealDistribution, generate_canonical};
