// SPDX-License-Identifier: GPL-2.0-only
//! Evaluate source-lift ray signs before rounding any conical projection.
use super::*;
use crate::predicates::roots::metric_bounds::conical_profile::angle;
use std::f64::consts::FRAC_PI_4;

/// Outward transverse and radial projections on a certified developed cut ray.
#[derive(Clone, Debug)]
pub struct DevelopedConeRay {
    cosine: Interval,
    sine: Interval,
}

/// A positive-X developed angle or cut ray could not be enclosed.
#[derive(Clone, Debug)]
pub enum DevelopedConeRayError {
    /// The point is outside the strict positive-X wedge.
    Point([f64; 2]),
    /// Input angle bounds do not lie in the strict positive-X wedge.
    Phase([f64; 2]),
    /// Fixed binary arithmetic could not certify the requested projection.
    Arithmetic(BoundedArithmeticFailure),
    /// Principal developed angle evaluation could not complete.
    Angle {
        /// Literal source coordinate.
        point: [f64; 2],
        /// Exact angle solver diagnostic.
        reason: String,
    },
    /// Trigonometric bounds for the supplied angle enclosure could not complete.
    Trigonometry(Box<CylindricalProfileFailure>),
}

impl From<BoundedArithmeticFailure> for DevelopedConeRayError {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

/// Enclose the principal developed angle of an original point in the strict positive-X wedge.
/// Example: `let angle = enclose_developed_cone_angle([2.0, 0.125])?;`.
pub fn enclose_developed_cone_angle(point: [f64; 2]) -> Result<[f64; 2], DevelopedConeRayError> {
    admit(point)?;
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
    let bounded = angle::initial_angle(point, &mut arithmetic).map_err(|cause| {
        DevelopedConeRayError::Angle {
            point,
            reason: cause.to_string(),
        }
    })?;
    Ok(endpoints(&arithmetic, bounded)?)
}

/// Enclose the sine and cosine of an already bounded ray relative to the developed positive X axis.
/// Example: `let ray = certify_developed_cone_ray([0.0, 0.0])?;`.
pub fn certify_developed_cone_ray(
    phase: [f64; 2],
) -> Result<DevelopedConeRay, DevelopedConeRayError> {
    let [lower, upper] = phase;
    if !(-FRAC_PI_4 < lower && lower <= upper && upper < FRAC_PI_4) {
        return Err(DevelopedConeRayError::Phase(phase));
    }
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
    let phase = Interval {
        lower: arithmetic.scalar(lower)?.lower,
        upper: arithmetic.scalar(upper)?.upper,
    };
    let [cosine, sine] = trigonometric::scalar(&phase, &mut arithmetic)
        .map_err(|cause| DevelopedConeRayError::Trigonometry(Box::new(cause)))?;
    Ok(DevelopedConeRay { cosine, sine })
}

impl DevelopedConeRay {
    /// Enclose signed transverse distance and distance along the original cut ray.
    /// Example: `let [normal, radial] = ray.project([2.0, 0.125])?;`.
    pub fn project(&self, point: [f64; 2]) -> Result<[[f64; 2]; 2], DevelopedConeRayError> {
        admit(point)?;
        let mut cancelled = || false;
        let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
        let x = arithmetic.scalar(point[0])?;
        let y = arithmetic.scalar(point[1])?;
        let xc = arithmetic.multiply(&x, &self.cosine)?;
        let ys = arithmetic.multiply(&y, &self.sine)?;
        let xs = arithmetic.multiply(&x, &self.sine)?;
        let yc = arithmetic.multiply(&y, &self.cosine)?;
        let normal = arithmetic.subtract(&yc, &xs)?;
        let radial = arithmetic.add(&xc, &ys)?;
        Ok([
            endpoints(&arithmetic, normal)?,
            endpoints(&arithmetic, radial)?,
        ])
    }
}

fn admit([x, y]: [f64; 2]) -> Result<(), DevelopedConeRayError> {
    if x.is_finite() && x > y.abs() {
        return Ok(());
    }
    Err(DevelopedConeRayError::Point([x, y]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const _: fn() = || {
        fn assert_thread_safe<T: Send + Sync>() {}
        assert_thread_safe::<DevelopedConeRay>();
        assert_thread_safe::<DevelopedConeRayError>();
    };

    #[test]
    fn zero_ray_retains_exact_cartesian_projection() {
        let ray = certify_developed_cone_ray([0.; 2]).unwrap();
        assert_eq!(ray.project([2., 0.125]).unwrap(), [[0.125; 2], [2.; 2]]);
    }

    #[test]
    fn ray_angle_enclosure_keeps_exact_opposite_sides() {
        let phase = enclose_developed_cone_angle([2., 0.5]).unwrap();
        let ray = certify_developed_cone_ray(phase).unwrap();
        let [normal, radial] = ray.project([2., 0.5]).unwrap();
        assert!(normal[0] <= 0. && normal[1] >= 0.);
        let reference = 4.25_f64.sqrt();
        assert!(radial[0] <= reference && radial[1] >= reference);
        assert!(ray.project([2., 0.499]).unwrap()[0][1] < 0.);
        assert!(ray.project([2., 0.501]).unwrap()[0][0] > 0.);
    }

    #[test]
    fn cut_angle_outside_the_positive_x_wedge_is_refused() {
        assert!(matches!(
            certify_developed_cone_ray([-1., 0.]),
            Err(DevelopedConeRayError::Phase(_))
        ));
        assert!(matches!(
            enclose_developed_cone_angle([1., 1.]),
            Err(DevelopedConeRayError::Point(_))
        ));
    }
}
