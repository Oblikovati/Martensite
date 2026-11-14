// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use num_bigint::BigInt;
#[derive(Clone)]
pub(super) struct HullBox {
    pub lower: [BigInt; 2],
    pub upper: [BigInt; 2],
}
impl HullBox {
    pub fn of(poles: &[Point]) -> Self {
        let mut result = Self {
            lower: poles[0].each_ref().map(|v| v.lower.clone()),
            upper: poles[0].each_ref().map(|v| v.upper.clone()),
        };
        for point in poles {
            for (axis, value) in point.iter().enumerate() {
                result.lower[axis] = result.lower[axis].clone().min(value.lower.clone());
                result.upper[axis] = result.upper[axis].clone().max(value.upper.clone());
            }
        }
        result
    }
    pub fn union(&self, other: &Self) -> Self {
        Self {
            lower: std::array::from_fn(|i| self.lower[i].clone().min(other.lower[i].clone())),
            upper: std::array::from_fn(|i| self.upper[i].clone().max(other.upper[i].clone())),
        }
    }
    pub fn separated(&self, other: &Self, shift: &[Interval; 2], delta: &Interval) -> bool {
        (0..2).any(|i| {
            &self.upper[i] + &delta.upper <= &other.lower[i] + &shift[i].lower
                || &other.upper[i] + &shift[i].upper + &delta.upper <= self.lower[i]
        })
    }
    pub fn width(&self) -> BigInt {
        (&self.upper[0] - &self.lower[0]).max(&self.upper[1] - &self.lower[1])
    }
    pub fn longest_axis(&self) -> usize {
        usize::from(&self.upper[1] - &self.lower[1] > &self.upper[0] - &self.lower[0])
    }
}
