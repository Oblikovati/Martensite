// SPDX-License-Identifier: GPL-2.0-only
//! Outward fixed-grid dyadic intervals; polynomial arithmetic never normalizes ratios.
use super::{CylindricalStorageFailure as Failure, CylindricalStorageOptions, error::Checked};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{ExactRatio, rounding::outward_binary64},
};
use num_bigint::{BigInt, Sign};

const MAX_INTEGER_BITS: u64 = 16_384;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Interval {
    pub lower: BigInt,
    pub upper: BigInt,
}
impl Interval {
    pub fn magnitude(&self) -> BigInt {
        absolute(&self.lower).max(absolute(&self.upper))
    }
}

pub(super) struct Arithmetic<'a> {
    options: CylindricalStorageOptions,
    unit: BigInt,
    operations: u32,
    cancelled: &'a mut (dyn FnMut() -> bool + Send),
    binomials: Box<[[u64; 51]; 51]>,
}
impl<'a> Arithmetic<'a> {
    pub fn new(
        options: CylindricalStorageOptions,
        cancelled: &'a mut (dyn FnMut() -> bool + Send),
    ) -> Self {
        Self {
            unit: BigInt::from(1) << usize::from(options.fractional_bits),
            options,
            operations: 0,
            cancelled,
            binomials: binomial_table(),
        }
    }
    pub fn operations(&self) -> u32 {
        self.operations
    }
    pub fn binomial(&self, n: usize, k: usize) -> u64 {
        self.binomials[n][k]
    }
    pub fn zero(&self) -> Interval {
        Interval {
            lower: 0.into(),
            upper: 0.into(),
        }
    }
    pub fn scalar(&mut self, value: f64) -> Checked<Interval> {
        self.charge()?;
        let integer =
            crate::predicates::exact_coordinate(value, 0, 0).map_err(|_| Failure::Value {
                depth: None,
                field: "dyadic scalar",
                index: 0,
                value,
            })?;
        let numerator = integer << usize::from(self.options.fractional_bits);
        self.rounded(&numerator, &numerator, &(BigInt::from(1) << 1074))
    }
    pub fn add(&mut self, a: &Interval, b: &Interval) -> Checked<Interval> {
        self.charge()?;
        checked(&a.lower + &b.lower, &a.upper + &b.upper)
    }
    pub fn subtract(&mut self, a: &Interval, b: &Interval) -> Checked<Interval> {
        self.charge()?;
        checked(&a.lower - &b.upper, &a.upper - &b.lower)
    }
    pub fn multiply(&mut self, a: &Interval, b: &Interval) -> Checked<Interval> {
        self.charge()?;
        let products = [
            &a.lower * &b.lower,
            &a.lower * &b.upper,
            &a.upper * &b.lower,
            &a.upper * &b.upper,
        ];
        let lower = products
            .iter()
            .fold(products[0].clone(), |a, b| a.min(b.clone()));
        let upper = products
            .iter()
            .fold(products[0].clone(), |a, b| a.max(b.clone()));
        self.rounded(&lower, &upper, &self.unit)
    }
    pub fn fraction(
        &mut self,
        a: &Interval,
        numerator: u64,
        denominator: u64,
    ) -> Checked<Interval> {
        self.charge()?;
        self.rounded(
            &(&a.lower * numerator),
            &(&a.upper * numerator),
            &BigInt::from(denominator),
        )
    }
    pub fn upper_quotient(&mut self, numerator: &BigInt, denominator: &BigInt) -> Checked<f64> {
        self.charge()?;
        if denominator.sign() != Sign::Plus {
            return Err(Failure::DenominatorUnresolved {
                lower: DyadicBound::new(
                    denominator.clone(),
                    -i32::from(self.options.fractional_bits),
                ),
            });
        }
        let scaled = numerator * &self.unit;
        check_bits(scaled.bits())?;
        self.upper_f64(divide_ceil(&scaled, denominator))
    }
    pub fn upper_f64(&self, integer: BigInt) -> Checked<f64> {
        let bound = DyadicBound::new(integer, -i32::from(self.options.fractional_bits));
        outward_binary64(&ExactRatio::from_dyadic(&bound))
            .map(|bounds| bounds[1])
            .ok_or(Failure::UnrepresentableBound { bound })
    }
    fn rounded(&self, lower: &BigInt, upper: &BigInt, denominator: &BigInt) -> Checked<Interval> {
        check_bits(lower.bits().max(upper.bits()))?;
        checked(
            divide_floor(lower, denominator),
            divide_ceil(upper, denominator),
        )
    }
    fn charge(&mut self) -> Checked<()> {
        if (self.cancelled)() {
            return Err(Failure::Cancelled);
        }
        if self.operations == self.options.max_operations {
            return Err(Failure::WorkLimit {
                completed: self.operations,
                limit: self.options.max_operations,
            });
        }
        self.operations += 1;
        Ok(())
    }
}

fn checked(lower: BigInt, upper: BigInt) -> Checked<Interval> {
    check_bits(lower.bits().max(upper.bits()))?;
    Ok(Interval { lower, upper })
}
fn check_bits(actual: u64) -> Checked<()> {
    if actual > MAX_INTEGER_BITS {
        return Err(Failure::ArithmeticBits {
            actual,
            limit: MAX_INTEGER_BITS,
        });
    }
    Ok(())
}
fn absolute(value: &BigInt) -> BigInt {
    if value.sign() == Sign::Minus {
        -value
    } else {
        value.clone()
    }
}
fn divide_floor(numerator: &BigInt, denominator: &BigInt) -> BigInt {
    let quotient = numerator / denominator;
    if numerator.sign() == Sign::Minus && numerator % denominator != BigInt::from(0) {
        return quotient - 1;
    }
    quotient
}
fn divide_ceil(numerator: &BigInt, denominator: &BigInt) -> BigInt {
    -divide_floor(&(-numerator), denominator)
}
fn binomial_table() -> Box<[[u64; 51]; 51]> {
    // Choose(50,25) < 2^47; every admitted product has total degree at most 50.
    let mut table = Box::new([[0; 51]; 51]);
    for n in 0..=50 {
        table[n][0] = 1;
        table[n][n] = 1;
        for k in 1..n {
            table[n][k] = table[n - 1][k - 1] + table[n - 1][k];
        }
    }
    table
}

#[cfg(test)]
#[path = "dyadic_tests.rs"]
mod tests;
