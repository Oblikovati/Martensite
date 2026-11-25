// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::{MetricBezier, MetricCurve};
use super::*;

fn options() -> IntegralProgramOptions {
    IntegralProgramOptions {
        version: IntegralProgramVersion::V1,
        integration: RationalIntegralOptions::default(),
        max_components: 8,
        max_total_subintervals: 4096,
    }
}
fn occurrences(poles: &[[[f64; 2]; 2]]) -> Vec<OrientedMetricCurve<'_>> {
    poles
        .iter()
        .map(|poles| OrientedMetricCurve {
            curve: MetricCurve::Bezier(MetricBezier {
                poles,
                weights: None,
            }),
            reversed: false,
        })
        .collect()
}
fn rectangle() -> Vec<[[f64; 2]; 2]> {
    let corners = [[0., 0.], [1.25, 0.], [1.25, 1.5], [0., 1.5], [0., 0.]];
    let counts = [23, 21, 21, 21];
    let mut result = Vec::new();
    for (edge, count) in counts.into_iter().enumerate() {
        let points: Vec<_> = (0..=count)
            .map(|i| {
                let t = i as f64 / count as f64;
                std::array::from_fn(|axis| {
                    corners[edge][axis] + t * (corners[edge + 1][axis] - corners[edge][axis])
                })
            })
            .collect();
        result.extend(points.windows(2).map(|pair| [pair[0], pair[1]]));
    }
    result
}
fn boundary(
    parts: &[&[OrientedMetricCurve<'_>]],
    policy: IntegralProgramOptions,
) -> Result<CertifiedIntegralProgram, IntegralProgramError> {
    certify_boundary_integral_program(
        BoundaryIntegralProgramQuery {
            components: parts,
            options: policy,
        },
        || false,
    )
}
fn face(
    loops: &[IntegralProgramLoop<'_>],
) -> Result<CertifiedIntegralProgram, IntegralProgramError> {
    certify_face_integral_program(
        FaceIntegralProgramQuery {
            loops,
            options: options(),
        },
        || false,
    )
}
#[test]
fn eighty_six_rounded_noninteger_edges_keep_exact_axis_area_and_legacy_refusal() {
    let poles = rectangle();
    let curves = occurrences(&poles);
    let components = [&curves[..43], &curves[43..]];
    let proof = boundary(&components, options()).unwrap();
    assert_eq!(proof.enclosure(), [1.875, 1.875]);
    assert_eq!(proof.completed_occurrences(), 86);
    assert_eq!(proof.integration_subintervals(), 0);
    assert_eq!(proof.evidence(), RationalIntegralEvidence::ExactRational);
    assert!(proof.matches_boundary(BoundaryIntegralProgramQuery {
        components: &components,
        options: options()
    }));
    assert_legacy_count_refusal(&curves);
}
fn assert_legacy_count_refusal(curves: &[OrientedMetricCurve<'_>]) {
    let error =
        certify_rational_boundary_integral(curves, Default::default(), || false).unwrap_err();
    assert!(matches!(
        error.cause,
        RationalIntegralFailure::CurveCount {
            actual: 86,
            limit: 64
        }
    ));
}
#[test]
fn whole_face_loops_use_magnitudes_after_open_component_sum() {
    let outer = rectangle();
    let inner: Vec<_> = outer
        .iter()
        .map(|pair| pair.map(|p| p.map(|x| x * 0.5)))
        .collect();
    let outer_curves = occurrences(&outer);
    let mut inner_curves = occurrences(&inner);
    for curve in &mut inner_curves {
        curve.reversed = true;
    }
    let outer_parts = [&outer_curves[..43], &outer_curves[43..]];
    let inner_parts = [&inner_curves[..43], &inner_curves[43..]];
    assert_face_oracle(&outer_parts, &inner_parts);
}
fn assert_face_oracle(outer: &[&[OrientedMetricCurve<'_>]], inner: &[&[OrientedMetricCurve<'_>]]) {
    let loops = [
        IntegralProgramLoop { components: outer },
        IntegralProgramLoop { components: inner },
    ];
    let proof = face(&loops).unwrap();
    assert_eq!(proof.enclosure(), [1.40625, 1.40625]);
    assert_eq!(proof.completed_occurrences(), 172);
    assert!(proof.matches_face(FaceIntegralProgramQuery {
        loops: &loops,
        options: options()
    }));
}
#[test]
fn signed_open_components_cancel_without_independent_orientation() {
    let poles = [[[1., 0.], [1., 2.]]];
    let forward = occurrences(&poles);
    let mut reverse = forward.clone();
    reverse[0].reversed = true;
    let parts = [forward.as_slice(), reverse.as_slice()];
    assert_eq!(boundary(&parts, options()).unwrap().enclosure(), [0., 0.]);
    let loops = [IntegralProgramLoop { components: &parts }];
    let error = face(&loops).unwrap_err();
    assert!(
        matches!(error.cause, IntegralProgramFailure::Numerical(cause) if matches!(*cause, RationalIntegralFailure::IndeterminateOrientation { first_curve: 0 }))
    );
}
#[test]
fn source_policy_order_partition_and_role_bits_are_owned() {
    let poles = rectangle();
    let mut curves = occurrences(&poles);
    let parts = [&curves[..43], &curves[43..]];
    let proof = boundary(&parts, options()).unwrap();
    assert_partitions_and_policy(&proof, &curves);
    let loops = [IntegralProgramLoop { components: &parts }];
    assert!(!proof.matches_face(FaceIntegralProgramQuery {
        loops: &loops,
        options: options()
    }));
    curves[0].reversed = true;
    let changed = [&curves[..43], &curves[43..]];
    assert!(!proof.matches_boundary(BoundaryIntegralProgramQuery {
        components: &changed,
        options: options()
    }));
}
fn assert_partitions_and_policy(
    proof: &CertifiedIntegralProgram,
    curves: &[OrientedMetricCurve<'_>],
) {
    let repartition = [&curves[..42], &curves[42..]];
    assert!(!proof.matches_boundary(BoundaryIntegralProgramQuery {
        components: &repartition,
        options: options()
    }));
    let parts = [&curves[..43], &curves[43..]];
    let mut policy = options();
    policy.integration.absolute_error_limit *= 2.;
    assert!(!proof.matches_boundary(BoundaryIntegralProgramQuery {
        components: &parts,
        options: policy
    }));
}
#[test]
fn final_rounding_cap_is_global_and_never_rounded_per_component() {
    let poles = [
        [[0., 0.], [1., 0.]],
        [[1., 0.], [0., 1.]],
        [[0., 1.], [0., 0.]],
    ];
    let curves = occurrences(&poles);
    let parts = [&curves[..1], &curves[1..]];
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    let proof = boundary(&parts, policy).unwrap();
    assert_eq!(proof.enclosure(), [0.5, 0.5]);
    assert_eq!(proof.absolute_error_bound(), 0.);
    assert_eq!(proof.effective_options(), policy);
}
#[test]
fn unrepresentable_exact_sixth_refuses_zero_final_error_cap() {
    let poles = [[0., 0.], [0.5, 0.], [1., 1.]];
    let curves = [OrientedMetricCurve {
        curve: MetricCurve::Bezier(MetricBezier {
            poles: &poles,
            weights: None,
        }),
        reversed: false,
    }];
    let parts = [curves.as_slice()];
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    let error = boundary(&parts, policy).unwrap_err();
    assert_eq!(error.completed_occurrences, 1);
    assert!(
        matches!(error.cause, IntegralProgramFailure::Numerical(cause) if matches!(*cause, RationalIntegralFailure::ErrorLimit { requested: 0., .. }))
    );
}
#[test]
fn actual_source_mutation_and_exact_policy_sign_bits_reject_rebinding() {
    let mut poles = rectangle();
    let curves = occurrences(&poles);
    let parts = [&curves[..43], &curves[43..]];
    let proof = boundary(&parts, options()).unwrap();
    poles[0][1][0] += f64::EPSILON;
    let changed = occurrences(&poles);
    let parts = [&changed[..43], &changed[43..]];
    assert!(!proof.matches_boundary(BoundaryIntegralProgramQuery {
        components: &parts,
        options: options()
    }));
}
#[test]
fn program_public_receipts_are_send_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<CertifiedIntegralProgram>();
    shared::<IntegralProgramError>();
    shared::<IntegralProgramOptions>();
    shared::<IntegralProgramVersion>();
    shared::<IntegralProgramLoop<'static>>();
    shared::<IntegralProgramFailure>();
    shared::<BoundaryIntegralProgramQuery<'static>>();
    shared::<FaceIntegralProgramQuery<'static>>();
}
#[path = "tests/work.rs"]
mod work;

#[test]
fn indeterminate_whole_loop_refusal_does_not_accuse_the_last_component() {
    let poles = [[[1., 0.], [1., 2.]]];
    let forward = occurrences(&poles);
    let mut reverse = forward.clone();
    reverse[0].reversed = true;
    let parts = [forward.as_slice(), reverse.as_slice()];
    let loops = [IntegralProgramLoop { components: &parts }];
    let error = face(&loops).unwrap_err();
    assert_eq!(
        (
            error.loop_index,
            error.component_index,
            error.occurrence_index
        ),
        (Some(0), None, None)
    );
    assert_eq!(error.completed_occurrences, 2);
    assert!(
        matches!(error.cause, IntegralProgramFailure::Numerical(cause) if matches!(*cause, RationalIntegralFailure::IndeterminateOrientation { first_curve: 0 }))
    );
}

#[test]
fn closed_whole_loop_combines_opposite_sign_open_components_before_magnitude() {
    let poles = [
        [[1., 0.], [2., 0.]],
        [[2., 0.], [2., 1.]],
        [[2., 1.], [1., 1.]],
        [[1., 1.], [1., 0.]],
    ];
    let curves = occurrences(&poles);
    let parts = [&curves[..3], &curves[3..]];
    assert_eq!(
        boundary(&parts[..1], options()).unwrap().enclosure(),
        [1.5, 1.5]
    );
    assert_eq!(
        boundary(&parts[1..], options()).unwrap().enclosure(),
        [-0.5, -0.5]
    );
    let loops = [IntegralProgramLoop { components: &parts }];
    assert_eq!(face(&loops).unwrap().enclosure(), [1., 1.]);
}

#[test]
fn receipt_equality_retains_source_partition_policy_and_scalar_provenance() {
    let poles = rectangle();
    let curves = occurrences(&poles);
    let parts = [&curves[..43], &curves[43..]];
    let proof = boundary(&parts, options()).unwrap();
    assert_eq!(proof, boundary(&parts, options()).unwrap());
    let partition = [&curves[..42], &curves[42..]];
    assert_ne!(proof, boundary(&partition, options()).unwrap());
    let mut policy = options();
    policy.integration.absolute_error_limit *= 2.;
    assert_ne!(proof, boundary(&parts, policy).unwrap());
    assert_translated_receipt_distinct(&proof, &poles);
}
fn assert_translated_receipt_distinct(proof: &CertifiedIntegralProgram, poles: &[[[f64; 2]; 2]]) {
    let shifted: Vec<_> = poles
        .iter()
        .map(|pair| pair.map(|[x, y]| [x + 0.25, y]))
        .collect();
    let curves = occurrences(&shifted);
    let parts = [&curves[..43], &curves[43..]];
    let translated = boundary(&parts, options()).unwrap();
    assert_eq!(translated.enclosure(), proof.enclosure());
    assert_eq!(translated.evidence(), proof.evidence());
    assert_eq!(
        translated.requested_options().version,
        proof.requested_options().version
    );
    assert_ne!(&translated, proof);
}
#[test]
fn receipt_equality_preserves_exact_signed_zero_policy_binding() {
    let poles = [[[1., 0.], [1., 2.]]];
    let curves = occurrences(&poles);
    let parts = [curves.as_slice()];
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    let positive_zero = boundary(&parts, policy).unwrap();
    policy.integration.absolute_error_limit = -0.;
    let negative_zero = boundary(&parts, policy).unwrap();
    assert_eq!(positive_zero.enclosure(), negative_zero.enclosure());
    assert_ne!(positive_zero, negative_zero);
}

#[path = "tests/error_traits.rs"]
mod error_traits;
