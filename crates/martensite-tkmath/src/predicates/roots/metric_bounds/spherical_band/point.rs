// SPDX-License-Identifier: GPL-2.0-only
//! Original lifted sphere coordinates, exact latitude sides and checked world metrics.
mod lift;

use super::{
    Arithmetic, Checked, ExactVector, POLICY, R, SphericalBand, SphericalBandError as Error,
    exact_vector, poll,
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::rounding::{nearest_binary64, outward_binary64},
    metric_bounds::{
        placed_plane::{dot, triple},
        segment_cylinder::require_nearby_lift,
    },
};
use std::{cmp::Ordering, f64::consts::TAU};

/// Location against the original closed latitude trims of a full spherical band.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SphericalBandPlace {
    /// Below the original lower rim.
    Below,
    /// Exactly on the lower latitude.
    FirstRim,
    /// Strictly between the latitude limits.
    Interior,
    /// Exactly on the upper latitude.
    LastRim,
    /// Above the original upper rim.
    Above,
}

/// Certified retained spherical point acceptance evidence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphericalPointQuery {
    /// Exact or interval-certified location of the retained world ray's latitude.
    pub world_place: SphericalBandPlace,
    /// Literal location of the actual solver latitude.
    pub uv_place: SphericalBandPlace,
    /// Outward bounds of the original world lift at the actual supplied UV.
    pub lifted: [[f64; 2]; 3],
    /// Certified inclusive Euclidean discrepancy between retained XYZ and the ideal lift.
    pub distance_bound: f64,
}

/// Nominal analytic spherical chart anchored at the original unwrapped UV.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphericalPointChart {
    /// Unwrapped longitude and latitude at the anchor.
    pub uv: [f64; 2],
    /// Nominal world point at the anchor.
    pub world: [f64; 3],
    /// Radius-scaled original radial basis, longitude tangent basis and axial basis.
    pub basis: [[f64; 3]; 3],
    /// Cosine and sine of the anchor latitude.
    pub latitude_phase: [f64; 2],
    /// Nominal longitude and latitude world tangents for parameter scaling.
    pub tangents: [[f64; 3]; 2],
}

impl SphericalBand {
    /// Positive whole-band lower bound on the world chart's tangent area divided by radius squared.
    /// Example: `band.relative_area_lower()`.
    pub fn relative_area_lower(&self) -> f64 {
        self.relative_area_lower
    }

    /// Seed lifted UV near an angular hint. Final distances and trim need independent qualification.
    /// Example: `band.point_seed(xyz, prior_u)?`.
    pub fn point_seed(&self, xyz: [f64; 3], near: f64) -> Checked<[f64; 2]> {
        let [x, y, z] = self.local(xyz)?;
        let radial = absolute(&x).max(absolute(&y));
        let principal =
            martensite_cmath::atan2(rounded(&y.divide(&radial))?, rounded(&x.divide(&radial))?);
        let scale = radial.max(absolute(&z));
        let x = rounded(&x.divide(&scale))?;
        let y = rounded(&y.divide(&scale))?;
        let z = rounded(&z.divide(&scale))?;
        let turns = ((near - principal) / TAU).round();
        let u = turns.mul_add(TAU, principal);
        require_spherical_lift(near, u)?;
        let v = martensite_cmath::atan2(z, martensite_cmath::hypot(x, y));
        Ok([u, v])
    }

    /// Classify a retained world ray against both exact selected latitude rims.
    /// Example: `band.point_place(xyz)?`.
    pub fn point_place(&self, xyz: [f64; 3]) -> Checked<SphericalBandPlace> {
        self.local_place(&self.local(xyz)?)
    }

