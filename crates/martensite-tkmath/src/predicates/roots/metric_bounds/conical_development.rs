// SPDX-License-Identifier: GPL-2.0-only
//! Analytic development of one positive-radius native cone nappe.
use super::{
    BoundedArithmeticFailure, CylindricalProfileFailure, cylindrical_profile::trigonometric,
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};
mod trim_band;
pub use trim_band::{ConicalFootprint, ConicalTrimBand};

/// A cone chart with a certified native-to-apex slant offset and angular scale.
#[derive(Clone, Debug)]
pub struct ConicalDevelopment {
    sine: Interval,
    cosine: Interval,
    semi_angle: f64,
    offset: Interval,
    seam: f64,
    growing: bool,
    direct: bool,
}

/// A native cone parameter did not give a finite developed point on the selected nappe.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConicalDevelopmentFailure {
    /// The supplied chart parameter is outside the supported scalar domain.
    Input {
        /// Invalid field.
        field: &'static str,
        /// Supplied value.
        value: f64,
    },
    /// The parameter cannot be certified strictly on the positive-radius nappe.
    Slant {
        /// Native meridian coordinate.
        v: f64,
        /// Outward lower slant endpoint.
        lower: DyadicBound,
    },
    /// The directed interval computation could not complete.
    Arithmetic(BoundedArithmeticFailure),
    /// The existing outward trigonometric enclosure could not complete.
    Trigonometric(Box<CylindricalProfileFailure>),
    /// An interval could not be ordered or strictly enclosed by the trim.
    Footprint {
        /// Quantity in the developed normal chart.
        field: &'static str,
        /// Actual outward interval or rejected endpoints.
        actual: [f64; 2],
        /// Required open interval.
        required: [f64; 2],
    },
    /// A developed control rectangle or cap envelope could not be resolved.
    Envelope {
        /// Rejected ordered Cartesian or UV intervals.
        bounds: [[f64; 2]; 2],
    },
}

type Checked<T> = Result<T, ConicalDevelopmentFailure>;
impl From<BoundedArithmeticFailure> for ConicalDevelopmentFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

impl ConicalDevelopment {
    /// Capture a signed native semi-angle, reference radius, seam and frame handedness.
    /// Example: `let chart = ConicalDevelopment::new(0.5, 2.0, 0.0, true)?;`.
    pub fn new(semi_angle: f64, radius: f64, seam: f64, direct: bool) -> Checked<Self> {
        admit(semi_angle, radius, seam)?;
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
        let angle = arithmetic.scalar(semi_angle.abs())?;
        let [cosine, sine] = trig(&angle, &mut arithmetic)?;
        let radius = arithmetic.scalar(radius)?;
        let offset = arithmetic.divide(&radius, &sine)?;
        Ok(Self {
            sine,
            cosine,
            semi_angle: semi_angle.abs(),
            offset,
            seam,
            growing: semi_angle > 0.,
            direct,
        })
    }

    /// Enclose developed Cartesian coordinates for exact binary64 native UV.
    /// Example: `let [x_bounds, y_bounds] = chart.bounds([0.25, 3.0])?;`.
    pub fn bounds(&self, [u, v]: [f64; 2]) -> Checked<[[f64; 2]; 2]> {
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
        let slant = self.slant(v, &mut arithmetic)?;
        let phase = self.phase(u, &mut arithmetic)?;
        let [cosine, sine] = trig(&phase, &mut arithmetic)?;
        let x = arithmetic.multiply(&slant, &cosine)?;
        let y = arithmetic.multiply(&slant, &sine)?;
        Ok([binary64(x)?, binary64(y)?])
    }

    fn slant(&self, v: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
        let slant = self.meridian(v, arithmetic)?;
        if slant.lower <= 0.into() {
            return Err(ConicalDevelopmentFailure::Slant {
                v,
                lower: dyadic(slant.lower),
            });
        }
        Ok(slant)
    }

    fn meridian(&self, v: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
        let native = arithmetic.scalar(if self.growing { v } else { -v })?;
        Ok(arithmetic.add(&self.offset, &native)?)
    }

    fn phase(&self, u: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
        let angle = self.relative_angle(u, arithmetic)?;
        Ok(arithmetic.multiply(&angle, &self.sine)?)
    }

    fn relative_angle(&self, u: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
        let u = arithmetic.scalar(u)?;
        let seam = arithmetic.scalar(self.seam)?;
        let sign_matches = self.growing == self.direct;
        let (first, last) = if sign_matches {
            (&u, &seam)
        } else {
            (&seam, &u)
        };
        Ok(arithmetic.subtract(first, last)?)
    }
}

fn admit(angle: f64, radius: f64, seam: f64) -> Checked<()> {
    for (field, value, admitted) in [
        (
            "native semi-angle",
            angle,
            angle.abs() > 0. && angle.abs() < std::f64::consts::FRAC_PI_2,
        ),
        (
            "native reference radius",
            radius,
            radius.is_finite() && radius >= 0.,
        ),
        ("native seam", seam, seam.is_finite()),
    ] {
        if !admitted {
            return Err(ConicalDevelopmentFailure::Input { field, value });
        }
    }
    Ok(())
}
fn trig(angle: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<[Interval; 2]> {
    trigonometric::scalar(angle, arithmetic)
        .map_err(|cause| ConicalDevelopmentFailure::Trigonometric(Box::new(cause)))
}
fn dyadic(value: num_bigint::BigInt) -> DyadicBound {
    DyadicBound::new(
        value,
        -i32::from(ArithmeticPolicy::default().fractional_bits),
    )
}
fn binary64(interval: Interval) -> Checked<[f64; 2]> {
    let [lower, upper] = [dyadic(interval.lower), dyadic(interval.upper)];
    let project = |bound: DyadicBound| {
        bound
            .binary64_bounds()
            .ok_or(BoundedArithmeticFailure::UnrepresentableBound { bound })
    };
    Ok([project(lower)?[0], project(upper)?[1]])
}

impl std::fmt::Display for ConicalDevelopmentFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Footprint {
            field,
            actual,
            required,
        } = self
        {
            return write!(
                f,
                "{field}={actual:?}, expected a finite enclosure strictly inside {required:?}"
            );
        }
        if let Self::Envelope { bounds } = self {
            return write!(
                f,
                "footprint envelope={bounds:?}, expected ordered finite bounds and a resolved chart away from the apex"
            );
        }
        write!(
            f,
            "{self:?}, expected finite UV on the positive-radius cone nappe"
        )
    }
}
impl std::error::Error for ConicalDevelopmentFailure {}

#[cfg(test)]
mod tests;
