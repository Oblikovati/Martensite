// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const INNER: [[f64; 3]; 2] = [[1., 0., 3.], [1., 0., 4.]];
const OUTER: [[f64; 3]; 2] = [[2., 0., 3.], [2., 0., 4.]];

fn query<'a>(a: &'a [[f64; 3]], b: &'a [[f64; 3]]) -> CylindricalStorageQuery<'a> {
    CylindricalStorageQuery {
        curves: [
            MetricBezier {
                poles: a,
                weights: None,
            },
            MetricBezier {
                poles: b,
                weights: None,
            },
        ],
        radii: [1., 2.],
        limits: CylindricalStorageLimits::uniform(1e-12),
        options: Default::default(),
    }
}

#[test]
fn exact_generator_proves_zero_and_retains_request() {
    let input = query(&INNER, &OUTER);
    let report = certify_cylindrical_storage(input, || false).unwrap();
    assert_eq!(report.radial_distance_bounds(), [0., 0.]);
    assert_eq!(
        (
            report.scaled_ruling_bound(),
            report.height_difference_bound()
        ),
        (0., 0.)
    );
    assert_eq!(report.weight_lower_bounds(), [1., 1.]);
    assert_eq!(report.requested_options(), input.options);
    assert_eq!(report.requested_limits(), input.limits);
    assert!(report.operations() > 0);
}

#[test]
fn certificate_source_identity_includes_radii_and_signed_zero() {
    let input = query(&INNER, &OUTER);
    let report = certify_cylindrical_storage(input, || false).unwrap();
    assert!(report.matches_source(input.curves, input.radii));
    assert!(!report.matches_source(input.curves, [1., 3.]));
    let mut changed = INNER;
    changed[0][1] = -0.;
    assert!(!report.matches_source(query(&changed, &OUTER).curves, input.radii));
}

#[test]
fn rounded_rational_circle_has_a_certified_storage_bound() {
    // t=u/2 gives X=1-t², Y=2t, W=1+t², degree two.
    let a = [[1., 0., 0.], [1., 0.5, 0.], [0.6, 0.8, 0.]];
    let b = a.map(|p| [2. * p[0], 2. * p[1], 0.]);
    let weights = [1., 1., 1.25];
    let mut query = query(&a, &b);
    query
        .curves
        .iter_mut()
        .for_each(|curve| curve.weights = Some(&weights));
    let report = certify_cylindrical_storage(query, || false).unwrap();
    assert!(report.radial_distance_bounds().iter().all(|b| *b <= 1e-14));
    assert!(report.scaled_ruling_bound() < 1e-34);
    assert_eq!(report.height_difference_bound(), 0.);
}

#[test]
fn height_drift_is_not_certified() {
    let shifted = OUTER.map(|p| [p[0], p[1], p[2] + 0.125]);
    let error = certify_cylindrical_storage(query(&INNER, &shifted), || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalStorageFailure::Uncertified {
            residual: CylindricalStorageResidual::HeightDrift,
            bound: 0.125,
            ..
        }
    ));
}

#[test]
fn rotated_rulings_are_not_certified() {
    let rotated = OUTER.map(|p| [0., p[0], p[2]]);
    let error = certify_cylindrical_storage(query(&INNER, &rotated), || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalStorageFailure::Uncertified {
            residual: CylindricalStorageResidual::ScaledRuling,
            bound: 4.,
            ..
        }
    ));
}

#[test]
fn zero_work_budget_refuses_without_partial_reports() {
    let mut input = query(&INNER, &OUTER);
    input.options.max_operations = 0;
    let error = certify_cylindrical_storage(input, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalStorageFailure::WorkLimit {
            completed: 0,
            limit: 0
        }
    ));
    assert_eq!(error.requested_options, input.options);
}

