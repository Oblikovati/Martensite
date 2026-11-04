// SPDX-License-Identifier: GPL-2.0-only
use super::*;
fn input(
    curve: VertexNeighborhoodCurve,
    radius: f64,
    ends: VertexNeighborhoodEnds,
) -> GivenVertexNeighborhoodInput {
    GivenVertexNeighborhoodInput {
        vertex: 19,
        curve,
        interval: [0., 1.],
        placements: Vec::new(),
        point: [0.; 3],
        point_placements: Vec::new(),
        radius,
        ends,
    }
}
fn line(origin: [f64; 3], direction: [f64; 3], radius: f64) -> GivenVertexNeighborhoodInput {
    input(
        VertexNeighborhoodCurve::Line { origin, direction },
        radius,
        VertexNeighborhoodEnds::First,
    )
}
fn spline(
    poles: [[f64; 3]; 3],
    radius: f64,
    ends: VertexNeighborhoodEnds,
) -> GivenVertexNeighborhoodInput {
    input(
        VertexNeighborhoodCurve::Quadratic {
            poles: poles.to_vec(),
            flat_knots: vec![0., 0., 0., 1., 1., 1.],
        },
        radius,
        ends,
    )
}
fn neighborhood(input: GivenVertexNeighborhoodInput) -> GivenVertexNeighborhoodReport {
    let report =
        certify_given_vertex_neighborhood(input, VertexNeighborhoodPolicy::v1(10000), || false);
    assert!(report.certificate().is_some(), "{:?}", report.failure());
    report
}
fn contacts(
    source: GivenVertexNeighborhoodInput,
    target: GivenVertexNeighborhoodInput,
) -> GivenVertexContactsReport {
    certify_given_vertex_contacts(
        &neighborhood(source),
        &neighborhood(target),
        VertexNeighborhoodPolicy::v1(10000),
        || false,
    )
}
#[test]
fn complete_contact_witnesses_retain_both_declared_curve_ends() {
    let source = spline(
        [[0.; 3], [2., 0., 0.], [0.; 3]],
        0.25,
        VertexNeighborhoodEnds::Both,
    );
    let result = contacts(source, line([0.; 3], [0., 0., 1.], 0.25));
    let witnesses = result.contacts().unwrap();
    assert_eq!(witnesses.len(), 2);
    assert!(
        witnesses
            .iter()
            .all(|c| c.line_bounds[0] == std::cmp::Ordering::Equal)
    );
    assert!(
        witnesses
            .iter()
            .all(|c| c.neighborhood_sign == std::cmp::Ordering::Less)
    );
}
#[test]
fn actual_no_contact_is_completed_empty_not_exact_coincidence() {
    let source = spline(
        [[0.125, 0., 0.], [2., 0., 0.], [0.125, 0., 0.]],
        0.25,
        VertexNeighborhoodEnds::Both,
    );
    let result = contacts(source, line([0.; 3], [0., 0., 1.], 0.25));
    assert_eq!(result.contacts(), Some([].as_slice()));
}
#[test]
fn actual_interior_contact_outside_given_components_refuses() {
    let source = spline(
        [[0.; 3], [0.5, -0.5, 0.], [1., 1., 0.]],
        0.1,
        VertexNeighborhoodEnds::First,
    );
    let result = contacts(source, line([0.; 3], [1., 0., 0.], 0.1));
    let Some(GivenVertexContactFailure::NonIncident { contact }) = result.failure() else {
        panic!("{result:?}");
    };
    assert_eq!(contact.neighborhood_sign, std::cmp::Ordering::Greater);
    assert!(result.contacts().is_none());
}
#[test]
fn irrational_actual_contact_uses_exact_signs_not_a_bracket_midpoint() {
    let source = spline(
        [[0.; 3], [0.; 3], [0., 1., 0.]],
        0.2,
        VertexNeighborhoodEnds::First,
    );
    let result = contacts(source, line([0., 0.125, 0.], [0., 0., 1.], 0.2));
    let witnesses = result.contacts().unwrap();
    assert_eq!(witnesses.len(), 1);
    assert_eq!(witnesses[0].neighborhood_sign, std::cmp::Ordering::Less);
    assert_eq!(witnesses[0].line_bounds[0], std::cmp::Ordering::Equal);
}
#[test]
fn collinear_overlap_does_not_become_a_finite_contact_list() {
    let result = contacts(
        line([0.; 3], [1., 0., 0.], 0.2),
        line([0.; 3], [1., 0., 0.], 0.2),
    );
    assert_eq!(
        result.failure(),
        Some(&GivenVertexContactFailure::CollinearUnclassified { span: 0 })
    );
}
#[test]
fn declared_vertex_identity_is_not_inferred_from_equal_coordinates() {
    let a = neighborhood(line([0.; 3], [1., 0., 0.], 0.2));
    let mut b = line([0.; 3], [0., 1., 0.], 0.2);
    b.vertex = 20;
    let result = certify_given_vertex_contacts(
        &a,
        &neighborhood(b),
        VertexNeighborhoodPolicy::v1(10000),
        || false,
    );
    assert_eq!(
        result.failure(),
        Some(&GivenVertexContactFailure::Binding {
            field: "given vertex identity"
        })
    );
}
#[test]
fn combined_budget_counts_both_existing_neighborhoods() {
    let a = neighborhood(line([0.; 3], [1., 0., 0.], 0.2));
    let b = neighborhood(line([0.; 3], [0., 1., 0.], 0.2));
    let limit = a.work() + b.work() - 1;
    let result =
        certify_given_vertex_contacts(&a, &b, VertexNeighborhoodPolicy::v1(limit), || false);
    assert_eq!(
        result.failure(),
        Some(&GivenVertexContactFailure::Exact(
            VertexNeighborhoodFailure::WorkLimit { limit }
        ))
    );
}
#[test]
fn negative_target_direction_preserves_actual_finite_trim_classification() {
    let result = contacts(
        line([0.; 3], [1., 0., 0.], 0.2),
        line([0.; 3], [0., -1., 0.], 0.2),
    );
    assert_eq!(result.contacts().unwrap().len(), 1);
    assert_eq!(
        result.contacts().unwrap()[0].line_bounds[0],
        std::cmp::Ordering::Equal
    );
}
#[test]
fn owned_contact_reports_are_send_sync_and_thread_deterministic() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<GivenVertexContactsReport>();
    let run = || {
        contacts(
            line([0.; 3], [1., 0., 0.], 0.2),
            line([0.; 3], [0., 1., 0.], 0.2),
        )
    };
    let first = run();
    let second = std::thread::spawn(run).join().unwrap();
    assert_eq!(first, second);
}

#[test]
fn disjoint_finite_collinear_trims_remain_unclassified_not_asserted_overlapping() {
    let a = line([-0.1, 0., 0.], [-1., 0., 0.], 0.2);
    let b = line([0.1, 0., 0.], [1., 0., 0.], 0.2);
    let report = contacts(a, b);
    assert_eq!(
        report.failure(),
        Some(&GivenVertexContactFailure::CollinearUnclassified { span: 0 })
    );
}

#[test]
fn print_owned_incidence_report_for_process_comparison() {
    let source = spline(
        [[0.; 3], [0.; 3], [0., 1., 0.]],
        0.2,
        VertexNeighborhoodEnds::First,
    );
    let report = contacts(source, line([0., 0.125, 0.], [0., 0., 1.], 0.2));
    assert_eq!(report.contacts().unwrap().len(), 1);
    println!("VERTEX_INCIDENCE_REPORT={report:?}");
}
