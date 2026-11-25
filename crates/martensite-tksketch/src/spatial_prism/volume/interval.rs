// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkmath::gp::Transform3;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Span {
    pub lo: f64,
    pub hi: f64,
}
pub(crate) type Point = [Span; 3];

impl Span {
    pub const ZERO: Self = Self::at(0.);
    pub const ONE: Self = Self::at(1.);
    const UNKNOWN: Self = Self {
        lo: f64::NEG_INFINITY,
        hi: f64::INFINITY,
    };

    pub const fn at(value: f64) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    pub fn normalized(t: f64, [first, last]: [f64; 2]) -> Self {
        if t == first {
            return Self::ZERO;
        }
        if t == last {
            return Self::ONE;
        }
        let numerator = Self::at(t) - Self::at(first);
        let denominator = Self::at(last) - Self::at(first);
        numerator.divide_positive(denominator)
    }

    fn divide_positive(self, divisor: Self) -> Self {
        if !self.finite() || !divisor.finite() || divisor.lo <= 0.0 {
            return Self::UNKNOWN;
        }
        self * Self {
            lo: (1.0 / divisor.hi).next_down(),
            hi: (1.0 / divisor.lo).next_up(),
        }
    }

    pub fn hull(self, other: Self) -> Self {
        if !self.finite() || !other.finite() {
            return Self::UNKNOWN;
        }
        Self {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }

    pub fn magnitude(self) -> f64 {
        self.lo.abs().max(self.hi.abs())
    }

    pub fn finite(self) -> bool {
        self.lo.is_finite() && self.hi.is_finite()
    }
}

impl Add for Span {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if !self.finite() || !other.finite() {
            return Self::UNKNOWN;
        }
        if self == Self::ZERO {
            return other;
        }
        if other == Self::ZERO {
            return self;
        }
        if self == -other && self.lo == self.hi {
            return Self::ZERO;
        }
        let lo = self.lo + other.lo;
        let hi = self.hi + other.hi;
        Self {
            lo: lo.next_down(),
            hi: hi.next_up(),
        }
    }
}

impl Neg for Span {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }
}

impl Sub for Span {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self == other && self.lo == self.hi {
            return Self::ZERO;
        }
        self + (-other)
    }
}

impl Mul for Span {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if !self.finite() || !other.finite() {
            return Self::UNKNOWN;
        }
        if self == Self::ZERO || other == Self::ZERO {
            return Self::ZERO;
        }
        if self == Self::ONE {
            return other;
        }
        if self == Self::at(-1.) {
            return -other;
        }
        if other == Self::at(-1.) {
            return -self;
        }
        if other == Self::ONE {
            return self;
        }
        let products = [
            self.lo * other.lo,
            self.lo * other.hi,
            self.hi * other.lo,
            self.hi * other.hi,
        ];
        let lo = products
            .iter()
            .copied()
            .reduce(f64::min)
            .unwrap_or(f64::NAN);
        let hi = products
            .iter()
            .copied()
            .reduce(f64::max)
            .unwrap_or(f64::NAN);
        Self {
            lo: lo.next_down(),
            hi: hi.next_up(),
        }
    }
}

pub(super) fn mapped_vector(local: Point, transform: Transform3) -> Point {
    let matrix = transform.vectorial_part();
    std::array::from_fn(|axis| {
        matrix.coefficients()[axis]
            .into_iter()
            .zip(local)
            .fold(Span::ZERO, |sum, (coefficient, value)| {
                sum + Span::at(coefficient) * value
            })
    })
}

pub(super) fn placed(local: Point, transform: Transform3, reference: [f64; 3]) -> Point {
    let vector = mapped_vector(local, transform);
    let translation = transform.translation_part();
    std::array::from_fn(|axis| {
        let offset = Span::at(translation.coordinates()[axis]) - Span::at(reference[axis]);
        offset + vector[axis]
    })
}

pub(crate) fn l1_difference(a: Point, b: Point) -> f64 {
    if a.into_iter().chain(b).any(|span| !span.finite()) {
        return f64::INFINITY;
    }
    a.into_iter()
        .zip(b)
        .fold(Span::ZERO, |bound, (a, b)| {
            bound + Span::at((a - b).magnitude())
        })
        .hi
}

pub(crate) fn determinant(a: Point, b: Point, c: Point) -> Span {
    (0..3).fold(Span::ZERO, |total, i| {
        let j = (i + 1) % 3;
        let k = (i + 2) % 3;
        total + a[i] * (b[j] * c[k] - b[k] * c[j])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_operations_contain_signed_extrema_and_translated_determinant() {
        let x = Span { lo: -2., hi: 3. };
        let y = Span { lo: -5., hi: 7. };
        assert!((x * y).lo <= -15. && (x * y).hi >= 21.);
        let factor = 134_217_728.;
        let p = placed(
            [Span::at(factor), Span::ZERO, Span::ZERO],
            Transform3::default(),
            [factor, 0., 0.],
        );
        assert_eq!(p, [Span::ZERO; 3]);
        let axis = |index| std::array::from_fn(|i| Span::at(if i == index { 1. } else { 0. }));
        let det = determinant(axis(0), axis(1), axis(2));
        assert_eq!(det, Span::ONE);
    }
}