#[test]
fn invalid_policy_and_early_cancellation_are_explicit() {
    let mut input = query(&INNER, &OUTER);
    assert_eq!(
        certify_cylindrical_storage(input, || true)
            .unwrap_err()
            .cause,
        CylindricalStorageFailure::Cancelled
    );
    input.options.fractional_bits = 7;
    assert!(matches!(
        certify_cylindrical_storage(input, || false)
            .unwrap_err()
            .cause,
        CylindricalStorageFailure::Policy(_)
    ));
}

#[test]
fn nonpositive_retained_weights_are_explicit() {
    let weights = [1., 0.];
    let mut input = query(&INNER, &OUTER);
    input.curves[0].weights = Some(&weights);
    assert!(matches!(
        certify_cylindrical_storage(input, || false)
            .unwrap_err()
            .cause,
        CylindricalStorageFailure::Value {
            field: "weight",
            depth: Some(0),
            ..
        }
    ));
}

#[test]
fn missing_retained_weights_are_explicit() {
    let mut input = query(&INNER, &OUTER);
    input.curves[0].weights = Some(&[]);
    assert!(matches!(
        certify_cylindrical_storage(input, || false)
            .unwrap_err()
            .cause,
        CylindricalStorageFailure::PoleCount { .. }
    ));
}

#[test]
fn insufficient_precision_cannot_turn_a_positive_denominator_into_a_certificate() {
    let mut input = query(&INNER, &OUTER);
    input.radii[0] = f64::from_bits(1);
    let cause = certify_cylindrical_storage(input, || false)
        .unwrap_err()
        .cause;
    let CylindricalStorageFailure::DenominatorUnresolved { lower } = cause else {
        panic!("unexpected cause: {cause:?}");
    };
    assert_eq!(lower.compare_f64(0.).unwrap(), std::cmp::Ordering::Equal);
}

#[test]
fn explicit_precision_resolves_a_subnormal_denominator() {
    let radius = f64::from_bits(1);
    let poles = [[radius, 0., 3.], [radius, 0., 4.]];
    let mut input = query(&poles, &poles);
    input.radii = [radius; 2];
    input.options.fractional_bits = 4096;
    input.limits = CylindricalStorageLimits::uniform(0.);
    let report = certify_cylindrical_storage(input, || false).unwrap();
    assert_eq!(report.radial_distance_bounds(), [0.; 2]);
    assert_eq!(report.normal_sweep_error_bound(), 0.);
}

#[test]
fn interior_radial_bulge_refuses_despite_exact_endpoint_incidence() {
    let bulge = [[1., 0., 3.], [1.25, 0., 3.5], [1., 0., 4.]];
    let error = certify_cylindrical_storage(query(&bulge, &OUTER), || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalStorageFailure::Uncertified {
            residual: CylindricalStorageResidual::FirstRadius,
            ..
        }
    ));
    // At u=1/2 the exact polynomial has radius 1.125, so it cannot meet 1e-12.
    assert!(error.to_string().contains("requested"));
}

#[test]
fn differing_stored_degrees_keep_the_complete_common_parameter_domain() {
    let elevated = [[2., 0., 3.], [2., 0., 3.5], [2., 0., 4.]];
    let report = certify_cylindrical_storage(query(&INNER, &elevated), || false).unwrap();
    assert!(report.scaled_ruling_bound() < 1e-30);
    assert!(report.height_difference_bound() < 1e-30);
}

#[test]
fn nonzero_bounds_match_closed_form_with_a_nonunit_first_radius() {
    let a = [[2., 0., 3.], [2., 0., 4.]];
    let b = [[4.5, 0., 3.125], [4.5, 0., 4.125]];
    let mut input = query(&a, &b);
    input.radii = [2., 4.];
    input.limits = CylindricalStorageLimits::uniform(2.);
    let report = certify_cylindrical_storage(input, || false).unwrap();
    // The radial enclosure is (4.5² - 4²)/4; the actual distance is 0.5.
    assert_eq!(report.radial_distance_bounds(), [0., 1.0625]);
    assert_eq!(report.scaled_ruling_bound(), 0.5);
    assert_eq!(report.height_difference_bound(), 0.125);
    assert_eq!(report.normal_sweep_error_bound(), 0.625);
}

