// SPDX-License-Identifier: GPL-2.0-only
//! Rule out different whole-turn lifts of the same retained XY direction.
use super::*;
/// No lift correspondence accompanies a failed source or bounded-arithmetic check.
#[derive(Clone, Debug)]
pub struct CylindricalLiftError {
    /// Requested arithmetic settings, without clamps.
    pub requested_options: CylindricalProfileOptions,
    /// Original source, arithmetic or angular-ambiguity failure.
    pub cause: CylindricalProfileFailure,
}
impl std::fmt::Display for CylindricalLiftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "angular lift options={:?}, cause={:?}; expected identical native references and total angular ambiguity < 6 radians",
            self.requested_options, self.cause
        )
    }
}
/// Source-bound proof that two continuous profile charts choose the same angular lift.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalLift {
    sources: [CertifiedCylindricalProfile; 2],
    ambiguity: f64,
    options: CylindricalProfileOptions,
    operations: u32,
}
impl CertifiedCylindricalLift {
    /// Outward endpoint ambiguity, strictly below six radians; `proof.ambiguity_bound()`.
    pub fn ambiguity_bound(&self) -> f64 {
        self.ambiguity
    }
    /// Actual requested precision and work allowance; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalProfileOptions {
        self.options
    }
    /// Charged interval operations; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Compare both complete retained profile sources; `proof.matches_sources([&a, &b])`.
    pub fn matches_sources(&self, sources: [&CertifiedCylindricalProfile; 2]) -> bool {
        self.sources
            .iter()
            .zip(sources)
            .all(|(a, b)| a.same_source(b))
    }
}
/// Certify one common lift using endpoint chart separation plus both angular error bounds.
/// Continuity follows from the source profile certificates; six is strictly below `2*pi`.
/// Example: `let lift = certify_cylindrical_lift([&developed, &angular], options, || false)?;`.
pub fn certify_cylindrical_lift(
    sources: [&CertifiedCylindricalProfile; 2],
    options: CylindricalProfileOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalLift, CylindricalLiftError> {
    execute(sources, options, &mut cancelled).map_err(|cause| CylindricalLiftError {
        requested_options: options,
        cause,
    })
}
fn execute(
    sources: [&CertifiedCylindricalProfile; 2],
    options: CylindricalProfileOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalLift> {
    admission::policy(options)?;
    require_reference(sources)?;
    let mut arithmetic = Arithmetic::new(arithmetic_policy(options), cancelled);
    let ambiguity = ambiguity(sources, &mut arithmetic)?;
    require_unambiguous(ambiguity)?;
    Ok(CertifiedCylindricalLift {
        sources: sources.map(Clone::clone),
        ambiguity,
        options,
        operations: arithmetic.operations(),
    })
}
fn require_reference(sources: [&CertifiedCylindricalProfile; 2]) -> Checked<()> {
    if !sources[0].matches_reference_curve(sources[1]) {
        return Err(CylindricalProfileFailure::Value {
            role: "native reference match",
            index: 0,
            value: 0.,
        });
    }
    Ok(())
}
fn arithmetic_policy(options: CylindricalProfileOptions) -> ArithmeticPolicy {
    ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    }
}
fn require_unambiguous(bound: f64) -> Checked<()> {
    if bound >= 6. {
        return Err(CylindricalProfileFailure::Uncertified {
            bound,
            requested: 6.,
        });
    }
    Ok(())
}

fn ambiguity(
    sources: [&CertifiedCylindricalProfile; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    let a = sources[0].authored_angle_start(arithmetic)?;
    let b = sources[1].authored_angle_start(arithmetic)?;
    let difference = arithmetic.subtract(&a, &b)?;
    let magnitude = arithmetic.upper_f64(difference.magnitude())?;
    let mut total = arithmetic.scalar(magnitude)?;
    for source in sources {
        let error = arithmetic.scalar(source.angular_position_bound())?;
        total = arithmetic.add(&total, &error)?;
    }
    Ok(arithmetic.upper_f64(total.upper)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn profile(angle: f64, height: f64) -> CertifiedCylindricalProfile {
        let authored = [[angle, 3.], [angle, 4.]];
        let poles = [[1., 0., 3. + height], [1., 0., 4. + height]];
        let input = CylindricalProfileQuery {
            authored: &authored,
            stored: super::super::super::MetricBezier {
                poles: &poles,
                weights: None,
            },
            interval: CylindricalProfileInterval::new(0, 0).unwrap(),
            support_radius: 1.,
            stored_radius: 1.,
            angular_origin: 0.,
            spatial_limit: 1e-5,
            options: Default::default(),
        };
        certify_cylindrical_profile(input, || false).unwrap()
    }
    #[test]
    fn identical_lifts_retain_sources_policy_and_work() {
        let a = profile(0., 0.);
        let b = profile(0., 0.);
        let proof = certify_cylindrical_lift([&a, &b], Default::default(), || false).unwrap();
        assert_eq!(proof.ambiguity_bound(), 0.);
        assert_eq!(
            proof.requested_options(),
            CylindricalProfileOptions::default()
        );
        assert!(proof.operations() > 0 && proof.matches_sources([&a, &b]));
        assert!(!proof.matches_sources([&a, &profile(0., 1e-7)]));
    }
    #[test]
    fn whole_turn_alias_is_refused_despite_identical_native_reference() {
        let a = profile(0., 0.);
        let b = profile(std::f64::consts::TAU, 0.);
        assert!(a.matches_reference_curve(&b));
        let error = certify_cylindrical_lift([&a, &b], Default::default(), || false).unwrap_err();
        assert!(
            matches!(error.cause, CylindricalProfileFailure::Uncertified { bound, requested: 6. } if bound > 6.)
        );
    }
    #[test]
    fn foreign_native_reference_and_cancelled_arithmetic_return_no_lift() {
        let a = profile(0., 0.);
        let b = profile(0., 1e-7);
        assert!(certify_cylindrical_lift([&a, &b], Default::default(), || false).is_err());
        let error = certify_cylindrical_lift([&a, &a], Default::default(), || true).unwrap_err();
        assert!(matches!(
            error.cause,
            CylindricalProfileFailure::Arithmetic(_)
        ));
    }
}