    /// Certify the actual unwrapped UV and retained world point at the original radius.
    /// Example: `band.point_qualify(uv, xyz, 1e-8, || false)?`.
    pub fn point_qualify(
        &self,
        uv: [f64; 2],
        xyz: [f64; 3],
        limit: f64,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<SphericalPointQuery> {
        poll(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(Error::Limit(limit));
        }
        let [angle, latitude, _] = exact_vector([uv[0], uv[1], 0.], 1)?;
        let world_place = self.point_place(xyz)?;
        let uv_place = place(
            uv[1].partial_cmp(&self.latitude[0]),
            uv[1].partial_cmp(&self.latitude[1]),
        )?;
        let retained = exact_vector(xyz, 0)?;
        let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
        let (lifted, distance_bound) =
            lift::qualify(self, [&angle, &latitude], &retained, limit, &mut arithmetic)?;
        poll(&mut cancelled)?;
        Ok(SphericalPointQuery {
            world_place,
            uv_place,
            lifted,
            distance_bound,
        })
    }

    /// Read the nominal local solver chart from the same retained coefficients and phase.
    /// Example: `band.point_chart(uv, || false)?`.
    pub fn point_chart(
        &self,
        uv: [f64; 2],
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<SphericalPointChart> {
        let [angle, latitude, _] = exact_vector([uv[0], uv[1], 0.], 1)?;
        let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
        let chart = lift::chart(self, uv, [&angle, &latitude], &mut arithmetic)?;
        poll(&mut cancelled)?;
        Ok(chart)
    }

    /// Check exact opposing radial directions in the frozen original placement.
    /// Example: `band.point_antipodes(previous, cursor)?`.
    pub fn point_antipodes(&self, previous: [f64; 3], cursor: [f64; 3]) -> Checked<bool> {
        let [ax, ay, _] = self.local(previous)?;
        let [bx, by, _] = self.local(cursor)?;
        let cross = ax.multiply(&by).subtract(&ay.multiply(&bx));
        let dot = ax.multiply(&bx).add(&ay.multiply(&by));
        Ok(cross.sign().is_eq() && dot.sign().is_lt())
    }

    fn local(&self, xyz: [f64; 3]) -> Checked<ExactVector> {
        let point = exact_vector(xyz, 0)?;
        let delta = std::array::from_fn(|i| point[i].subtract(&self.origin[i]));
        let local = self.inverse.each_ref().map(|row| dot(row, &delta));
        if local[0].sign().is_eq() && local[1].sign().is_eq() {
            return Err(Error::Pole);
        }
        Ok(local)
    }

    fn local_place(&self, position: &ExactVector) -> Checked<SphericalBandPlace> {
        let norm_squared = dot(position, position);
        let height = &position[2];
        let height_squared = height.multiply(height);
        place(
            latitude_side(
                height,
                &height_squared,
                &norm_squared,
                self.latitude[0],
                &self.sine_rims[0],
            ),
            latitude_side(
                height,
                &height_squared,
                &norm_squared,
                self.latitude[1],
                &self.sine_rims[1],
            ),
        )
    }
}

fn latitude_side(
    height: &R,
    height_squared: &R,
    norm_squared: &R,
    rim: f64,
    [low, high]: &[R; 2],
) -> Option<Ordering> {
    let height_sign = height.sign();
    let rim_sign = R::from_finite(rim).sign();
    if height_sign != rim_sign || height_sign.is_eq() {
        return Some(height_sign.cmp(&rim_sign));
    }
    let magnitude = if rim_sign.is_gt() {
        [low.clone(), high.clone()]
    } else {
        [high.negated(), low.negated()]
    };
    let zero = R::from_finite(0.);
    let endpoints = magnitude.map(|sine| {
        let sine = sine.max(zero.clone());
        sine.multiply(&sine).multiply(norm_squared)
    });
    let a = &endpoints[0];
    let b = &endpoints[1];
    let order = if height_squared < a.min(b) {
        Ordering::Less
    } else if height_squared > a.max(b) {
        Ordering::Greater
    } else if a == b && height_squared == a {
        Ordering::Equal
    } else {
        return None;
    };
    Some(if rim_sign.is_lt() {
        order.reverse()
    } else {
        order
    })
}

fn place(first: Option<Ordering>, last: Option<Ordering>) -> Checked<SphericalBandPlace> {
    use SphericalBandPlace::*;
    match (first, last) {
        (Some(Ordering::Less), _) => Ok(Below),
        (_, Some(Ordering::Greater)) => Ok(Above),
        (Some(Ordering::Equal), _) => Ok(FirstRim),
        (_, Some(Ordering::Equal)) => Ok(LastRim),
        (Some(Ordering::Greater), Some(Ordering::Less)) => Ok(Interior),
        _ => Err(Error::TrimUnresolved),
    }
}

/// Require a uniquely admitted exact binary64 longitude step strictly within half a turn.
/// Example: `require_spherical_lift(prior, next)?`.
pub fn require_spherical_lift(prior: f64, next: f64) -> Checked<()> {
    require_nearby_lift(prior, next).map_err(|_| Error::Lift { prior, next })
}

pub(super) fn admit(
    band: &SphericalBand,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([[R; 2]; 2], f64)> {
    let [a, b] = band.latitude;
    let [ca, sa] = super::scalar_trig(a, arithmetic)?;
    let [cb, sb] = super::scalar_trig(b, arithmetic)?;
    // On the admitted non-pole span, cos is positive and its minimum is at a rim.
    let cosine = grid(ca.lower.min(cb.lower), arithmetic);
    let area = area_lower(&band.axes, &cosine)?;
    let rim = |sine: crate::predicates::roots::certificate_arithmetic::bounded_dyadic::Interval| {
        [grid(sine.lower, arithmetic), grid(sine.upper, arithmetic)]
    };
    Ok(([rim(sa), rim(sb)], area))
}

fn area_lower(axes: &[ExactVector; 3], min_cosine: &R) -> Checked<f64> {
    let determinant = absolute(&triple(&axes[0], &axes[1], &axes[2]));
    let sum = axes
        .iter()
        .flatten()
        .fold(R::from_finite(0.), |sum, value| sum.add(&absolute(value)));
    // For unit tangent axes a,b, (Aa)x(Ab)=det(A)A^-T(a x b).
    // The sum of absolute chart entries upper-bounds the matrix spectral norm.
    let bound = determinant.multiply(min_cosine).divide(&sum);
    let low = outward_binary64(&bound).ok_or(Error::Unrepresentable)?[0];
    if low <= 0. {
        return Err(Error::Pole);
    }
    Ok(low)
}

fn absolute(value: &R) -> R {
    if value.sign().is_lt() {
        value.negated()
    } else {
        value.clone()
    }
}
fn rounded(value: &R) -> Checked<f64> {
    nearest_binary64(value).ok_or(Error::Unrepresentable)
}
fn grid(value: num_bigint::BigInt, arithmetic: &Arithmetic<'_>) -> R {
    R::from_dyadic(&DyadicBound::new(
        value,
        -i32::from(arithmetic.fractional_bits()),
    ))
}
