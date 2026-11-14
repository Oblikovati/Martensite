// SPDX-License-Identifier: GPL-2.0-only
use super::*;
const UV: [[f64; 2]; 2] = [[0., 0.], [0., 0.]];
const A: [[f64; 3]; 2] = [[0., 0., 1.], [1., 0., 1.]];
const B: [[f64; 3]; 2] = [[0., 1., 1.], [1., 1., 1.]];
fn span<'a>(
    a: &'a [[f64; 3]],
    b: &'a [[f64; 3]],
    weights: Option<&'a [f64]>,
) -> CylindricalFluxSpan<'a> {
    CylindricalFluxSpan {
        boundaries: [
            MetricBezier { poles: a, weights },
            MetricBezier { poles: b, weights },
        ],
        cap_pcurves: [&UV, &UV],
        side_reversed: false,
    }
}
fn query<'a>(spans: &'a [CylindricalFluxSpan<'a>]) -> CylindricalFluxQuery<'a> {
    CylindricalFluxQuery {
        spans,
        radii: [1., 2.],
        options: Default::default(),
    }
}
#[test]
fn flat_unit_patch_has_exact_one_third_flux_without_integration_cells() {
    let spans = [span(&A, &B, None)];
    let proof = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    assert!(proof.enclosure()[0] <= 1. / 3. && proof.enclosure()[1] >= 1. / 3.);
    assert_eq!(proof.evidence(), RationalIntegralEvidence::ExactRational);
    assert_eq!(proof.integration_subintervals(), 0);
    assert!(proof.matches_source(query(&spans)));
    assert_eq!(proof.span_count(), 1);
}
#[test]
fn nonuniform_rational_parameterization_keeps_the_same_patch_flux() {
    let spans = [span(&A, &B, Some(&[1., 2.]))];
    let proof = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    assert!((proof.representative() - 1. / 3.).abs() <= proof.absolute_error_bound());
    assert!(proof.integration_subintervals() > 0);
    assert_eq!(
        proof.evidence(),
        RationalIntegralEvidence::ErrorBoundedIntegration
    );
}
#[test]
fn bilinear_patch_matches_independent_determinant_and_scale_alignment() {
    // S=(u+v,uv,1+v/2): det(S,Su,Sv)=u-v+uv/2; integral/3=1/24.
    let spans = [span(&A, &[[1., 0., 1.5], [2., 1., 1.5]], None)];
    let proof = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    assert!(proof.enclosure()[0] <= 1. / 24. && proof.enclosure()[1] >= 1. / 24.);
}
#[test]
fn explicit_side_reversal_negates_the_conditional_scalar() {
    let mut spans = [span(&A, &B, None)];
    let positive = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    spans[0].side_reversed = true;
    let negative = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    assert_eq!(
        negative.enclosure(),
        [-positive.enclosure()[1], -positive.enclosure()[0]]
    );
    assert!(!positive.matches_source(query(&spans)));
}
#[test]
fn aggregate_cell_budget_refuses_the_second_span_instead_of_resetting() {
    let spans = [span(&A, &B, Some(&[1., 2.])), span(&A, &B, Some(&[1., 2.]))];
    let full = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    let mut input = query(&spans);
    input.options.max_total_subintervals = full.integration_subintervals() as u32 - 1;
    let failure = certify_cylindrical_boundary_flux(input, || false).unwrap_err();
    assert!(matches!(
        failure.cause,
        CylindricalFluxFailure::Integration { span: Some(1), .. }
    ));
}

#[test]
fn both_cylindrical_cap_terms_use_their_actual_radii_and_directed_pcurves() {
    let mut spans = [span(&[[0.; 3]; 2], &[[0.; 3]; 2], None)];
    spans[0].cap_pcurves = [&[[1., 0.], [0., 1.]], &[[1., 0.], [0., 1.]]];
    let mut input = query(&spans);
    input.options.absolute_error_limit = 0.;
    let proof = certify_cylindrical_boundary_flux(input, || false).unwrap();
    assert_eq!(proof.enclosure(), [0.5, 0.5]);
    assert_eq!(proof.absolute_error_bound(), 0.);
    assert_eq!(proof.requested_options(), input.options);
}

#[test]
fn zero_cell_allowance_permits_exact_polynomial_work_and_refuses_rational_integration() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    input.options.max_total_subintervals = 0;
    assert_eq!(
        certify_cylindrical_boundary_flux(input, || false)
            .unwrap()
            .integration_subintervals(),
        0
    );
    let rational = [span(&A, &B, Some(&[1., 2.]))];
    input.spans = &rational;
    assert!(certify_cylindrical_boundary_flux(input, || false).is_err());
}

