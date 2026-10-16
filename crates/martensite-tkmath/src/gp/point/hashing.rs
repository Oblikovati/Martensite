//! The `std::hash<Point3>` and `std::equal_to<Point3>` specializations.
//!
//! The hash reads the three doubles through a union as six 32-bit ints (little-endian
//! words, low half first), sums their truncating quotients by 23, 19, 17, 13, 11 and 7 in
//! `int`, and hashes that sum converted to double with libstdc++ `std::hash<double>` (the
//! linux-gcc profile). The sum cannot overflow: its magnitude is below
//! `2^31 * (1/23 + 1/19 + 1/17 + 1/13 + 1/11 + 1/7) < 2^31`.
use super::super::Point3;
use martensite_tkernel::ncollection::{HashKey, HashPolicy};
use martensite_tkernel::standard::real::epsilon;
use std::cmp::Ordering;
const DIVISORS: [i32; 6] = [23, 19, 17, 13, 11, 7];
impl HashKey for Point3 {
    /// `std::hash<Point3>{}(point)`; `point.kernel_hash()`.
    fn kernel_hash(&self) -> usize {
        let words = self.coordinates.iter().flat_map(|value| {
            let bits = value.to_bits();
            [bits as u32 as i32, (bits >> 32) as u32 as i32]
        });
        let sum: i32 = words
            .zip(DIVISORS)
            .map(|(word, divisor)| word / divisor)
            .sum();
        f64::from(sum).kernel_hash()
    }
}
/// Hash/equality policy pairing `std::hash<Point3>` with `std::equal_to<Point3>`, whose
/// equality accepts coordinate differences up to one ulp of the second point;
/// `PointHashPolicy.equal(&first,&second)`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PointHashPolicy;
impl HashPolicy<Point3> for PointHashPolicy {
    fn hash(&self, key: &Point3) -> usize {
        key.kernel_hash()
    }
    fn equal(&self, first: &Point3, second: &Point3) -> bool {
        // `a > b` is false for NaN, so a NaN difference compares equal.
        (0..3).all(|index| {
            let (left, right) = (first.coordinates[index], second.coordinates[index]);
            (left - right).abs().partial_cmp(&epsilon(right)) != Some(Ordering::Greater)
        })
    }
}
