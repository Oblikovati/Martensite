// SPDX-License-Identifier: GPL-2.0-only
use crate::{
    body_check::{
        CertifiedPlanarMaterial, DirectedCapCause, DirectedCapFailure, DirectedCapPolicy,
        DirectedCapWork,
    },
    profile_extrusion::*,
};

fn resolution() -> Resolution {
    Resolution::new(
        1e-5,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}
fn policy(cap: u16, requested: u64, remaining: u64) -> DirectedCapPolicy {
    DirectedCapPolicy {
        pair_bisections: 8192,
        max_components: cap,
        requested_integration_cells: requested,
        remaining_integration_cells: remaining,
    }
}
fn source(rings: &[&[[f64; 2]]]) -> (SketchProfile, Vec<ProfileLoop>) {
    let mut profile = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vec![],
        curves: vec![],
    };
    let mut loops = Vec::new();
    for &ring in rings {
        let first = profile.vertices.len();
        let keys = (0..ring.len())
            .map(|i| ProfileVertexId((first + i + 1) as u64))
            .collect::<Vec<_>>();
        let mut uses = Vec::new();
        for (i, &point) in ring.iter().enumerate() {
            let next = (i + 1) % ring.len();
            let id = ProfileCurveId(keys[i].0);
            let [start, end] = [keys[i], keys[next]];
            profile.vertices.push(ProfileVertex { id: start, point });
            profile.curves.push(ProfileCurve::Spline {
                id,
                start,
                end,
                degree: 1,
                poles: vec![point, ring[next]],
                weights: None,
                knots: vec![0., 0., 1., 1.],
            });
            uses.push(LoopUse {
                curve: id,
                reversed: false,
            });
        }
        loops.push(ProfileLoop { uses });
    }
    (profile, loops)
}
fn issue(
    rings: &[&[[f64; 2]]],
    policy: DirectedCapPolicy,
    cancel: impl FnMut() -> bool,
) -> Result<(CertifiedPlanarMaterial, DirectedCapWork), DirectedCapFailure> {
    let (profile, mut loops) = source(rings);
    let outer = loops.remove(0);
    CertifiedPlanarMaterial::from_directed_cycles_v1(
        profile,
        outer,
        loops,
        resolution(),
        policy,
        cancel,
    )
}
const OUTER: [[f64; 2]; 4] = [[0., 0.], [4., 0.], [4., 4.], [0., 4.]];
const HOLE: [[f64; 2]; 4] = [[1., 1.], [1., 2.], [2., 2.], [2., 1.]];

#[test]
fn original_invalid_is_refused_before_loan_clamp_or_profile_admission() {
    let invalid = 64 * 4096 + 1;
    let (mut profile, mut loops) = source(&[&OUTER]);
    profile.vertices[0].point[0] = f64::NAN;
    let result = CertifiedPlanarMaterial::from_directed_cycles_v1(
        profile,
        loops.remove(0),
        vec![],
        resolution(),
        policy(1, invalid, 0),
        || false,
    )
    .unwrap_err();
    assert!(
        matches!(*result.cause,DirectedCapCause::Policy{field:"max_total_subintervals",actual,minimum:0,maximum} if actual==invalid && maximum==invalid-1)
    );
    assert!(result.observed.is_none());
}

#[test]
fn complete_face_counts_outer_and_hole_components_before_numeric_work() {
    let result = issue(&[&OUTER, &HOLE], policy(1, 200, 0), || false).unwrap_err();
    assert!(matches!(
        *result.cause,
        DirectedCapCause::Components {
            actual: 2,
            limit: 1
        }
    ));
    assert!(result.observed.is_none());
    assert!(issue(&[&OUTER, &HOLE], policy(2, 200, 0), || false).is_ok());
}

#[test]
fn long_outer_chunks_and_holes_share_the_face_component_limit() {
    let outer = (0..65u32).map(|i| [f64::from(i), 0.]).collect::<Vec<_>>();
    let result = issue(&[&outer, &HOLE], policy(2, 200, 0), || false).unwrap_err();
    assert!(matches!(
        *result.cause,
        DirectedCapCause::Components {
            actual: 3,
            limit: 2
        }
    ));
    assert!(result.observed.is_none());
}

