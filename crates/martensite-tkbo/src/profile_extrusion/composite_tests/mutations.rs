// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::sketch_region_prism::{IntegralRole, IntegralStep};
fn issued() -> PlanarIntegralEvidence {
    build(&profile(), version(1024), 17)
        .unwrap()
        .integral_program_evidence()
        .unwrap()
        .clone()
}

#[test]
fn cloned_program_shares_captured_sources_and_rejects_an_unknown_parent() {
    use std::sync::Arc;
    let original = issued();
    let mut clone = original.clone();
    assert!(Arc::ptr_eq(&original.source, &clone.source));
    assert!(Arc::ptr_eq(&original.source_bits, &clone.source_bits));
    assert!(Arc::ptr_eq(&original.parents, &clone.parents));
    Arc::make_mut(&mut clone.parents)[0].1 = ProfileCurveId(u64::MAX);
    assert!(!clone.matches_profile(&profile()));
    assert!(original.matches_profile(&profile()));
}
#[test]
fn source_signed_zero_order_and_authored_identity_are_bound() {
    let proof = issued();
    let mut source = profile();
    source.vertices[0].point[0] = -0.;
    assert!(!proof.matches_profile(&source));
    let mut source = profile();
    source.curves.swap(0, 1);
    assert!(!proof.matches_profile(&source));
    let mut source = profile();
    source.vertices[0].id = ProfileVertexId(99);
    assert!(!proof.matches_profile(&source));
}
#[test]
fn altered_generation_grant_and_observed_work_refuse_rebinding() {
    let mut proof = issued();
    proof.entry_allowance = 16;
    assert!(!proof.matches_profile(&profile()));
    let mut proof = issued();
    std::sync::Arc::make_mut(&mut proof.graph).consumed += 1;
    assert!(!proof.matches_profile(&profile()));
}
#[test]
fn actual_coefficients_and_program_role_are_not_scalar_substitutions() {
    let mut proof = issued();
    std::sync::Arc::make_mut(&mut proof.graph).sources[0].poles[0][0] += 1.;
    assert!(!proof.matches_profile(&profile()));
    let mut proof = issued();
    change_boundary_role(&mut proof);
    assert!(!proof.matches_profile(&profile()));
}
#[test]
fn reverse_derived_step_must_refer_to_an_earlier_actual_source_reversal() {
    let mut proof = issued();
    let graph = std::sync::Arc::make_mut(&mut proof.graph);
    let reverse = graph
        .steps
        .iter_mut()
        .find(|step| matches!(step, IntegralStep::Reverse { .. }))
        .unwrap();
    let IntegralStep::Reverse { parent, .. } = reverse else {
        panic!("derived reverse")
    };
    *parent = usize::MAX;
    assert!(!proof.matches_profile(&profile()));
}

fn change_boundary_role(proof: &mut PlanarIntegralEvidence) {
    let graph = std::sync::Arc::make_mut(&mut proof.graph);
    let direct = graph
        .steps
        .iter_mut()
        .find(|step| {
            matches!(
                step,
                IntegralStep::Direct {
                    role: IntegralRole::Boundary(_),
                    ..
                }
            )
        })
        .unwrap();
    let IntegralStep::Direct { role, .. } = direct else {
        panic!("boundary program")
    };
    *role = IntegralRole::Face(1);
}
