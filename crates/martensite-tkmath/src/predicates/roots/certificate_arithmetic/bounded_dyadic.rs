// SPDX-License-Identifier: GPL-2.0-only
//! Outward fixed-grid dyadic intervals; polynomial arithmetic never normalizes ratios.
use super::bounded_failure::{BoundedArithmeticFailure as Failure, Checked};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{ExactRatio, rounding::outward_binary64},
};
use num_bigint::{BigInt, Sign};

const MAX_INTEGER_BITS: u64 = 16_384;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::predicates::roots) struct ArithmeticPolicy {
    pub fractional_bits: u16,
    pub max_operations: u32,
}
impl Default for ArithmeticPolicy {
    fn default() -> Self {
        Self {
            fractional_bits: 128,
            max_operations: 100_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::predicates::roots) struct Interval {
    pub lower: BigInt,
    pub upper: BigInt,
}
impl Interval {
    pub fn magnitude(&self) -> BigInt {
        absolute(&self.lower).max(absolute(&self.upper))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::predicates::roots) enum PositiveSqrtFailure {
    Arithmetic(Failure),
    InvalidInterval {
        lower: DyadicBound,
        upper: DyadicBound,
    },
}
impl From<Failure> for PositiveSqrtFailure {
    fn from(cause: Failure) -> Self {
        Self::Arithmetic(cause)
    }
}

pub(in crate::predicates::roots) struct Arithmetic<'a> {
    options: ArithmeticPolicy,
    unit: BigInt,
    operations: u32,
    cancelled: &'a mut (dyn FnMut() -> bool + Send),
    binomials: Box<[[u64; 51]; 51]>,
}
impl<'a> Arithmetic<'a> {
    pub fn new(options: ArithmeticPolicy, cancelled: &'a mut (dyn FnMut() -> bool + Send)) -> Self {
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
    pub fn policy(&self) -> ArithmeticPolicy {
        self.options
    }
    pub fn fractional_bits(&self) -> u16 {
        self.options.fractional_bits
    }
    pub fn quantum(&self) -> Interval {
        Interval {
            lower: 1.into(),
            upper: 1.into(),
        }
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
        let integer = crate::predicates::exact_coordinate(value, 0, 0)
            .map_err(|_| Failure::NonFiniteScalar { value })?;
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
    pub fn positive_sqrt(&mut self, value: &Interval) -> Result<Interval, PositiveSqrtFailure> {
        self.charge()?;
        self.positive_sqrt_domain(value)?;
        let lower = &value.lower * &self.unit;
        let upper = &value.upper * &self.unit;
        check_bits(lower.bits().max(upper.bits()))?;
        let lower_root = lower.sqrt();
        let mut upper_root = upper.sqrt();
        if &upper_root * &upper_root < upper {
            upper_root += 1;
        }
        Ok(checked(lower_root, upper_root)?)
    }
    fn positive_sqrt_domain(&self, value: &Interval) -> Result<(), PositiveSqrtFailure> {
        if value.lower > value.upper {
            let exponent = -i32::from(self.options.fractional_bits);
            return Err(PositiveSqrtFailure::InvalidInterval {
                lower: DyadicBound::new(value.lower.clone(), exponent),
                upper: DyadicBound::new(value.upper.clone(), exponent),
            });
        }
        self.positive_denominator(&value.lower).map_err(Into::into)
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
    pub fn divide(&mut self, a: &Interval, b: &Interval) -> Checked<Interval> {
        self.charge()?;
        self.positive_denominator(&b.lower)?;
        let pairs = [
            (&a.lower, &b.lower),
            (&a.lower, &b.upper),
            (&a.upper, &b.lower),
            (&a.upper, &b.upper),
        ];
        let bounds = pairs.map(|(a, b)| self.rounded(&(a * &self.unit), &(a * &self.unit), b));
        let [a, b, c, d] = bounds;
        let bounds = [a?, b?, c?, d?];
        let lower = bounds
            .iter()
            .fold(bounds[0].lower.clone(), |a, b| a.min(b.lower.clone()));
        let upper = bounds
            .iter()
            .fold(bounds[0].upper.clone(), |a, b| a.max(b.upper.clone()));
        checked(lower, upper)
    }
    pub fn integer_scale(&mut self, a: &Interval, factor: &BigInt) -> Checked<Interval> {
        self.charge()?;
        let (lower, upper) = if factor.sign() == Sign::Minus {
            (&a.upper, &a.lower)
        } else {
            (&a.lower, &a.upper)
        };
        checked(lower * factor, upper * factor)
    }
    pub fn integer_quotient(
        &mut self,
        numerator: &BigInt,
        denominator: &BigInt,
    ) -> Checked<BigInt> {
        self.charge()?;
        self.positive_denominator(denominator)?;
        Ok(numerator / denominator)
    }
    pub fn upper_quotient(&mut self, numerator: &BigInt, denominator: &BigInt) -> Checked<f64> {
        self.charge()?;
        self.positive_denominator(denominator)?;
        let scaled = numerator * &self.unit;
        check_bits(scaled.bits())?;
        self.upper_f64(divide_ceil(&scaled, denominator))
    }
    fn positive_denominator(&self, lower: &BigInt) -> Checked<()> {
        if lower.sign() != Sign::Plus {
            return Err(Failure::DenominatorUnresolved {
                lower: DyadicBound::new(lower.clone(), -i32::from(self.options.fractional_bits)),
            });
        }
        Ok(())
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
    /// Charge one finite certificate step in the same remaining work ledger.
    pub fn certificate_step(&mut self) -> Checked<()> {
        self.charge()
    }
    pub fn check_cancelled(&mut self) -> Checked<()> {
        if (self.cancelled)() {
            return Err(Failure::Cancelled);
        }
        Ok(())
    }
    fn charge(&mut self) -> Checked<()> {
        self.check_cancelled()?;
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
#[path = "bounded_dyadic_tests.rs"]
mod tests;
