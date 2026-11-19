// SPDX-License-Identifier: GPL-2.0-only
//! Explicit V5 keeps original policy, actual loans and issued sources independently observable.
use super::*;
mod chunks;
mod geometric;
mod mutations;
fn version(cap: u64) -> PlanarProofVersion {
    PlanarProofVersion::FilteredCompositeBernsteinV5 {
        max_components: 64,
        max_total_subintervals: cap,
    }
}
fn profile() -> SketchProfile {
    let points = [[0., 0.], [2., 0.], [2., 1.], [0., 1.]];
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: points
            .iter()
            .enumerate()
            .map(|(id, point)| ProfileVertex {
                id: ProfileVertexId(id as u64),
                point: *point,
            })
            .collect(),
        curves: (0..4)
            .map(|id| ProfileCurve::Line {
                id: ProfileCurveId(id),
                start: ProfileVertexId(id),
                end: ProfileVertexId((id + 1) % 4),
            })
            .collect(),
    }
}
fn resolution() -> Resolution {
    Resolution::new(
        1e-9,
        1e-8,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}
fn build(
    source: &SketchProfile,
    requested: PlanarProofVersion,
    allowance: u64,
) -> PlanarRegionWorkResult {
    build_planar_regions_with_integration_budget_and_cancel(
        source,
        resolution(),
        PlanarProofWorkLimit::CurrentDefault,
        requested,
        allowance,
        || false,
    )
}
#[test]
fn explicit_program_retains_requested_and_minimum_actual_loan() {
    let source = profile();
    let result = build(&source, version(1024), 17).unwrap();
    assert_eq!(result.requested_proof_version(), version(1024));
    assert_eq!(result.proof_version(), version(17));
    let proof = result.integral_program_evidence().unwrap();
    assert_eq!(
        (proof.entry_allowance(), proof.granted_subintervals()),
        (17, 17)
    );
    assert!(proof.matches_profile(&source));
    assert!(result.regions()[0].area().encloses(2.));
    assert!(proof.integration_subintervals() <= 17);
}
#[test]
fn nonprogram_budget_entry_refuses_before_late_cancel_without_fabricated_entities() {
    for actual in [
        PlanarProofVersion::CarrierV2,
        PlanarProofVersion::BernsteinV3,
        PlanarProofVersion::FilteredBernsteinV4,
    ] {
        assert_nonprogram_refusal(actual);
    }
}
#[test]
fn default_and_all_old_versions_retain_no_program_authority() {
    for version in [
        PlanarProofVersion::CarrierV2,
        PlanarProofVersion::BernsteinV3,
        PlanarProofVersion::FilteredBernsteinV4,
    ] {
        let result = build_planar_regions_with_proof_and_cancel(
            &profile(),
            resolution(),
            PlanarProofWorkLimit::CurrentDefault,
            version,
            || false,
        )
        .unwrap();
        assert_eq!(result.requested_proof_version(), version);
        assert_eq!(result.proof_version(), version);
        assert!(result.integral_program_evidence().is_none());
    }
}
#[test]
fn zero_program_allowance_is_honest_and_never_widened() {
    let result = build(&profile(), version(1024), 0);
    assert_zero_loan(result);
}
fn assert_zero_loan(result: PlanarRegionWorkResult) {
    let counts = match result {
        Ok(result) => {
            let proof = result.integral_program_evidence().unwrap();
            (
                proof.granted_subintervals(),
                proof.integration_subintervals(),
            )
        }
        Err(failure) => {
            let proof = failure.integral_program_failure().unwrap();
            (
                proof.granted_subintervals(),
                proof.integration_subintervals(),
            )
        }
    };
    assert_eq!(counts, (0, 0));
}

fn assert_nonprogram_refusal(actual: PlanarProofVersion) {
    let mut polls = 0;
    let failure = nonprogram_failure(actual, &mut polls);
    let policy = failure.integral_policy_failure().unwrap();
    assert_eq!(
        (
            policy.actual(),
            policy.entry_allowance(),
            policy.expected_request_version()
        ),
        (actual, 7, 5)
    );
    assert_eq!(policy.class(), DeclineClass::Unsupported);
    assert_eq!(failure.decline().class(), DeclineClass::Incomplete);
    assert_eq!(polls, 0);
    assert!(!failure.is_cancelled());
}

fn nonprogram_failure(actual: PlanarProofVersion, polls: &mut usize) -> PlanarRegionWorkFailure {
    build_planar_regions_with_integration_budget_and_cancel(
        &profile(),
        resolution(),
        PlanarProofWorkLimit::CurrentDefault,
        actual,
        7,
        || {
            *polls += 1;
            true
        },
    )
    .unwrap_err()
}
