// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::MetricBezier;

fn query(curve: &[[f64; 3]]) -> ConicalTrimQuery<'_> {
    ConicalTrimQuery {
        curve: MetricBezier {
            poles: curve,
            weights: None,
        },
        pcurve: &[[0., 0.], [0., 0.]],
        carrier: ConicalTrimCarrier {
            radius: 2.,
            axial_origin: 3.,
            semi_angle: 0.5,
        },
        spatial_limit: 0.,
        options: Default::default(),
    }
}

#[test]
fn constant_reference_circle_point_is_exact_for_every_admitted_angle() {
    for angle in [0.01, 0.5, 1.5] {
        let mut input = query(&[[2., 0., 3.]; 2]);
        input.carrier.semi_angle = angle;
        let proof = certify_conical_trim(input, || false).unwrap();
        assert_eq!(proof.residual_bounds(), [0.; 3]);
        assert_eq!(proof.requested_spatial_limit(), 0.);
        assert_eq!(proof.requested_options(), input.options);
        assert!(proof.matches_source(input));
        assert!(proof.operations() > 0);
    }
}

#[test]
fn independent_generator_values_and_deliberate_offset_are_bounded() {
    // Independently rounded sin(1/2), cos(1/2); no production evaluator builds these poles.
    let curve = [[2., 0., 3.], [2.479425538604203, 0., 3.8775825618903728]];
    let mut input = query(&curve);
    input.pcurve = &[[0., 0.], [0., 1.]];
    input.spatial_limit = 1e-14;
    let proof = certify_conical_trim(input, || false).unwrap();
    assert!(proof.residual_bounds()[2] < 1e-14);
    let displaced = curve.map(|[x, y, z]| [x + 0.125, y, z + 0.25]);
    input.curve.poles = &displaced;
    input.spatial_limit = 0.376;
    let shifted = certify_conical_trim(input, || false).unwrap();
    assert!(shifted.residual_bounds()[0] >= 0.125);
    assert!(shifted.residual_bounds()[1] >= 0.25);
    input.spatial_limit = 0.374;
    assert!(matches!(
        certify_conical_trim(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::Uncertified { .. }
    ));
}

#[test]
fn carrier_bits_are_owned_source_identity() {
    let input = query(&[[2., 0., 3.]; 2]);
    let proof = certify_conical_trim(input, || false).unwrap();
    let mut changed = [input.carrier; 3];
    changed[0].radius = 4.;
    changed[1].axial_origin = 4.;
    changed[2].semi_angle = 0.6;
    for carrier in changed {
        assert!(!proof.matches_source(ConicalTrimQuery { carrier, ..input }));
    }
}

#[test]
fn uv_bits_are_owned_source_identity() {
    let input = query(&[[2., 0., 3.]; 2]);
    let proof = certify_conical_trim(input, || false).unwrap();
    let uv = ConicalTrimQuery {
        pcurve: &[[0., 0.], [0., 1.]],
        ..input
    };
    assert!(!proof.matches_source(uv));
}

#[test]
fn curve_and_weight_bits_are_owned_source_identity() {
    let input = query(&[[2., 0., 3.]; 2]);
    let proof = certify_conical_trim(input, || false).unwrap();
    let weighted = MetricBezier {
        weights: Some(&[1., 2.]),
        ..input.curve
    };
    assert!(!proof.matches_source(ConicalTrimQuery {
        curve: weighted,
        ..input
    }));
    let changed = MetricBezier {
        poles: &[[2., 0., 3.], [2., 0., 4.]],
        ..input.curve
    };
    assert!(!proof.matches_source(ConicalTrimQuery {
        curve: changed,
        ..input
    }));
}

#[test]
fn exhausted_and_cancelled_traces_never_publish() {
    let mut input = query(&[[2., 0., 3.]; 2]);
    input.options.max_operations = 0;
    let exhausted = certify_conical_trim(input, || false).unwrap_err();
    assert!(matches!(
        exhausted.cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
    input.options = Default::default();
    let cancelled = certify_conical_trim(input, || true).unwrap_err();
    assert!(matches!(
        cancelled.cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ConicalTrimCarrier>();
    send_sync::<CertifiedConicalTrim>();
    send_sync::<ConicalTrimError>();
    send_sync::<ConicalTrimQuery<'static>>();
}

#[test]
fn nonpositive_radius_is_refused_before_any_residual_acceptance() {
    let mut input = query(&[[2., 0., 3.]; 2]);
    input.pcurve = &[[0., -8.], [0., -8.]];
    let error = certify_conical_trim(input, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalProfileFailure::DirectionUnresolved { .. }
    ));
}

#[test]
fn invalid_angles_report_the_offending_semi_angle_on_an_otherwise_valid_trace() {
    let mut input = query(&[[2., 0., 3.]; 2]);
    for angle in [0., -0.5, f64::NAN, std::f64::consts::FRAC_PI_2] {
        input.carrier.semi_angle = angle;
        let error = certify_conical_trim(input, || false).unwrap_err();
        let CylindricalProfileFailure::Value { role, index, value } = error.cause else {
            panic!(
                "expected semi-angle admission failure, got {:?}",
                error.cause
            );
        };
        assert!(role.contains("cone semi-angle"));
        assert_eq!(index, 0);
        assert_eq!(value.to_bits(), angle.to_bits());
    }
}

#[test]
fn curved_quarter_circle_pays_same_parameter_error_between_endpoints() {
    let curve = [[2., 0., 3.], [2., 2., 3.], [0., 2., 3.]];
    let weights = [1., std::f64::consts::FRAC_1_SQRT_2, 1.];
    let mut input = query(&curve);
    input.curve.weights = Some(&weights);
    input.pcurve = &[[0., 0.], [std::f64::consts::FRAC_PI_2, 0.]];
    input.spatial_limit = 0.2;
    let proof = certify_conical_trim(input, || false).unwrap();
    assert!(proof.residual_bounds()[2] >= 0.03);
    assert!(proof.residual_bounds()[2] <= input.spatial_limit);
    input.spatial_limit = 0.001;
    assert!(certify_conical_trim(input, || false).is_err());
}

#[path = "tests/requested_bounds.rs"]
mod requested_bounds;

#[test]
fn cone_projected_degree_five_chart_and_space_parent_have_distinct_ranges() {
    use crate::predicates::roots::metric_bounds::MetricBezier;
    let semi_angle = std::f64::consts::FRAC_PI_6;
    let carrier = ConicalTrimCarrier {
        radius: 2.,
        axial_origin: 3.,
        semi_angle,
    };
    let points = [[2., 0., 3.], [2.5, 0., 3. + 3f64.sqrt() / 2.]];
    let uv = [0.25, 0.35, 0.45, 0.55, 0.65, 0.75].map(|v| [0., v]);
    let query = ConicalTrimQuery {
        curve: MetricBezier {
            poles: &points,
            weights: None,
        },
        pcurve: &uv,
        carrier,
        spatial_limit: 1e-12,
        options: Default::default(),
    };
    let ranges = [[0.25, 0.75], [0., 1.]];
    let proof = certify_conical_trim_ranges(query, ranges, || false).unwrap();
    assert!(proof.residual_bounds()[2] < 1e-12);
    assert!(proof.matches_ranges(query, ranges));
    assert!(!proof.matches_source(query));
    assert!(certify_conical_trim(query, || false).is_err());
}
