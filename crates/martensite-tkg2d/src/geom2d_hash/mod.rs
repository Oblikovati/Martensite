//! Quantized hashes for planar curve geometry.
//!
//! Hashes combine native-endian words on a 64-bit profile. They are lookup keys,
//! not persistent entity identities or a cross-architecture serialization format.
/// Declares a hasher struct with its two public tolerances, `new` and the defined
/// default arguments (`precision::angular()`, `precision::confusion()` unless two
/// default expressions follow the name).
///
/// Exported so other geometry hashers can share the tolerance contract;
/// `hasher_tolerances!(/// doc
/// MyHasher, martensite_tkernel::precision::computational(), martensite_tkernel::precision::computational());`.
#[macro_export]
macro_rules! hasher_tolerances {
    ($(#[$doc:meta])* $name:ident) => {
        $crate::hasher_tolerances!(
            $(#[$doc])* $name,
            ::martensite_tkernel::precision::angular(),
            ::martensite_tkernel::precision::confusion()
        );
    };
    ($(#[$doc:meta])* $name:ident, $comp_default:expr, $hash_default:expr) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name {
            /// Tolerance of the equality comparison (`CompTolerance`).
            pub comp_tolerance: f64,
            /// Rounding step of hashed values (`HashTolerance`).
            pub hash_tolerance: f64,
        }
        impl $name {
            /// The hasher with explicit tolerances (the defined constructor);
            #[doc = concat!("`", stringify!($name), "::new(1e-12, 1e-7)`.")]
            pub fn new(comp_tolerance: f64, hash_tolerance: f64) -> Self {
                Self { comp_tolerance, hash_tolerance }
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new($comp_default, $hash_default)
            }
        }
    };
}

pub mod class_dispatch;
pub mod hash_key;

mod placement_hashers;
pub use placement_hashers::{
    Geom2dHashAxisPlacement, Geom2dHashDirectionHasher, Geom2dHashPointHasher,
};
mod conic_hashers;
pub use conic_hashers::{
    Geom2dHashCircleHasher, Geom2dHashEllipseHasher, Geom2dHashHyperbolaHasher,
    Geom2dHashLineHasher, Geom2dHashParabolaHasher,
};
mod spline_hashers;
pub use spline_hashers::{Geom2dHashBSplineCurveHasher, Geom2dHashBezierCurveHasher};
mod wrapped_hashers;
pub use wrapped_hashers::{Geom2dHashOffsetCurveHasher, Geom2dHashTrimmedCurveHasher};
mod curve_hasher;
pub use curve_hasher::Geom2dHashCurveHasher;