#[test]
fn source_binding_covers_pcurves_radii_weights_and_space_coefficients() {
    let spans = [span(&A, &B, None)];
    let proof = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    let mut changed = query(&spans);
    changed.radii = [1., 3.];
    assert!(!proof.matches_source(changed));
    let mut modified = spans;
    modified[0].cap_pcurves[1] = &[[1., 0.], [0., 1.]];
    assert!(!proof.matches_source(query(&modified)));
    modified = [span(&A, &B, Some(&[1., 1.]))];
    assert!(!proof.matches_source(query(&modified)));
    modified = [span(&A, &[[0., 1., 2.], [1., 1., 2.]], None)];
    assert!(!proof.matches_source(query(&modified)));
}

#[test]
fn unsupported_input_counts_weights_and_cancellation_return_no_scalar() {
    let mut spans = [span(&A, &B, None)];
    spans[0].boundaries[1].weights = Some(&[1., 2.]);
    assert!(matches!(
        certify_cylindrical_boundary_flux(query(&spans), || false)
            .unwrap_err()
            .cause,
        CylindricalFluxFailure::PairWeights { .. }
    ));
}

#[test]
fn input_count_and_cancellation_do_not_publish_partial_scalars() {
    let spans = [span(&A, &B, None), span(&A, &B, None)];
    let mut input = query(&spans);
    input.options.max_spans = 1;
    assert!(matches!(
        certify_cylindrical_boundary_flux(input, || false)
            .unwrap_err()
            .cause,
        CylindricalFluxFailure::SpanCount {
            actual: 2,
            maximum: 1
        }
    ));
    input.options.max_spans = 2;
    assert!(certify_cylindrical_boundary_flux(input, || true).is_err());
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CertifiedCylindricalFlux>();
}

#[test]
fn invalid_policy_radii_and_degree_refuse_with_original_context() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    input.options.max_degree = 17;
    assert!(matches!(
        certify_cylindrical_boundary_flux(input, || false)
            .unwrap_err()
            .cause,
        CylindricalFluxFailure::Policy(_)
    ));
}

#[test]
fn invalid_radius_order_refuses() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    input.radii = [1., 1.];
    assert!(matches!(
        certify_cylindrical_boundary_flux(input, || false)
            .unwrap_err()
            .cause,
        CylindricalFluxFailure::Radii(_)
    ));
}

#[test]
fn unsupported_degree_retains_the_original_error_context() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    input.options.max_degree = 0;
    let error = certify_cylindrical_boundary_flux(input, || false).unwrap_err();
    assert!(error.to_string().contains("cylindrical boundary flux"));
    assert!(matches!(
        error.cause,
        CylindricalFluxFailure::Integration { .. }
    ));
}

#[test]
fn generated_planar_patches_include_collapsed_axes_zero_flux_and_weight_gauges() {
    for index in 0..108 {
        let width = [0., 0.25, 1., 2.][index % 4];
        let height = [0., 0.5, 1.][index / 4 % 3];
        let z = [-2., 0., 1.5][index / 12 % 3];
        let weights: [Option<&[f64]>; 3] = [None, Some(&[1., 2.]), Some(&[1.5, 3.])];
        assert_planar_flux(width, height, z, weights[index / 36]);
    }
}

fn assert_planar_flux(width: f64, height: f64, z: f64, weights: Option<&[f64]>) {
    let a = [[0., 0., z], [width, 0., z]];
    let b = [[0., height, z], [width, height, z]];
    let spans = [span(&a, &b, weights)];
    let proof = certify_cylindrical_boundary_flux(query(&spans), || false).unwrap();
    let expected = width * height * z / 3.;
    assert!(
        proof.enclosure()[0] <= expected && expected <= proof.enclosure()[1],
        "width={width}, height={height}, z={z}, weights={weights:?}, enclosure={:?}",
        proof.enclosure()
    );
}

#[test]
fn public_flux_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CylindricalFluxOptions>();
    assert_send_sync::<CylindricalFluxSpan<'static>>();
    assert_send_sync::<CylindricalFluxQuery<'static>>();
    assert_send_sync::<CertifiedCylindricalFlux>();
    assert_send_sync::<CylindricalFluxError>();
    assert_send_sync::<CylindricalFluxFailure>();
    assert_send_sync::<crate::predicates::roots::metric_bounds::CylindricalFluxWork>();
}

#[test]
fn per_span_receipts_record_effective_limits_and_total_work() {
    let spans = [span(&A, &B, Some(&[1., 2.])), span(&A, &B, None)];
    let input = query(&spans);
    let proof = certify_cylindrical_boundary_flux(input, || false).unwrap();
    assert_eq!(proof.span_work().len(), 2);
    let total: u64 = proof
        .span_work()
        .iter()
        .map(|w| u64::from(w.examined_subintervals))
        .sum();
    assert_eq!(total, proof.integration_subintervals());
    for work in proof.span_work() {
        assert!(work.examined_subintervals <= work.effective_limit);
        assert!(work.effective_limit <= input.options.max_subintervals_per_span);
    }
}
