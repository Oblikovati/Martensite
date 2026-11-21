//! Global arrangement nesting determines selected material, not independent regional area.
use super::super::tests::{contour, request};
use super::*;
pub(super) fn rectangle(region: u64, first: u64, min: [f64; 2], max: [f64; 2]) -> Request {
    let [x, y] = min;
    let [a, b] = max;
    let mut result = request();
    result.region = SurfaceRegionId(region);
    result.loops = vec![contour(
        first,
        &[
            &[[x, y], [a, y]],
            &[[a, y], [a, b]],
            &[[a, b], [x, b]],
            &[[x, b], [x, y]],
        ],
    )];
    result
}
#[test]
fn separate_material_regions_are_selected_from_one_global_capture() {
    let requests = [request(), rectangle(100, 1000, [4., 0.], [5., 1.])];
    let proof = qualify(&requests).unwrap();
    assert_eq!(
        proof.members.keys().copied().collect::<Vec<_>>(),
        [SurfaceRegionId(5), SurfaceRegionId(100)]
    );
    assert!(proof.members[&SurfaceRegionId(5)].area.encloses(3.525));
    assert!(proof.members[&SurfaceRegionId(100)].area.encloses(1.));
    assert_eq!(proof.certificate.components, 3);
    assert_eq!(proof.certificate.loops, 3);
}
#[test]
fn an_island_inside_an_unselected_hole_is_disconnected_material() {
    let requests = [
        request(),
        rectangle(100, 1000, [0.625, 0.625], [0.875, 0.875]),
    ];
    let proof = qualify(&requests).unwrap();
    assert_eq!(proof.members.len(), 2);
    assert!(proof.members[&SurfaceRegionId(100)].area.encloses(0.0625));
    assert!(proof.members[&SurfaceRegionId(5)].area.encloses(3.525));
}
#[test]
fn region_containment_in_filled_material_is_not_disconnected_selection() {
    let requests = [request(), rectangle(100, 1000, [1.25, 0.5], [1.5, 0.75])];
    assert!(matches!(
        qualify(&requests),
        Err(SelectionFailure::Region {
            region: SurfaceRegionId(5),
            ..
        })
    ));
}
#[test]
fn selection_order_and_loop_start_do_not_change_authored_owners() {
    let mut requests = [request(), rectangle(100, 1000, [4., 0.], [5., 1.])];
    let first = qualify(&requests).unwrap();
    requests.reverse();
    for region in &mut requests {
        for contour in &mut region.loops {
            contour.rotate_left(1);
        }
    }
    let second = qualify(&requests).unwrap();
    assert_eq!(
        first.members.keys().collect::<Vec<_>>(),
        second.members.keys().collect::<Vec<_>>()
    );
    assert!(second.members[&SurfaceRegionId(5)].area.encloses(3.525));
}
#[test]
fn duplicate_region_or_curve_names_are_refused_before_capture() {
    let a = request();
    let mut b = rectangle(5, 1000, [4., 0.], [5., 1.]);
    assert!(matches!(
        qualify(&[a.clone(), b.clone()]),
        Err(SelectionFailure::DuplicateRegion(SurfaceRegionId(5)))
    ));
    b = rectangle(100, 10, [4., 0.], [5., 1.]);
    assert!(matches!(
        qualify(&[a, b]),
        Err(SelectionFailure::DuplicateCurve {
            curve: SurfaceCurveId(10),
            ..
        })
    ));
}
#[test]
fn different_support_or_extrusion_contexts_cannot_share_a_selection() {
    let a = request();
    let mut b = rectangle(100, 1000, [4., 0.], [5., 1.]);
    b.radius += 1.;
    assert!(matches!(
        qualify(&[a, b]),
        Err(SelectionFailure::Context {
            region: SurfaceRegionId(100),
            ..
        })
    ));
}
#[test]
fn empty_selection_and_malformed_source_are_refused_without_indexing() {
    assert!(matches!(qualify(&[]), Err(SelectionFailure::Empty)));
    let mut a = request();
    a.loops[0][0].poles.clear();
    assert!(matches!(
        qualify(&[a]),
        Err(SelectionFailure::Admission {
            region: SurfaceRegionId(5),
            ..
        })
    ));
}

