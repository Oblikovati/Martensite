//! Closed-form material controls exercise containment that boundary separation cannot detect.
use super::super::tests::{contour, request};
use super::tests::rectangle;
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::CylindricalProbeSense;
use material::OutsideReason;
fn unit_support(mut regions: Vec<Request>) -> Vec<Request> {
    for region in &mut regions {
        region.radius = 1.;
        region.depths = [-0.1, 0.2];
    }
    regions
}
fn separated() -> Vec<Request> {
    unit_support(vec![
        rectangle(50, 100, [0., 0.], [0.25, 1.]),
        rectangle(51, 200, [6.5, 2.], [6.75, 3.]),
    ])
}
#[test]
fn narrow_disconnected_selection_needs_no_explicit_winding_queries() {
    let inputs = [request(), rectangle(100, 1000, [4., 0.], [5., 1.])];
    let mut options = MaterialOptions::default();
    options.probes.max_probes = 0;
    options.probes.max_operations = 0;
    options.max_locations = 0;
    options.winding.max_bisections = 0;
    let proof = certify(&inputs, options).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(proof.prepared.planar.members.len(), 2);
    assert!(proof.material.probes.probes().is_empty());
    assert!(proof.material.locations.is_empty());
    assert!(proof.material.relations.is_empty());
    assert_eq!(proof.material.options, options);
}
#[test]
fn both_directions_of_all_ordered_periodic_region_pairs_are_classified() {
    let inputs = separated();
    let proof = certify(&inputs, Default::default()).unwrap_or_else(|e| panic!("{e}"));
    let m = &proof.material;
    assert_eq!(m.relations.len(), 8);
    assert_eq!(m.probes.probes().len(), 8);
    assert!(
        m.relations
            .iter()
            .all(|r| r.reason == OutsideReason::OutsideOuter)
    );
    for relation in &m.relations {
        let probe = &m.probes.probes()[relation.probe];
        assert_eq!(probe.request().turn, 1);
        assert!(matches!(relation.source.0, 50 | 51));
        assert!(matches!(relation.target.0, 50 | 51));
        assert!(matches!(relation.witness.0, 100 | 200));
    }
    receipts(m);
}
fn receipts(m: &PeriodicMaterialEvidence) {
    let mut used = 0;
    for receipt in &m.locations {
        assert_eq!(
            receipt.effective_bisections,
            m.options.winding.max_bisections - used
        );
        used += receipt.bisections;
        assert!(matches!(receipt.winding, -1..=1));
        assert!(matches!(receipt.target.0, 50 | 51));
        assert!(matches!(receipt.contour.0, 100 | 200 | 300));
        assert!(receipt.probe < m.probes.probes().len());
    }
    assert_eq!(used, m.bisections);
}
fn contained() -> Vec<Request> {
    unit_support(vec![
        rectangle(50, 100, [0., 0.], [2., 2.]),
        rectangle(51, 200, [6.5, 0.5], [7., 1.]),
    ])
}

#[test]
fn periodic_strict_containment_in_filled_material_is_refused() {
    let inputs = contained();
    let Err(SelectionFailure::Material(cause)) = certify(&inputs, Default::default()) else {
        panic!("wrapped containment must refuse");
    };
    let MaterialFailure::Overlap {
        source,
        target,
        witness,
        probe,
    } = *cause
    else {
        panic!("expected containment witness");
    };
    assert_eq!((source.0, target.0, witness.0), (51, 50, 200));
    assert_eq!(probe.turn, 1);
    assert_eq!(probe.sense, CylindricalProbeSense::Subtract);
}
fn hole_selection() -> Vec<Request> {
    let mut a = rectangle(50, 100, [0., 0.], [2., 2.]);
    a.loops.push(contour(
        300,
        &[
            &[[0.25, 0.25], [0.25, 1.75]],
            &[[0.25, 1.75], [1.75, 1.75]],
            &[[1.75, 1.75], [1.75, 0.25]],
            &[[1.75, 0.25], [0.25, 0.25]],
        ],
    ));
    unit_support(vec![a, rectangle(51, 200, [6.8, 0.5], [7.3, 1.])])
}

