// SPDX-License-Identifier: GPL-2.0-only
//! Share the certified circular constant and trigonometric enclosures for developed charts.
use super::{
    BoundedArithmeticFailure, CylindricalProfileFailure, cylindrical_profile::trigonometric,
};
use crate::predicates::roots::certificate_arithmetic::{
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
    circular_constants,
};
use std::f64::consts::FRAC_PI_2;

const POLICY: ArithmeticPolicy = ArithmeticPolicy {
    fractional_bits: 128,
    max_operations: 10_000,
};

/// Physical-to-developed angular scale of a cylinder or cone.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DevelopedScale {
    /// Positive radius in model units.
    CylinderRadius(f64),
    /// Semi-angle of a nondegenerate half-cone, in radians.
    ConeSemiAngle(f64),
}

/// Outward finite binary64 enclosures for a developed chart.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DevelopedPeriod {
    /// Developed arc units per radian of physical sweep.
    pub scale: [f64; 2],
    /// Developed displacement of one physical revolution.
    pub period: [f64; 2],
}

/// Failure to obtain a bounded developed period.
#[derive(Clone, Debug)]
pub enum DevelopedPeriodError {
    /// The supplied support scale is outside its domain.
    InvalidScale(DevelopedScale),
    /// The bounded arithmetic proof could not complete.
    Arithmetic(BoundedArithmeticFailure),
    /// The conical scale's existing trigonometric proof could not complete.
    Trigonometry(Box<CylindricalProfileFailure>),
}

/// Enclose one full turn without treating rounded π or sine as exact.
/// Example: `let period = certify_developed_period(DevelopedScale::CylinderRadius(5.0))?;`.
pub fn certify_developed_period(
    input: DevelopedScale,
) -> Result<DevelopedPeriod, DevelopedPeriodError> {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(POLICY, &mut cancelled);
    let scale = enclose_scale(input, &mut arithmetic)?;
    let pi = circular_constants::pi(&mut arithmetic).map_err(DevelopedPeriodError::Arithmetic)?;
    let tau = arithmetic
        .fraction(&pi, 2, 1)
        .map_err(DevelopedPeriodError::Arithmetic)?;
    let period = arithmetic
        .multiply(&tau, &scale)
        .map_err(DevelopedPeriodError::Arithmetic)?;
    let scale = endpoints(&arithmetic, scale).map_err(DevelopedPeriodError::Arithmetic)?;
    let period = endpoints(&arithmetic, period).map_err(DevelopedPeriodError::Arithmetic)?;
    Ok(DevelopedPeriod { scale, period })
}

fn enclose_scale(
    input: DevelopedScale,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, DevelopedPeriodError> {
    match input {
        DevelopedScale::CylinderRadius(r) if r.is_finite() && r > 0. => arithmetic
            .scalar(r)
            .map_err(DevelopedPeriodError::Arithmetic),
        DevelopedScale::ConeSemiAngle(a) if a > 0. && a < FRAC_PI_2 => {
            let phase = arithmetic
                .scalar(a)
                .map_err(DevelopedPeriodError::Arithmetic)?;
            let [_, sine] = trigonometric::scalar(&phase, arithmetic)
                .map_err(|cause| DevelopedPeriodError::Trigonometry(Box::new(cause)))?;
            Ok(sine)
        }
        _ => Err(DevelopedPeriodError::InvalidScale(input)),
    }
}

fn endpoints(
    arithmetic: &Arithmetic<'_>,
    value: Interval,
) -> Result<[f64; 2], BoundedArithmeticFailure> {
    Ok([
        -arithmetic.upper_f64(-value.lower)?,
        arithmetic.upper_f64(value.upper)?,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, TAU};

    #[test]
    fn cylindrical_period_encloses_real_tau_and_retains_an_exact_radius() {
        let answer = certify_developed_period(DevelopedScale::CylinderRadius(1.)).unwrap();
        assert_eq!(answer.scale, [1., 1.]);
        assert_eq!(answer.period, [TAU, TAU.next_up()]);
    }

    #[test]
    fn cone_uses_sine_of_the_supplied_angle_and_an_outward_circular_constant() {
        // A direct Taylor evaluation of this exact dyadic argument gives
        // 0.5000000000000000464225... for the sine.
        let angle = f64::from_bits(0x3fe0_c152_382d_7366);
        let answer = certify_developed_period(DevelopedScale::ConeSemiAngle(angle)).unwrap();
        assert_eq!(answer.scale, [0.5, 0.5_f64.next_up()]);
        assert!(PI - 2e-15 < answer.period[0] && answer.period[1] < PI + 2e-15);
        assert!(answer.period[0] < answer.period[1]);
    }

    #[test]
    fn nonpositive_or_degenerate_support_is_rejected() {
        for input in [
            DevelopedScale::CylinderRadius(0.),
            DevelopedScale::ConeSemiAngle(FRAC_PI_2),
        ] {
            assert!(matches!(
                certify_developed_period(input),
                Err(DevelopedPeriodError::InvalidScale(_))
            ));
        }
    }
}
