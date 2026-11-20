// SPDX-License-Identifier: GPL-2.0-only
//! Compare authored and stored cap envelopes in the selected cone's normal coordinates.
use super::*;
use crate::predicates::roots::{
    certificate_arithmetic::{bounded_dyadic::PositiveSqrtFailure, circular_constants},
    metric_bounds::{ConicalSectorWedge, ConicalTrimCarrier, conical_profile::angle},
};

/// Open trim band of the positive-radius nappe, expressed in normal coordinates.
pub struct ConicalTrimBand {
    sine: Interval,
    cosine: Interval,
    semi_angle: f64,
    slant: [f64; 2],
    angular: Option<[f64; 2]>,
    enclose_full_angles: bool,
}

/// Outward bounds of an admitted developed or stored cap footprint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConicalFootprint {
    /// Slant from the original apex, in model units.
    pub slant: [f64; 2],
    /// Physical radians relative to the seam, lifted into a partial trim when present.
    pub angular: Option<[f64; 2]>,
}

impl ConicalDevelopment {
    /// Capture the exact native trim endpoints and round the developed limits inward.
    /// Example: `let band = chart.trim_band([1., 8.], Some([0., 2.]))?;`.
    pub fn trim_band(
        &self,
        meridian: [f64; 2],
        angular: Option<[f64; 2]>,
    ) -> Checked<ConicalTrimBand> {
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
        let [v0, v1] = ordered(meridian, self.growing);
        let [lower, upper] = inward(
            self.meridian(v0, &mut arithmetic)?,
            self.meridian(v1, &mut arithmetic)?,
        )?;
        let slant = [lower.max(0.), upper];
        admit_open("positive slant band", slant)?;
        let angular = angular
            .map(|endpoints| self.trim_angles(endpoints, &mut arithmetic))
            .transpose()?;
        Ok(ConicalTrimBand {
            sine: self.sine.clone(),
            cosine: self.cosine.clone(),
            semi_angle: self.semi_angle,
            slant,
            angular,
            enclose_full_angles: false,
        })
    }

    fn trim_angles(
        &self,
        endpoints: [f64; 2],
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<[f64; 2]> {
        let [u0, u1] = ordered(endpoints, self.growing == self.direct);
        let band = inward(
            self.relative_angle(u0, arithmetic)?,
            self.relative_angle(u1, arithmetic)?,
        )?;
        admit_open("angular band", band)?;
        Ok(band)
    }

    /// Outward normal-chart enclosure of a bounded native UV trim.
    /// Example: `let bounds = chart.native_trim_envelope([[0.5, 0.75], [3., 4.]])?;`.
    pub fn native_trim_envelope(&self, uv: [[f64; 2]; 2]) -> Checked<ConicalFootprint> {
        admit_bounds(uv)?;
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
        let [v0, v1] = ordered(uv[1], self.growing);
        let [u0, u1] = ordered(uv[0], self.growing == self.direct);
        let slant = outward(
            self.meridian(v0, &mut arithmetic)?,
            self.meridian(v1, &mut arithmetic)?,
        )?;
        let angular = outward(
            self.relative_angle(u0, &mut arithmetic)?,
            self.relative_angle(u1, &mut arithmetic)?,
        )?;
        Ok(ConicalFootprint {
            slant,
            angular: Some(angular),
        })
    }
}

impl ConicalTrimBand {
    /// Retain unwrapped angles for full bands so their internal trims can be excluded.
    /// Example: `let checked = band.with_angular_enclosures();`.
    pub fn with_angular_enclosures(mut self) -> Self {
        self.enclose_full_angles = true;
        self
    }
    /// Inward positive slant bounds and optional native angular clip.
    /// Example: `let limits = band.limits();`.
    pub fn limits(&self) -> ConicalFootprint {
        ConicalFootprint {
            slant: self.slant,
            angular: self.angular,
        }
    }

