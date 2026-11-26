// SPDX-License-Identifier: GPL-2.0-only
use super::super::{NonFiniteCoordinate, binary64_parts};
use super::certificate_arithmetic::{
    ExactRatio,
    rounding::{nearest_binary64, outward_binary64},
};
use num_bigint::BigInt;
use std::{cmp::Ordering, fmt};

/// An exact number `significand * 2^exponent`; integer storage stays private.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DyadicBound {
    significand: BigInt,
    exponent: i32,
}
impl DyadicBound {
    /// Convert a finite binary64 exactly, with either signed zero represented by zero.
    /// Example: `DyadicBound::from_f64(0.5)?`.
    pub fn from_f64(value: f64) -> Result<Self, NonFiniteCoordinate> {
        let parts = binary64_parts(value, 0, 0)?;
        if parts.significand == 0 {
            return Ok(Self {
                significand: 0.into(),
                exponent: 0,
            });
        }
        let zeros = parts.significand.trailing_zeros();
        let magnitude = (parts.significand >> zeros) as i64;
        let signed = if parts.negative {
            -magnitude
        } else {
            magnitude
        };
        // Strip powers of two in the machine word before constructing the BigInt.
        Ok(Self {
            significand: signed.into(),
            exponent: parts.shift as i32 + zeros as i32 - 1074,
        })
    }
    /// Compare a root bound to an exact binary64 value. Example: `bound.compare_f64(0.)?`.
    pub fn compare_f64(&self, value: f64) -> Result<Ordering, NonFiniteCoordinate> {
        Ok(self.cmp(&Self::from_f64(value)?))
    }
    /// Project outward to finite binary64 endpoints, returning `None` on overflow.
    /// Example: `assert_eq!(DyadicBound::from_f64(0.5)?.binary64_bounds(), Some([0.5; 2]));`.
    pub fn binary64_bounds(&self) -> Option<[f64; 2]> {
        outward_binary64(&ExactRatio::from_dyadic(self))
    }
    /// Round this exact dyadic to nearest binary64 with ties to even.
    /// Use `bound.nearest_binary64()` when a construction needs one nominal coordinate.
    pub fn nearest_binary64(&self) -> Option<f64> {
        nearest_binary64(&ExactRatio::from_dyadic(self))
    }

    /// Add dyadics without intermediate binary64 rounding.
    /// Use `left.sum(&right)` to establish ordered authored stations.
    pub fn sum(&self, other: &Self) -> Self {
        let (left, right, exponent) = self.aligned(other);
        Self::new(left + right, exponent)
    }

    /// Multiply two finite literals and optionally scale by a negative power of two.
    /// Use `DyadicBound::product_of_f64(pitch, width, 1)?` to compute exact half rise.
    pub fn product_of_f64(
        left: f64,
        right: f64,
        halvings: u8,
    ) -> Result<Self, NonFiniteCoordinate> {
        let left = Self::from_f64(left)?;
        let right = Self::from_f64(right)?;
        Ok(Self::new(
            left.significand * right.significand,
            left.exponent + right.exponent - i32::from(halvings),
        ))
    }

    pub(super) fn new(significand: BigInt, exponent: i32) -> Self {
        // Binary64 bounds plus at most u16::MAX subdivisions fit i32 exponents
        // and 32-bit shift counts, including a difference across opposite signs.
        let Some(zeros) = significand.trailing_zeros() else {
            return Self {
                significand,
                exponent: 0,
            };
        };
        Self {
            significand: significand >> zeros,
            exponent: exponent + zeros as i32,
        }
    }
    pub(super) fn scale_pow2(&self, power: u8) -> Self {
        Self::new(self.significand.clone(), self.exponent + i32::from(power))
    }
    /// Compute the exact midpoint before any binary64 rounding.
    /// Use `start.midpoint(&end)` for a shared Bernstein control.
    pub fn midpoint(&self, other: &Self) -> Self {
        let (left, right, exponent) = self.aligned(other);
        Self::new(left + right, exponent - 1)
    }
    pub(super) fn sum(&self, other: &Self) -> Self {
        let (left, right, exponent) = self.aligned(other);
        Self::new(left + right, exponent)
    }
    pub(super) fn product(&self, other: &Self) -> Self {
        Self::new(
            &self.significand * &other.significand,
            self.exponent + other.exponent,
        )
    }
    pub(super) fn absolute(&self) -> Self {
        Self {
            significand: self.significand.magnitude().clone().into(),
            exponent: self.exponent,
        }
    }
    /// Subtract dyadics without intermediate binary64 rounding.
    /// Use `total.difference(&reserved)` to check the exact remaining width.
    pub fn difference(&self, other: &Self) -> Self {
        let (left, right, exponent) = self.aligned(other);
        Self::new(left - right, exponent)
    }
    fn aligned(&self, other: &Self) -> (BigInt, BigInt, i32) {
        let exponent = self.exponent.min(other.exponent);
        let left = &self.significand << (self.exponent - exponent) as usize;
        let right = &other.significand << (other.exponent - exponent) as usize;
        (left, right, exponent)
    }
    pub(super) fn ratio(&self) -> (BigInt, BigInt) {
        if self.exponent >= 0 {
            return (&self.significand << self.exponent as usize, BigInt::from(1));
        }
        (
            self.significand.clone(),
            BigInt::from(1) << (-self.exponent) as usize,
        )
    }
}
impl Ord for DyadicBound {
    fn cmp(&self, other: &Self) -> Ordering {
        let (left, right, _) = self.aligned(other);
        left.cmp(&right)
    }
}
impl PartialOrd for DyadicBound {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for DyadicBound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} * 2^{}", self.significand, self.exponent)
    }
}

#[cfg(test)]
#[path = "dyadic_tests.rs"]
mod tests;
