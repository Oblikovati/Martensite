//! Outward enclosures of finite binary64 source values.
use core::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub(super) struct I(pub(super) f64, pub(super) f64);
impl I {
    pub(super) fn p(x: f64) -> Self {
        Self(x, x)
    }
    pub(super) fn valid(self) -> bool {
        self.0.is_finite() && self.1.is_finite() && self.0 <= self.1
    }
    pub(super) fn square(self) -> Self {
        let low = if self.0 <= 0. && 0. <= self.1 {
            0.
        } else {
            self.0.abs().min(self.1.abs())
        };
        let high = self.0.abs().max(self.1.abs());
        Self((low * low).next_down().max(0.), (high * high).next_up())
    }
}
impl Add for I {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0).next_down(), (self.1 + rhs.1).next_up())
    }
}
impl Sub for I {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self((self.0 - rhs.1).next_down(), (self.1 - rhs.0).next_up())
    }
}
impl Mul for I {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let vals = [
            self.0 * rhs.0,
            self.0 * rhs.1,
            self.1 * rhs.0,
            self.1 * rhs.1,
        ];
        Self(
            vals.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            vals.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        )
    }
}
impl Div for I {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.0 <= 0. && 0. <= rhs.1 {
            return Self(f64::NAN, f64::NAN);
        }
        self * Self((1. / rhs.1).next_down(), (1. / rhs.0).next_up())
    }
}
