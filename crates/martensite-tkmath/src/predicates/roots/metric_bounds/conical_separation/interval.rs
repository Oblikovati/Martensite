// SPDX-License-Identifier: GPL-2.0-only
//! Directed binary64 enclosure for the conditioned, bounded-magnitude filter.
use super::SeparationSense;

#[derive(Clone, Copy)]
pub(super) struct Enclosure {
    pub lo: f64,
    pub hi: f64,
}

impl Enclosure {
    pub const ZERO: Self = Self::point(0.);

    pub const fn point(x: f64) -> Self {
        Self { lo: x, hi: x }
    }

    pub fn add(self, other: Self) -> Self {
        if self.lo == 0. && self.hi == 0. {
            return other;
        }
        if other.lo == 0. && other.hi == 0. {
            return self;
        }
        Self {
            lo: (self.lo + other.lo).next_down(),
            hi: (self.hi + other.hi).next_up(),
        }
    }

    pub fn subtract(self, other: Self) -> Self {
        self.add(Self {
            lo: -other.hi,
            hi: -other.lo,
        })
    }

    pub fn multiply(self, other: Self) -> Self {
        if (self.lo == 0. && self.hi == 0.) || (other.lo == 0. && other.hi == 0.) {
            return Self::ZERO;
        }
        let p = [
            self.lo * other.lo,
            self.lo * other.hi,
            self.hi * other.lo,
            self.hi * other.hi,
        ];
        Self {
            lo: p.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            hi: p.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        }
    }

    pub fn divide_positive(self, other: Self) -> Self {
        self.multiply(Self {
            lo: other.hi.recip().next_down(),
            hi: other.lo.recip().next_up(),
        })
    }

    pub fn square(self) -> Self {
        self.multiply(self)
    }

    pub fn positive_sqrt(self) -> Option<Self> {
        if self.lo <= 0. {
            return None;
        }
        let mut lo = self.lo.sqrt();
        let mut hi = self.hi.sqrt();
        for _ in 0..4 {
            lo = lo.next_down();
            hi = hi.next_up();
            if Self::point(lo).square().hi <= self.lo && Self::point(hi).square().lo >= self.hi {
                return Some(Self { lo, hi });
            }
        }
        None
    }

    pub fn finite(self) -> bool {
        self.lo.is_finite() && self.hi.is_finite() && self.lo <= self.hi
    }

    pub fn magnitude(self) -> f64 {
        self.lo.abs().max(self.hi.abs())
    }

    pub fn directed_lower(self, sense: SeparationSense) -> f64 {
        match sense {
            SeparationSense::Outside => self.lo,
            SeparationSense::Inside => -self.hi,
        }
    }
}
