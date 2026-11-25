// SPDX-License-Identifier: GPL-2.0-only
//! A difference-of-squares lower bound in scaled cursor coordinates.
use super::{Component, Prepared};

// Sufficient decrease keeps nearly undamped reflected steps from crawling near a curved optimum.
const SUFFICIENT_FRACTION: f64 = 0.125;

pub(super) fn improves(
    program: &Prepared,
    component: &Component,
    prior: &[f64],
    next: &[f64],
    targets: &[(usize, f64)],
    prediction: f64,
) -> bool {
    let mut lower: f64 = 0.;
    for &(local, target) in targets {
        let i = component.free[local];
        if prior[i] == next[i] {
            continue;
        }
        let scale = program.domains[i].scale();
        let Some(term) = coordinate_gain(prior[i], next[i], target, scale) else {
            return false;
        };
        lower = (lower + term).next_down();
    }
    let required = (2. * SUFFICIENT_FRACTION * prediction).next_up();
    lower.is_finite() && lower > required
}

fn coordinate_gain(old: f64, new: f64, target: f64, scale: f64) -> Option<f64> {
    // The product is the old squared cursor distance minus the new one.
    let step = Bounds::difference(new, old)?.divide(scale)?;
    let before = Bounds::difference(target, old)?.divide(scale)?;
    let after = Bounds::difference(target, new)?.divide(scale)?;
    step.product_lower(before.add(after)?)
}

#[derive(Clone, Copy)]
struct Bounds(f64, f64);

impl Bounds {
    fn outward(lower: f64, upper: f64) -> Option<Self> {
        let (lo, hi) = (lower.next_down(), upper.next_up());
        (lo.is_finite() && hi.is_finite()).then_some(Self(lo, hi))
    }

    fn difference(first: f64, second: f64) -> Option<Self> {
        if first == second {
            return Some(Self(0., 0.));
        }
        let delta = first - second;
        Self::outward(delta, delta)
    }

    fn divide(self, positive: f64) -> Option<Self> {
        Self::outward(self.0 / positive, self.1 / positive)
    }

    fn add(self, other: Self) -> Option<Self> {
        Self::outward(self.0 + other.0, self.1 + other.1)
    }

    fn product_lower(self, other: Self) -> Option<f64> {
        let products = [
            self.0 * other.0,
            self.0 * other.1,
            self.1 * other.0,
            self.1 * other.1,
        ];
        if products.iter().any(|product| !product.is_finite()) {
            return None;
        }
        Some(
            products
                .into_iter()
                .fold(f64::INFINITY, f64::min)
                .next_down(),
        )
    }
}