    /// Prove containment of the full Cartesian enclosure, including any material inside it.
    /// Example: `band.authored([[4., 5.], [0., 1.]], ConicalSectorWedge::PositiveX)?;`.
    pub fn authored(
        &self,
        bounds: [[f64; 2]; 2],
        wedge: ConicalSectorWedge,
    ) -> Checked<ConicalFootprint> {
        admit_bounds(bounds)?;
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
        let slant = radius_bounds(bounds, &mut arithmetic)?;
        inside("authored slant", slant, self.slant)?;
        let angular = self.authored_angles(bounds, wedge, &mut arithmetic)?;
        Ok(ConicalFootprint { slant, angular })
    }

    fn authored_angles(
        &self,
        bounds: [[f64; 2]; 2],
        wedge: ConicalSectorWedge,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Option<[f64; 2]>> {
        if self.angular.is_none() && !self.enclose_full_angles {
            return Ok(None);
        }
        let phase = phase_bounds(bounds, wedge, arithmetic)?;
        let physical = arithmetic.divide(&phase, &self.sine)?;
        self.angle_enclosure(physical, arithmetic).map(Some)
    }

    /// Prove containment of retained pcurves projected along the canonical cone normals.
    /// Example: `band.fitted(carrier, [[0.25, 0.5], [1., 2.]])?;`.
    pub fn fitted(
        &self,
        carrier: ConicalTrimCarrier,
        uv: [[f64; 2]; 2],
    ) -> Checked<ConicalFootprint> {
        admit_bounds(uv)?;
        if carrier.semi_angle != self.semi_angle {
            return Err(ConicalDevelopmentFailure::Input {
                field: "cap semi-angle matching the support",
                value: carrier.semi_angle,
            });
        }
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
        let offset = self.cap_slant_origin(carrier, &mut arithmetic)?;
        let v = enclose(uv[1], &mut arithmetic)?;
        let cap_radius = arithmetic.scalar(carrier.radius)?;
        let radial_offset = arithmetic.multiply(&v, &self.sine)?;
        let radius = binary64(arithmetic.add(&cap_radius, &radial_offset)?)?;
        inside("fitted radius", radius, [0., f64::INFINITY])?;
        let slant = binary64(arithmetic.add(&offset, &v)?)?;
        inside("fitted slant", slant, self.slant)?;
        let angular = if self.angular.is_some() || self.enclose_full_angles {
            let physical = enclose(uv[0], &mut arithmetic)?;
            Some(self.angle_enclosure(physical, &mut arithmetic)?)
        } else {
            None
        };
        Ok(ConicalFootprint { slant, angular })
    }

    fn angle_enclosure(
        &self,
        actual: Interval,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<[f64; 2]> {
        match self.angular {
            Some(trim) => lift_into_trim(actual, trim, arithmetic),
            None => binary64(actual),
        }
    }

    fn cap_slant_origin(
        &self,
        carrier: ConicalTrimCarrier,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Interval> {
        // Projection on the unit meridian (sin(alpha), cos(alpha)) gives V a coefficient of one.
        let radius = arithmetic.scalar(carrier.radius)?;
        let axial = arithmetic.scalar(carrier.axial_origin)?;
        let radial_shift = arithmetic.multiply(&radius, &self.sine)?;
        let axial_shift = arithmetic.multiply(&axial, &self.cosine)?;
        Ok(arithmetic.add(&radial_shift, &axial_shift)?)
    }
}

fn admit_bounds(bounds: [[f64; 2]; 2]) -> Checked<()> {
    if bounds
        .iter()
        .any(|[a, b]| !a.is_finite() || !b.is_finite() || a > b)
    {
        return Err(ConicalDevelopmentFailure::Envelope { bounds });
    }
    Ok(())
}
fn ordered([a, b]: [f64; 2], positive: bool) -> [f64; 2] {
    if positive { [a, b] } else { [b, a] }
}
fn inward(first: Interval, last: Interval) -> Checked<[f64; 2]> {
    Ok([binary64(first)?[1], binary64(last)?[0]])
}
fn outward(first: Interval, last: Interval) -> Checked<[f64; 2]> {
    Ok([binary64(first)?[0], binary64(last)?[1]])
}
fn admit_open(field: &'static str, actual: [f64; 2]) -> Checked<()> {
    if actual[0] >= actual[1] {
        return Err(ConicalDevelopmentFailure::Footprint {
            field,
            actual,
            required: [f64::NEG_INFINITY, f64::INFINITY],
        });
    }
    Ok(())
}
fn enclose(bounds: [f64; 2], arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    Ok(Interval {
        lower: arithmetic.scalar(bounds[0])?.lower,
        upper: arithmetic.scalar(bounds[1])?.upper,
    })
}
fn inside(field: &'static str, actual: [f64; 2], required: [f64; 2]) -> Checked<()> {
    if !(actual[0] > required[0] && actual[1] < required[1]) {
        return Err(ConicalDevelopmentFailure::Footprint {
            field,
            actual,
            required,
        });
    }
    Ok(())
}

fn radius_bounds(bounds: [[f64; 2]; 2], arithmetic: &mut Arithmetic<'_>) -> Checked<[f64; 2]> {
    // The nearest point can be interior to the rectangle, even when every control lies farther out.
    let nearest = bounds.map(|[lo, hi]| lo.max(0.).max(-hi));
    let farthest = bounds.map(|[lo, hi]| lo.abs().max(hi.abs()));
    let [low, high] = [
        norm_squared(nearest, arithmetic)?,
        norm_squared(farthest, arithmetic)?,
    ];
    let squared = Interval {
        lower: low.lower,
        upper: high.upper,
    };
    let radius = arithmetic
        .positive_sqrt(&squared)
        .map_err(|cause| match cause {
            PositiveSqrtFailure::Arithmetic(cause) => cause.into(),
            PositiveSqrtFailure::InvalidInterval { .. } => {
                ConicalDevelopmentFailure::Envelope { bounds }
            }
        })?;
    binary64(radius)
}
fn norm_squared(point: [f64; 2], arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let [x, y] = [arithmetic.scalar(point[0])?, arithmetic.scalar(point[1])?];
    let [x2, y2] = [arithmetic.multiply(&x, &x)?, arithmetic.multiply(&y, &y)?];
    Ok(arithmetic.add(&x2, &y2)?)
}

fn phase_bounds(
    [x, y]: [[f64; 2]; 2],
    wedge: ConicalSectorWedge,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let bounds = [x, y];
    let negative_x = wedge == ConicalSectorWedge::NegativeX;
    if (negative_x && x[1] >= 0.) || (!negative_x && x[0] <= 0. && y[0] <= 0. && y[1] >= 0.) {
        return Err(ConicalDevelopmentFailure::Envelope { bounds });
    }
    let mut phase = corner_angle([x[0], y[0]], negative_x, arithmetic, bounds)?;
    for point in [[x[0], y[1]], [x[1], y[0]], [x[1], y[1]]] {
        let corner = corner_angle(point, negative_x, arithmetic, bounds)?;
        phase.lower = phase.lower.min(corner.lower);
        phase.upper = phase.upper.max(corner.upper);
    }
    Ok(phase)
}
fn corner_angle(
    point: [f64; 2],
    negative_x: bool,
    arithmetic: &mut Arithmetic<'_>,
    bounds: [[f64; 2]; 2],
) -> Checked<Interval> {
    let point = if negative_x {
        point.map(|coordinate| -coordinate)
    } else {
        point
    };
    let angle = angle::initial_angle(point, arithmetic)
        .map_err(|_| ConicalDevelopmentFailure::Envelope { bounds })?;
    if negative_x {
        let pi = circular_constants::pi(arithmetic)?;
        return Ok(arithmetic.add(&angle, &pi)?);
    }
    Ok(angle)
}

fn lift_into_trim(
    actual: Interval,
    required: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 2]> {
    // The floating midpoint proposes a lift. The outward period enclosure proves containment.
    let approximate = binary64(actual.clone())?;
    let midpoint_difference =
        required[0].midpoint(required[1]) - approximate[0].midpoint(approximate[1]);
    let turns = (midpoint_difference / std::f64::consts::TAU).round() as i64;
    let period = circular_constants::pi(arithmetic)?;
    let twice_turns = num_bigint::BigInt::from(turns) * 2;
    let shift = arithmetic.integer_scale(&period, &twice_turns)?;
    let actual = binary64(arithmetic.add(&actual, &shift)?)?;
    inside("physical angle", actual, required)?;
    Ok(actual)
}
