// SPDX-License-Identifier: GPL-2.0-only
//! Exact enclosure arithmetic; rounding is postponed until a proved endpoint.
use super::super::DyadicBound;
use num_bigint::{BigInt, Sign};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::predicates::roots) struct ExactRatio {
    numerator: BigInt,
    denominator: BigInt,
}
impl ExactRatio {
    pub(in crate::predicates::roots) fn new(
        mut numerator: BigInt,
        mut denominator: BigInt,
    ) -> Self {
        assert_ne!(
            denominator,
            BigInt::from(0),
            "denominator 0; expected nonzero"
        );
        if denominator.sign() == Sign::Minus {
            numerator = -numerator;
            denominator = -denominator;
        }
        if numerator.sign() == Sign::NoSign {
            return Self {
                numerator,
                denominator: 1.into(),
            };
        }
        if denominator.bits() != 1 {
            let divisor = positive_gcd(numerator.clone(), denominator.clone());
            if divisor.bits() != 1 {
                numerator /= &divisor;
                denominator /= divisor;
            }
        }
        Self {
            numerator,
            denominator,
        }
    }
    pub(in crate::predicates::roots) fn from_dyadic(bound: &DyadicBound) -> Self {
        let (numerator, denominator) = bound.ratio();
        // Canonical dyadics give either a unit denominator or an odd numerator
        // over a positive power of two. Both forms are already reduced.
        Self {
            numerator,
            denominator,
        }
    }
    pub(in crate::predicates::roots) fn from_finite(value: f64) -> Self {
        let bound = DyadicBound::from_f64(value).expect("finite input already validated");
        Self::from_dyadic(&bound)
    }
    pub(in crate::predicates::roots) fn add(&self, other: &Self) -> Self {
        if self.denominator == other.denominator {
            return Self::new(&self.numerator + &other.numerator, self.denominator.clone());
        }
        let numerator = &self.numerator * &other.denominator + &other.numerator * &self.denominator;
        Self::new(numerator, &self.denominator * &other.denominator)
    }
    pub(in crate::predicates::roots) fn negated(&self) -> Self {
        Self {
            numerator: -&self.numerator,
            denominator: self.denominator.clone(),
        }
    }
    pub(in crate::predicates::roots) fn subtract(&self, other: &Self) -> Self {
        if self.denominator == other.denominator {
            return Self::new(&self.numerator - &other.numerator, self.denominator.clone());
        }
        let numerator = &self.numerator * &other.denominator - &other.numerator * &self.denominator;
        Self::new(numerator, &self.denominator * &other.denominator)
    }
    pub(in crate::predicates::roots) fn multiply(&self, other: &Self) -> Self {
        let numerator = &self.numerator * &other.numerator;
        Self::new(numerator, &self.denominator * &other.denominator)
    }
    pub(in crate::predicates::roots) fn divide(&self, other: &Self) -> Self {
        let numerator = &self.numerator * &other.denominator;
        Self::new(numerator, &self.denominator * &other.numerator)
    }
    pub(in crate::predicates::roots) fn sign(&self) -> Ordering {
        self.numerator.cmp(&BigInt::from(0))
    }
    /// Reduced numerator; the sign of the value is carried here.
    pub(in crate::predicates::roots) fn numerator(&self) -> &BigInt {
        &self.numerator
    }
    /// Reduced, strictly positive denominator.
    pub(in crate::predicates::roots) fn denominator(&self) -> &BigInt {
        &self.denominator
    }
}
impl Ord for ExactRatio {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.denominator == other.denominator {
            return self.numerator.cmp(&other.numerator);
        }
        let left = &self.numerator * &other.denominator;
        let right = &other.numerator * &self.denominator;
        left.cmp(&right)
    }
}
impl PartialOrd for ExactRatio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn positive_gcd(mut left: BigInt, mut right: BigInt) -> BigInt {
    if left.sign() == Sign::Minus {
        left = -left;
    }
    while right.sign() != Sign::NoSign {
        let remainder = left % &right;
        left = right;
        right = remainder;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arithmetic_keeps_rational_value_and_canonical_positive_denominator() {
        let third = ExactRatio::new(1.into(), 3.into());
        let half = ExactRatio::new((-2).into(), (-4).into());
        assert_eq!(third.add(&half), ExactRatio::new(5.into(), 6.into()));
        assert_eq!(half.subtract(&third), ExactRatio::new(1.into(), 6.into()));
        assert_eq!(half.multiply(&third), ExactRatio::new(1.into(), 6.into()));
        assert_eq!(half.divide(&third), ExactRatio::new(3.into(), 2.into()));
        assert_eq!(third.negated().sign(), Ordering::Less);
    }
    #[test]
    fn canonical_dyadics_match_general_ratio_reduction() {
        let low = DyadicBound::new((-8).into(), -66609);
        let high = DyadicBound::new(12.into(), 1022);
        let middle = low.midpoint(&high);
        for bound in [
            DyadicBound::from_f64(-0.).unwrap(),
            low.scale_pow2(32),
            high.difference(&middle),
            middle.difference(&high),
        ] {
            let (numerator, denominator) = bound.ratio();
            assert_eq!(
                ExactRatio::from_dyadic(&bound),
                ExactRatio::new(numerator, denominator)
            );
        }
    }
    #[test]
    fn reduced_arithmetic_matches_cross_products_and_general_gcd() {
        let samples: [(i64, i64); 7] =
            [(0, -9), (3, -5), (6, 10), (2, 5), (9, -3), (5, 7), (13, 17)];
        for (a, b) in samples {
            let left = ExactRatio::new(a.into(), b.into());
            assert_eq!(left, literal_reduction(a.into(), b.into()));
            assert_eq!(left.negated(), literal_reduction((-a).into(), b.into()));
            for (c, d) in samples {
                let right = ExactRatio::new(c.into(), d.into());
                assert_eq!(
                    left.add(&right),
                    literal_reduction((a * d + b * c).into(), (b * d).into())
                );
                assert_eq!(
                    left.subtract(&right),
                    literal_reduction((a * d - b * c).into(), (b * d).into())
                );
                let sign = (b * d).signum();
                assert_eq!(left.cmp(&right), (sign * a * d).cmp(&(sign * b * c)));
            }
        }
    }

    fn literal_reduction(mut numerator: BigInt, mut denominator: BigInt) -> ExactRatio {
        if denominator.sign() == Sign::Minus {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = positive_gcd(numerator.clone(), denominator.clone());
        ExactRatio {
            numerator: numerator / &divisor,
            denominator: denominator / divisor,
        }
    }

    #[test]
    fn exact_binary64_conversion_preserves_subnormal_and_extreme_values() {
        for value in [0., -0., f64::from_bits(1), -f64::MAX, 0.1] {
            let dyadic = DyadicBound::from_f64(value).unwrap();
            assert_eq!(
                ExactRatio::from_finite(value),
                ExactRatio::from_dyadic(&dyadic)
            );
        }
        assert!(ExactRatio::from_finite(0.1) > ExactRatio::new(1.into(), 10.into()));
    }
}
