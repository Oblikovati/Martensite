// SPDX-License-Identifier: GPL-2.0-only
//! Green's theorem gives the signed cap moments from polynomial coefficients.
use super::*;
use crate::predicates::roots::{
    certificate_arithmetic::{
        bernstein_bounds::{integrate_unit, lower, product},
        bernstein_curve::{capture_planar, derivative},
    },
    metric_bounds::cylindrical_profile::trigonometric,
};
mod types;
pub use types::*;
#[cfg(test)]
mod tests;

type VolumeResult<T> = Result<T, ConicalCapVolumeFailure>;

impl ConicalCapHomotopyEvidence<'_> {
    /// Enclose the signed volume integral of the exact normal extrusion of the cap profile.
    /// Wire orientation gives the sign. Periodic filled-material admission is separate.
    /// Example: `let volume = homotopy.signed_normal_volume(options, || false)?;`.
    pub fn signed_normal_volume(
        &self,
        options: ConeClearanceOptions,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<ConicalCapVolumeEvidence, ConicalCapVolumeError> {
        let policy = sector_api::policy(options);
        sector::admit_policy(policy).map_err(|cause| ConicalCapVolumeError {
            cause: ConicalCapVolumeFailure::Policy(cause),
            span: None,
            operations: 0,
        })?;
        let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
        let mut moments = Moments::empty(&arithmetic);
        for (index, span) in self.cap.spans.iter().enumerate() {
            moments
                .include(&span.uv, &mut arithmetic)
                .map_err(|cause| error(cause.into(), Some(index), &arithmetic))?;
        }
        let volume = finish(self.cap, moments, &mut arithmetic)
            .map_err(|cause| error(cause, None, &arithmetic))?;
        Ok(ConicalCapVolumeEvidence {
            enclosure: exact_enclosure(&volume, options.fractional_bits),
            operations: arithmetic.operations(),
        })
    }
}

struct Moments {
    area: Interval,
    slant: Interval,
    minimum_v: Option<num_bigint::BigInt>,
}
impl Moments {
    fn empty(arithmetic: &Arithmetic<'_>) -> Self {
        Self {
            area: arithmetic.zero(),
            slant: arithmetic.zero(),
            minimum_v: None,
        }
    }

    fn include(&mut self, uv: &[[u64; 2]], arithmetic: &mut Arithmetic<'_>) -> Checked<()> {
        let points = uv
            .iter()
            .map(|point| point.map(f64::from_bits))
            .collect::<Vec<_>>();
        let [u, v] = capture_planar(&points, arithmetic)?;
        let du = derivative(&u, arithmetic)?;
        let area = integrate_unit(&product(&v, &du, arithmetic)?, arithmetic)?;
        let squared = product(&v, &v, arithmetic)?;
        let slant = integrate_unit(&product(&squared, &du, arithmetic)?, arithmetic)?;
        self.area = arithmetic.add(&self.area, &area)?;
        self.slant = arithmetic.add(&self.slant, &slant)?;
        let minimum = lower(&v);
        self.minimum_v = Some(match self.minimum_v.take() {
            Some(old) => old.min(minimum),
            None => minimum,
        });
        Ok(())
    }
}

fn finish(
    cap: &CapBinding,
    moments: Moments,
    arithmetic: &mut Arithmetic<'_>,
) -> VolumeResult<Interval> {
    let meridian = Meridian::capture(cap, arithmetic)?;
    let minimum = moments.minimum_v.ok_or(ConicalCapVolumeFailure::Empty)?;
    meridian.admit_radius(minimum, arithmetic)?;
    let radius_area = arithmetic.multiply(&meridian.center_radius, &moments.area)?;
    let slant = arithmetic.fraction(&moments.slant, 1, 2)?;
    let radial_moment = arithmetic.multiply(&meridian.sine, &slant)?;
    let cross_section = arithmetic.add(&radius_area, &radial_moment)?;
    Ok(arithmetic.multiply(&cross_section, &meridian.thickness)?)
}

struct Meridian {
    center_radius: Interval,
    reference_radius: Interval,
    depth_minimum: Interval,
    thickness: Interval,
    sine: Interval,
}
impl Meridian {
    fn capture(cap: &CapBinding, arithmetic: &mut Arithmetic<'_>) -> VolumeResult<Self> {
        let alpha = arithmetic.scalar(f64::from_bits(cap.support[0]))?;
        let [cosine, sine] = trigonometric::scalar(&alpha, arithmetic)
            .map_err(ConicalCapVolumeFailure::Trigonometric)?;
        let reference = arithmetic.scalar(f64::from_bits(cap.reference))?;
        let reference_radius = arithmetic.multiply(&reference, &sine)?;
        let [midpoint, lowest, thickness] = depth_range(cap, arithmetic)?;
        let center_depth = arithmetic.multiply(&midpoint, &cosine)?;
        Ok(Self {
            center_radius: arithmetic.add(&reference_radius, &center_depth)?,
            reference_radius,
            depth_minimum: arithmetic.multiply(&lowest, &cosine)?,
            thickness,
            sine,
        })
    }

    fn admit_radius(
        &self,
        lower: num_bigint::BigInt,
        arithmetic: &mut Arithmetic<'_>,
    ) -> VolumeResult<()> {
        let minimum_v = Interval {
            lower: lower.clone(),
            upper: lower,
        };
        let slant = arithmetic.multiply(&minimum_v, &self.sine)?;
        let radius = arithmetic.add(&self.reference_radius, &slant)?;
        let radius = arithmetic.add(&radius, &self.depth_minimum)?;
        if radius.lower <= 0.into() {
            let lower = DyadicBound::new(radius.lower, -i32::from(arithmetic.fractional_bits()));
            return Err(ConicalCapVolumeFailure::Radius { lower });
        }
        Ok(())
    }
}

fn depth_range(cap: &CapBinding, arithmetic: &mut Arithmetic<'_>) -> Checked<[Interval; 3]> {
    let [_, _, first, last, outward] = cap.support;
    let [a, b] = [f64::from_bits(first), f64::from_bits(last)];
    let [first, last] = [arithmetic.scalar(a)?, arithmetic.scalar(b)?];
    let depth_sum = arithmetic.add(&first, &last)?;
    let midpoint = arithmetic.fraction(&depth_sum, 1, 2)?;
    let thickness = arithmetic.subtract(&last, &first)?;
    if outward != 0 {
        return Ok([midpoint, first, thickness]);
    }
    let zero = arithmetic.zero();
    Ok([
        arithmetic.subtract(&zero, &midpoint)?,
        arithmetic.subtract(&zero, &last)?,
        thickness,
    ])
}
