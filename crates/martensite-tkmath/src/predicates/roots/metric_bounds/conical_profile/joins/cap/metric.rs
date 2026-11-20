// SPDX-License-Identifier: GPL-2.0-only
//! World-distance bounds use each span's own slant hull and simultaneous residuals.
use super::*;

pub(in super::super) struct MotionMetric<'a> {
    sine: &'a Interval,
    radius_offset: Interval,
}
impl<'a> MotionMetric<'a> {
    pub(in super::super) fn new(
        reference: f64,
        maximum_depth: f64,
        [sine, cosine]: [&'a Interval; 2],
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Self> {
        let reference = arithmetic.scalar(reference)?;
        let depth = arithmetic.scalar(maximum_depth)?;
        let radius = arithmetic.multiply(&reference, sine)?;
        let offset = arithmetic.multiply(&depth, cosine)?;
        let radius_offset = arithmetic.add(&radius, &offset)?;
        Ok(Self {
            sine,
            radius_offset,
        })
    }

    pub(in super::super) fn squared_motion(
        &self,
        slant_hull: &Interval,
        angle: &Interval,
        slant: &Interval,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Interval> {
        let swept = arithmetic.add(slant_hull, slant)?;
        let slant_radius = arithmetic.multiply(&swept, self.sine)?;
        let radius = arithmetic.add(&slant_radius, &self.radius_offset)?;
        let arc = arithmetic.multiply(&upper_magnitude(radius), angle)?;
        // At equal depth the squared distance is slant difference squared plus
        // 2*r_first*r_last*(1-cos(delta_u)). The local swept radius bounds both factors.
        let angular_squared = arithmetic.multiply(&arc, &arc)?;
        let slant_squared = arithmetic.multiply(slant, slant)?;
        arithmetic.add(&slant_squared, &angular_squared)
    }
}

fn upper_magnitude(radius: Interval) -> Interval {
    // The normal reference checks positivity of the entire swept radius independently.
    let upper = radius.upper.max(0.into());
    Interval {
        lower: upper.clone(),
        upper,
    }
}
