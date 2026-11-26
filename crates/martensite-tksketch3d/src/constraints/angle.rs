// SPDX-License-Identifier: GPL-2.0-only
//! Authored open-interval line angles in radians.

use std::{error::Error, f64::consts::PI, fmt};

/// Directed line angle in the open interval from zero to pi radians.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineAngle {
    radians: f64,
    sine: f64,
    cosine: f64,
}

impl LineAngle {
    /// Validate the requested radians and store its trigonometric constants.
    /// Example: `LineAngle::new(std::f64::consts::FRAC_PI_4)?`.
    pub fn new(radians: f64) -> Result<Self, InvalidLineAngle> {
        if !(radians > 0. && radians < PI) {
            return Err(InvalidLineAngle { radians });
        }
        let (sine, cosine) = martensite_cmath::sincos(radians);
        Ok(Self {
            radians,
            sine,
            cosine,
        })
    }

    /// Return the original requested radians. Example: `angle.radians()`.
    pub const fn radians(self) -> f64 {
        self.radians
    }

    pub(super) const fn sin_cos(self) -> (f64, f64) {
        (self.sine, self.cosine)
    }
}

/// A nonfinite angle or an angle outside the supported open interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvalidLineAngle {
    /// Offending authored value in radians.
    pub radians: f64,
}

impl fmt::Display for InvalidLineAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "line angle {} radians: expected a finite value strictly between zero and pi",
            self.radians
        )
    }
}
impl Error for InvalidLineAngle {}