#[test]
fn cancellation_and_cheap_shape_limits_precede_native_admission() {
    let (mut profile, mut loops) = source(&[&OUTER]);
    profile.vertices[0].point[0] = f64::NAN;
    let ProfileCurve::Spline { knots, .. } = &mut profile.curves[0] else {
        panic!("expected spline")
    };
    knots.resize(1024, 0.);
    let outer = loops.remove(0);
    let mut polls = 0;
    let failure = CertifiedPlanarMaterial::from_directed_cycles_v1(
        profile.clone(),
        outer.clone(),
        vec![],
        resolution(),
        policy(1, 100, 0),
        || {
            polls += 1;
            true
        },
    )
    .unwrap_err();
    assert!(failure.cancelled && matches!(*failure.cause, DirectedCapCause::Cancelled));
    assert_eq!(polls, 1);
    assert!(failure.observed.is_none());
    let failure = CertifiedPlanarMaterial::from_directed_cycles_v1(
        profile,
        outer,
        vec![],
        resolution(),
        policy(1, 100, 0),
        || false,
    )
    .unwrap_err();
    assert!(matches!(*failure.cause, DirectedCapCause::Topology(_)));
    assert!(failure.observed.is_none());
}

#[test]
fn original_and_effective_caps_are_retained_separately() {
    let (_, a) = issue(&[&OUTER], policy(1, 200, 7), || false).unwrap();
    let (_, b) = issue(&[&OUTER], policy(1, 201, 7), || false).unwrap();
    assert_eq!(a.effective_integration_cells, 7);
    assert_eq!(b.effective_integration_cells, 7);
    assert_eq!(a.requested.requested_integration_cells, 200);
    assert_eq!(b.requested.requested_integration_cells, 201);
    assert_eq!(a.requested.remaining_integration_cells, 7);
    assert_eq!(a.integration_cells, 0);
    assert_ne!(a, b);
}

#[test]
fn numerical_refusal_retains_completed_work_in_separate_categories() {
    let mut reversed = OUTER;
    reversed.reverse();
    let result = issue(&[&reversed], policy(1, 200, 7), || false).unwrap_err();
    assert!(matches!(
        *result.cause,
        DirectedCapCause::Orientation {
            ring: 0,
            expected: 1,
            actual: -1
        }
    ));
    let work = result.observed.unwrap();
    assert_eq!(work.pair.requested_pair_bisections, 8192);
    assert_eq!(work.integration_cells, 0);
    assert_eq!(work.winding_bisections, 0);
    let crossed = [[0., 0.], [4., 4.], [0., 4.], [4., 1.]];
    let contact = issue(&[&crossed], policy(1, 200, 7), || false).unwrap_err();
    assert!(matches!(*contact.cause, DirectedCapCause::Pair { .. }));
    let partial = contact.observed.unwrap();
    assert!((1..10).contains(&partial.pair_domains));
    let pair = partial.pair;
    assert!(
        pair.consumed_pair_bisections > 0,
        "expected failed algebraic work: {pair:?}"
    );
}

#[test]
fn selected_roles_and_polynomial_bits_rebind_exactly() {
    let (profile, mut loops) = source(&[&OUTER, &HOLE]);
    let outer = loops.remove(0);
    let (proof, work) = CertifiedPlanarMaterial::from_directed_cycles_v1(
        profile.clone(),
        outer.clone(),
        loops.clone(),
        resolution(),
        policy(2, 100, 7),
        || false,
    )
    .unwrap();
    assert_eq!((work.pair_domains, work.loops), (36, 2));
    assert!(proof.matches_directed_cycles(&profile, &outer, &loops));
    assert!(!proof.matches_directed_cycles(&profile, &loops[0], std::slice::from_ref(&outer)));
    let mut changed = profile;
    let ProfileCurve::Spline { poles, .. } = &mut changed.curves[0] else {
        panic!("expected polynomial")
    };
    poles[0][0] = -0.;
    assert!(!proof.matches_directed_cycles(&changed, &outer, &loops));
}

#[test]
fn material_refuses_disjoint_outer_and_wrongly_nested_holes_after_pair_proof() {
    let outside = HOLE.map(|[x, y]| [x + 5., y]);
    let issue_cap = |rings: &[&[[f64; 2]]]| issue(rings, policy(3, 200, 0), || false);
    let outside_failure = issue_cap(&[&OUTER, &outside]).unwrap_err();
    assert!(matches!(
        *outside_failure.cause,
        DirectedCapCause::Nesting {
            ring: 1,
            against: 0,
            expected: true
        }
    ));
    assert_eq!(outside_failure.observed.unwrap().pair_domains, 36);

    let inside_hole = HOLE.map(|[x, y]| [1.25 + (x - 1.) / 2., 1.25 + (y - 1.) / 2.]);
    let nested_failure = issue_cap(&[&OUTER, &HOLE, &inside_hole]).unwrap_err();
    assert!(matches!(
        *nested_failure.cause,
        DirectedCapCause::Nesting {
            ring: 2,
            against: 1,
            expected: false
        }
    ));
    assert_eq!(nested_failure.observed.unwrap().pair_domains, 78);
}