#[test]
fn mismatched_polynomial_counts_cannot_silently_truncate_a_residual() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let a = vec![arithmetic.scalar(1.).unwrap()];
    let b = vec![arithmetic.zero(); 2];
    let expected = CylindricalStorageFailure::CoefficientCount { left: 1, right: 2 };
    assert_eq!(
        polynomial::sum(&a, &b, &mut arithmetic)
            .map_err(CylindricalStorageFailure::from)
            .unwrap_err(),
        expected
    );
    assert_eq!(
        polynomial::difference(&a, &b, &mut arithmetic)
            .map_err(CylindricalStorageFailure::from)
            .unwrap_err(),
        expected
    );
}

#[test]
fn combined_normal_sweep_budget_refuses_individually_small_residuals() {
    let b = [[2.0625, 0., 3.125], [2.0625, 0., 4.125]];
    let mut input = query(&INNER, &b);
    input.limits = CylindricalStorageLimits::uniform(0.15);
    let error = certify_cylindrical_storage(input, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalStorageFailure::Uncertified {
            residual: CylindricalStorageResidual::NormalSweep,
            bound: 0.1875,
            ..
        }
    ));
}

#[test]
fn reference_curve_on_the_axis_cannot_define_a_normal_sweep() {
    let axis = [[0., 0., 3.], [0., 0., 4.]];
    let mut input = query(&axis, &OUTER);
    input.limits = CylindricalStorageLimits::uniform(10.);
    let error = certify_cylindrical_storage(input, || false).unwrap_err();
    assert_eq!(
        error.cause,
        CylindricalStorageFailure::ReferenceDirectionUnresolved {
            radial_bound: 1.,
            radius: 1.,
        }
    );
}

#[test]
fn work_limit_applies_after_partial_arithmetic() {
    let mut input = query(&INNER, &OUTER);
    input.options.max_operations = 17;
    let error = certify_cylindrical_storage(input, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalStorageFailure::WorkLimit {
            completed: 17,
            limit: 17
        }
    ));
}

#[test]
fn cancellation_applies_after_partial_arithmetic() {
    let mut polls = 0;
    let error = certify_cylindrical_storage(query(&INNER, &OUTER), || {
        polls += 1;
        polls > 10
    })
    .unwrap_err();
    assert_eq!(error.cause, CylindricalStorageFailure::Cancelled);
    assert_eq!(polls, 11);
}

#[test]
fn certificate_and_request_types_are_send_sync() {
    fn require<T: Send + Sync>() {}
    require::<CertifiedCylindricalStorage>();
    require::<CylindricalStorageQuery<'static>>();
    require::<CylindricalStorageOptions>();
    require::<CylindricalStorageLimits>();
    require::<CylindricalStorageError>();
    require::<CylindricalStorageFailure>();
    require::<CylindricalStorageResidual>();
}

#[test]
fn unsupported_stored_degree_is_not_reduced_before_admission() {
    let poles = vec![INNER[0]; 27];
    let error = certify_cylindrical_storage(query(&poles, &OUTER), || false).unwrap_err();
    assert_eq!(
        error.cause,
        CylindricalStorageFailure::Degree {
            depth: 0,
            actual: 26,
            limit: 25
        }
    );
}

#[test]
fn nonfinite_stored_coordinates_retain_the_offending_depth_and_axis() {
    let mut poles = OUTER;
    poles[0][2] = f64::NAN;
    let error = certify_cylindrical_storage(query(&INNER, &poles), || false).unwrap_err();
    assert!(matches!(error.cause, CylindricalStorageFailure::Value {
        depth: Some(1), field: "pole z", index: 0, value,
    } if value.is_nan()));
}