#[test]
fn periodic_containment_in_an_unselected_hole_is_allowed() {
    let inputs = hole_selection();
    let proof = certify(&inputs, Default::default()).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        proof
            .material
            .relations
            .iter()
            .any(|r| r.reason == OutsideReason::InsideHole(SurfaceCurveId(300)))
    );
    receipts(&proof.material);
}
#[test]
fn wire_location_cap_is_not_reset_per_member() {
    let inputs = separated();
    let proof = certify(&inputs, Default::default()).unwrap_or_else(|e| panic!("{e}"));
    let options = MaterialOptions {
        max_locations: proof.material.locations.len() as u32 - 1,
        ..Default::default()
    };
    assert!(matches!(
        material::certify(&inputs, &proof.prepared, options),
        Err(MaterialFailure::Limit {
            role: "wire locations",
            ..
        })
    ));
}
#[test]
fn probe_cap_cannot_drop_required_region_pair_directions() {
    let inputs = separated();
    let proof = prepare(&inputs).unwrap_or_else(|e| panic!("{e}"));
    let options = MaterialOptions {
        probes: martensite_tkmath::predicates::roots::metric_bounds::CylindricalProbeOptions {
            max_probes: 7,
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(matches!(
        material::certify(&inputs, &proof, options),
        Err(MaterialFailure::Limit {
            role: "periodic probes",
            actual: 8,
            maximum: 7
        })
    ));
}
#[test]
fn stale_original_curve_geometry_cannot_reuse_planar_or_boundary_evidence() {
    let mut inputs = [request()];
    let prepared = prepare(&inputs).unwrap_or_else(|e| panic!("{e}"));
    inputs[0].loops[0][1].poles[1][0] += 0.01;
    assert!(matches!(
        material::certify(&inputs, &prepared, Default::default()),
        Err(MaterialFailure::Source(SurfaceRegionId(5)))
    ));
}
#[test]
fn periodic_material_evidence_is_owned_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<QualifiedSelection>();
    check::<PeriodicMaterialEvidence>();
    check::<MaterialOptions>();
    check::<MaterialFailure>();
}

fn curved_separated() -> Vec<Request> {
    unit_support(vec![request(), rectangle(51, 200, [9.8, 0.6], [10.1, 0.9])])
}
#[test]
fn curved_winding_queries_share_one_actual_subdivision_allowance() {
    let inputs = curved_separated();
    let proof = certify(&inputs, Default::default()).unwrap_or_else(|e| panic!("{e}"));
    assert!(proof.material.bisections > 0);
    let mut options = MaterialOptions::default();
    options.winding.max_bisections = proof.material.bisections - 1;
    let Err(MaterialFailure::Winding {
        previous_bisections,
        effective,
        ..
    }) = material::certify(&inputs, &proof.prepared, options)
    else {
        panic!("one fewer subdivision must refuse");
    };
    assert!(previous_bisections > 0);
    assert!(effective.max_bisections < options.winding.max_bisections);
}
#[test]
fn cancellation_is_observed_before_probes_and_in_winding() {
    let inputs = separated();
    let proof = certify(&inputs, Default::default()).unwrap_or_else(|e| panic!("{e}"));
    assert!(matches!(
        material::certify_with_cancel(&inputs, &proof.prepared, Default::default(), || true),
        Err(MaterialFailure::Cancelled)
    ));
    let mut polls = 0;
    let cutoff = proof.material.probes.operations() + 2;
    let result =
        material::certify_with_cancel(&inputs, &proof.prepared, Default::default(), || {
            polls += 1;
            polls > cutoff
        });
    assert!(matches!(result, Err(MaterialFailure::Winding { .. })));
}
#[test]
fn source_interval_omission_cannot_stand_in_for_complete_boundary_clearance() {
    let inputs = separated();
    let mut prepared = prepare(&inputs).unwrap_or_else(|e| panic!("{e}"));
    prepared.prepared[0].loops[0].pop();
    assert!(matches!(
        planar_clearance::certify_many(&inputs, &prepared.prepared),
        Err(planar_clearance::ClearanceFailure::OriginalBinding)
    ));
}
#[test]
fn caller_order_and_loop_start_do_not_change_material_owners() {
    let mut inputs = separated();
    let first = certify(&inputs, Default::default()).unwrap_or_else(|e| panic!("{e}"));
    inputs.reverse();
    for r in &mut inputs {
        r.loops[0].rotate_left(1);
    }
    let second = certify(&inputs, Default::default()).unwrap_or_else(|e| panic!("{e}"));
    let owners = |m: &PeriodicMaterialEvidence| {
        m.relations
            .iter()
            .map(|r| (r.source, r.target, r.reason))
            .collect::<Vec<_>>()
    };
    assert_eq!(owners(&first.material), owners(&second.material));
}

#[test]
fn policy_and_prepared_context_edits_are_refused_before_classification() {
    let inputs = separated();
    let mut proof = prepare(&inputs).unwrap_or_else(|e| panic!("{e}"));
    let mut options = MaterialOptions::default();
    options.probes.max_probes = u32::MAX;
    assert!(matches!(
        material::certify(&inputs, &proof, options),
        Err(MaterialFailure::Policy(_))
    ));
    proof.prepared[0].depths[0] -= 0.1;
    assert!(matches!(
        material::certify(&inputs, &proof, Default::default()),
        Err(MaterialFailure::Source(SurfaceRegionId(50)))
    ));
}
#[test]
fn an_unrelated_periodic_report_cannot_replace_the_recorded_global_relation() {
    let inputs = separated();
    let mut proof = prepare(&inputs).unwrap_or_else(|e| panic!("{e}"));
    let unrelated = prepare(&[request()]).unwrap_or_else(|e| panic!("{e}"));
    proof.boundary_clearance.periodic = unrelated.boundary_clearance.periodic;
    assert!(matches!(
        material::certify(&inputs, &proof, Default::default()),
        Err(MaterialFailure::Clearance(_))
    ));
}