#[test]
fn all_regions_share_one_complete_prepared_boundary_relation() {
    let requests = [request(), rectangle(100, 1000, [4., 0.], [5., 1.])];
    let evidence = prepare(&requests).unwrap();
    let count: usize = evidence
        .prepared
        .iter()
        .flat_map(|r| &r.loops)
        .map(Vec::len)
        .sum();
    assert_eq!(evidence.planar.members.len(), 2);
    assert_eq!(evidence.boundary_clearance.spans.len(), count);
    assert_eq!(
        evidence.boundary_clearance.certificate.pair_count(),
        count * (count - 1) / 2 - count
    );
    assert!(evidence.boundary_clearance.periodic.images().is_empty());
}
#[test]
fn separated_boundaries_do_not_prove_periodic_material_disjointness() {
    let mut a = rectangle(50, 100, [0., 0.], [2., 2.]);
    let mut b = rectangle(51, 200, [6.5, 0.5], [7., 1.]);
    for r in [&mut a, &mut b] {
        r.radius = 1.;
        r.depths = [-0.1, 0.2];
    }
    let evidence = prepare(&[a, b]).unwrap();
    assert_eq!(evidence.planar.members.len(), 2);
    assert_eq!(evidence.boundary_clearance.periodic.images().len(), 1);
    // These true-period bounds put B-period strictly inside A without touching its boundary.
    let [lo, hi] = evidence.boundary_clearance.periodic.circumference_bounds();
    assert!(hi.compare_f64(6.5).unwrap().is_lt());
    assert!(lo.compare_f64(5.).unwrap().is_gt());
}
#[test]
fn duplicate_vertex_names_and_collection_limits_are_refused() {
    let a = request();
    let mut b = rectangle(100, 1000, [4., 0.], [5., 1.]);
    for (i, piece) in b.loops[0].iter_mut().enumerate() {
        piece.start = CornerSource::Authored(SurfaceVertexId(10 + i as u64));
        piece.end = CornerSource::Authored(SurfaceVertexId(10 + (i as u64 + 1) % 4));
    }
    assert!(matches!(
        qualify(&[a.clone(), b]),
        Err(SelectionFailure::DuplicateVertex { .. })
    ));
    assert!(matches!(
        qualify(&vec![a; 65]),
        Err(SelectionFailure::Limit {
            role: "selected regions",
            actual: 65,
            maximum: 64
        })
    ));
}

#[test]
fn authored_curve_limit_and_depth_context_are_checked_before_capture() {
    let mut a = request();
    a.loops = vec![a.loops[0].clone(); 1025];
    assert!(matches!(
        qualify(&[a]),
        Err(SelectionFailure::Limit {
            role: "authored curves",
            actual: 4100,
            maximum: 4096
        })
    ));
    let a = request();
    let mut b = rectangle(100, 1000, [4., 0.], [5., 1.]);
    b.depths[1] += 1.;
    assert!(matches!(
        qualify(&[a, b]),
        Err(SelectionFailure::Context { .. })
    ));
}
#[test]
fn preparation_refusals_preserve_region_and_global_clearance_scope() {
    let mut a = request();
    a.max_fit_level = 0;
    assert!(matches!(
        prepare(&[a]),
        Err(SelectionFailure::Planning {
            region: SurfaceRegionId(5),
            ..
        })
    ));
    let a = rectangle(50, 100, [0., 0.], [1., 1.]);
    let b = rectangle(51, 200, [1. + 2e-5, 0.], [2., 1.]);
    assert!(matches!(
        prepare(&[a, b]),
        Err(SelectionFailure::Clearance(_))
    ));
}
#[test]
fn global_clearance_requires_complete_prepared_region_membership() {
    let a = request();
    let prepared = subdivision::prepare(&a).unwrap();
    assert!(matches!(
        planar_clearance::certify_many(&[a], &[]),
        Err(planar_clearance::ClearanceFailure::SelectionBinding { .. })
    ));
    assert!(matches!(
        planar_clearance::certify_many(&[], &[prepared]),
        Err(planar_clearance::ClearanceFailure::EmptySelection)
    ));
}

#[test]
fn undeclared_cross_component_contacts_are_refused_by_global_capture() {
    let requests = [request(), rectangle(100, 1000, [-0.125, 0.5], [0.125, 1.])];
    assert!(matches!(
        qualify(&requests),
        Err(SelectionFailure::Capture(_))
    ));
}
#[test]
fn selection_evidence_is_owned_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<PlanarSelectionEvidence>();
    check::<PreparedSelection>();
    check::<SelectionFailure>();
}

#[test]
fn cancellation_before_selection_preparation_does_not_enter_capture() {
    let mut malformed = request();
    malformed.loops.clear();
    assert!(matches!(
        prepare_with_cancel(&[malformed], || true),
        Err(SelectionFailure::Cancelled)
    ));
}
#[test]
fn a_selection_cancelled_in_fit_work_keeps_the_authored_owner_and_typed_cause() {
    let originals = [request()];
    let stop = successful_planar_polls(&originals) + 8;
    let mut polls = 0;
    let result = prepare_with_cancel(&originals, || {
        polls += 1;
        polls >= stop
    });
    assert!(matches!(
        result,
        Err(SelectionFailure::Planning {
            region: SurfaceRegionId(5),
            cause,
        }) if crate::surface_sketch::wrapped_profile::math_cancellation::MathCancellation::is_cancelled(cause.source_cause())
    ));
    assert_eq!(polls, stop);
}

fn successful_planar_polls(originals: &[Request]) -> usize {
    let mut planar_polls = 0;
    let work = martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault;
    qualify_with_work(originals, work, &mut || {
        planar_polls += 1;
        false
    })
    .unwrap();
    planar_polls
}
