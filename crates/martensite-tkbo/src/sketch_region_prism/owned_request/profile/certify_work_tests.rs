// SPDX-License-Identifier: GPL-2.0-only
//! Explicit pair work never changes geometric or other child-policy limits.
use super::*;

#[test]
fn inherited_pair_work_keeps_the_existing_4096_limit() {
    let inherited = policy(1e-8, None);
    assert_eq!(inherited.census.roots.max_bisections, 4096);
    assert_eq!(inherited.source.roots, inherited.census.roots);
    assert_eq!(inherited.contacts.roots, inherited.census.roots);
    assert_eq!(inherited.region.winding, inherited.census.roots);
}
#[test]
fn explicit_pair_work_changes_only_the_census_bisection_limit() {
    let inherited = policy(1e-8, None);
    let explicit = policy(1e-8, Some(5000));
    let mut expected = inherited.census.roots;
    expected.max_bisections = 5000;
    assert_eq!(explicit.census.roots, expected);
    assert_eq!(
        explicit.census.max_world_error,
        inherited.census.max_world_error
    );
    assert_eq!(explicit.source.roots, inherited.source.roots);
    assert_eq!(explicit.contacts.roots, inherited.contacts.roots);
    assert_eq!(explicit.region.winding, inherited.region.winding);
}
fn lens_input(pair_bisections: Option<u16>) -> ProfileGraphInput {
    ProfileGraphInput {
        edges: vec![
            lens_edge(1, [[0., 0., 0.], [1., -1., 0.], [2., 0., 0.]], [1, 2]),
            lens_edge(2, [[2., 0., 0.], [1., 1., 0.], [0., 0., 0.]], [2, 1]),
        ],
        plane: Plane3::default(),
        spatial_limit: 1e-8,
        pair_bisections,
    }
}
fn lens_edge(id: u64, poles: [[f64; 3]; 3], ends: [u64; 2]) -> ProofEdge {
    ProofEdge {
        id,
        world_poles: poles.to_vec(),
        weights: None,
        ends,
    }
}
#[test]
fn explicit_work_retains_the_independent_lens_area() {
    let result = certify_profile_graph(lens_input(Some(4096))).unwrap();
    assert_eq!(result.faces.len(), 1);
    let area = &result.faces[0].area;
    assert!((area.representative() - 4. / 3.).abs() <= area.absolute_error_bound() + f64::EPSILON);
}
#[test]
fn zero_pair_work_accepts_the_analytically_complete_lens() {
    // The budget limits subdivisions; exact algebraic completion needs none.
    let result = certify_profile_graph(lens_input(Some(0))).unwrap();
    assert_eq!(result.faces.len(), 1);
    let area = &result.faces[0].area;
    assert!((area.representative() - 4. / 3.).abs() <= area.absolute_error_bound() + f64::EPSILON);
}

fn crossing_input() -> ProfileGraphInput {
    let points = [[0., 0., 0.], [2., 2., 0.], [0., 2., 0.], [2., 0., 0.]];
    let edges = (0..4)
        .map(|i| {
            let next = (i + 1) % 4;
            ProofEdge {
                id: i as u64 + 1,
                world_poles: vec![points[i], points[next]],
                weights: None,
                ends: [i as u64 + 1, next as u64 + 1],
            }
        })
        .collect();
    ProfileGraphInput {
        edges,
        plane: Plane3::default(),
        spatial_limit: 1e-8,
        pair_bisections: None,
    }
}
#[test]
fn graph_refusal_retains_observed_shared_work() {
    let failure = certify_profile_graph(crossing_input()).unwrap_err();
    let work = failure
        .work
        .expect("crossing graph reaches the completed census");
    assert_eq!(work.requested_pair_bisections, 4096);
    assert_eq!(work.effective_pair_bisections, 4096);
    assert!(work.consumed_pair_bisections <= 4096);
}
#[test]
fn graph_admission_failure_does_not_invent_an_observed_receipt() {
    let mut input = lens_input(None);
    input.edges[0].world_poles.clear();
    let failure = certify_profile_graph(input).unwrap_err();
    assert!(failure.work.is_none());
    assert!(matches!(failure.cause, ProfileProofCause::Incomplete(_)));
}
