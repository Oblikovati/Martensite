// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn each_actual_carrier_coordinate_is_bound_without_loss_of_signed_zero() {
    let spans = [span(&A, &B, None)];
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    for depth in 0..2 {
        let mut input = query(&spans);
        input.carriers[depth].radius += 1.;
        assert!(!proof.matches_source(input));
        input = query(&spans);
        input.carriers[depth].axial_origin += 1.;
        assert!(!proof.matches_source(input));
        input = query(&spans);
        input.carriers[depth].semi_angle += 0.125;
        assert!(!proof.matches_source(input));
    }
}

#[test]
fn actual_occurrence_flags_and_cap_controls_are_source_bound() {
    let mut spans = [span(&A, &B, None)];
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    spans[0].side_reversed = true;
    assert!(!proof.matches_source(query(&spans)));
    spans[0].side_reversed = false;
    for depth in 0..2 {
        spans[0].cap_reversed[depth] = true;
        assert!(!proof.matches_source(query(&spans)));
        spans[0].cap_reversed[depth] = false;
        spans[0].cap_pcurves[depth] = &[[0., -0.], [0., 0.]];
        assert!(!proof.matches_source(query(&spans)));
        spans[0].cap_pcurves[depth] = &UV;
    }
    assert!(proof.matches_source(query(&spans)));
}

#[test]
fn both_space_sources_and_explicit_unit_weights_are_bound() {
    let mut spans = [span(&A, &B, None)];
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    spans[0].boundaries[0].poles = &[[0., 0., 2.], [1., 0., 1.]];
    assert!(!proof.matches_source(query(&spans)));
    spans[0] = span(&A, &[[0., 1., 2.], [1., 1., 1.]], None);
    assert!(!proof.matches_source(query(&spans)));
    spans[0] = span(&A, &B, Some(&[1., 1.]));
    assert!(!proof.matches_source(query(&spans)));
}

#[test]
fn valid_policy_changes_do_not_change_the_retained_geometric_source() {
    let spans = [span(&A, &B, None)];
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    let mut input = query(&spans);
    input.options.max_total_subintervals = 0;
    input.arithmetic.max_operations = 0;
    assert!(proof.matches_source(input));
}

#[test]
fn actual_reference_circle_origin_preserves_signed_zero_bits() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    input.carriers[0].axial_origin = 0.;
    let proof = certify_analytic_conical_boundary_flux(input, || false).unwrap();
    input.carriers[0].axial_origin = -0.;
    assert!(!proof.matches_source(input));
}
