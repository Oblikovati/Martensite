// SPDX-License-Identifier: GPL-2.0-only
//! Operation-local common coefficients never carry a depth-specific acceptance verdict.
use super::*;
pub(in super::super) struct CommonTrace<'a> {
    pcurve: &'a [[f64; 2]],
    pub(super) uv: [Vec<Interval>; 2],
    pub(super) angular: Interval,
    pub(super) delta: Vec<Interval>,
}
impl<'a> CommonTrace<'a> {
    pub(super) fn new(
        pcurve: &'a [[f64; 2]],
        uv: [Vec<Interval>; 2],
        angular: Interval,
        delta: Vec<Interval>,
    ) -> Self {
        Self {
            pcurve,
            uv,
            angular,
            delta,
        }
    }
    pub(super) fn matches(&self, pcurve: &[[f64; 2]]) -> bool {
        pcurve.len() == self.pcurve.len()
            && pcurve
                .iter()
                .flatten()
                .zip(self.pcurve.iter().flatten())
                .all(|(actual, old)| actual.to_bits() == old.to_bits())
    }
}

impl CommonTrace<'_> {
    pub(in super::super) fn into_retained(
        self,
    ) -> super::super::super::developed_boundary::RetainedOperands {
        super::super::super::developed_boundary::RetainedOperands {
            uv: self.uv.each_ref().map(|axis| algebra::hull(axis)),
            angular: self.angular,
            delta: self.delta,
        }
    }
}
