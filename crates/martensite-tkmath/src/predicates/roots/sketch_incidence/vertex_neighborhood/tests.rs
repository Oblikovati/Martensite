// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::cmp::Ordering::{Greater, Less};
fn line() -> GivenVertexNeighborhoodInput {
    GivenVertexNeighborhoodInput {
        vertex: 7,
        curve: VertexNeighborhoodCurve::Line {
            origin: [0.; 3],
            direction: [1., 0., 0.],
        },
        interval: [0., 4.],
        placements: Vec::new(),
        point: [0.; 3],
        point_placements: Vec::new(),
        radius: 1.,
        ends: VertexNeighborhoodEnds::First,
    }
}
fn run(input: GivenVertexNeighborhoodInput) -> GivenVertexNeighborhoodReport {
    certify_given_vertex_neighborhood(input, VertexNeighborhoodPolicy::v1(10000), || false)
}
#[test]
fn line_keeps_exact_boundary_and_declared_identity() {
    let report = run(line());
    let proof = report.certificate().unwrap();
    assert_eq!(report.input().vertex, 7);
    assert_eq!(proof.endpoint_signs(), [Less, Greater]);
    assert_eq!(proof.crossings().len(), 1);
    assert_eq!(
        proof.crossings()[0]
            .parameter
            .bounds()
            .0
            .compare_f64(1.)
            .unwrap(),
        std::cmp::Ordering::Equal
    );
}
#[test]
fn quadratic_has_two_proper_endpoint_components() {
    let mut input = line();
    input.curve = VertexNeighborhoodCurve::Quadratic {
        poles: vec![[0.; 3], [2., 0., 0.], [0.; 3]],
        flat_knots: vec![0., 0., 0., 1., 1., 1.],
    };
    input.interval = [0., 1.];
    input.radius = 0.25;
    input.ends = VertexNeighborhoodEnds::Both;
    let report = run(input);
    assert_eq!(report.certificate().unwrap().crossings().len(), 2);
    assert_eq!(report.certificate().unwrap().endpoint_signs(), [Less, Less]);
}
#[test]
fn entire_curve_in_ball_never_proves_a_proper_endpoint_component() {
    let mut input = line();
    input.radius = 100.;
    assert!(matches!(
        run(input).failure(),
        Some(VertexNeighborhoodFailure::Components { .. })
    ));
}
#[test]
fn sphere_tangency_is_not_an_exit() {
    let mut input = line();
    input.point = [2., 1., 0.];
    assert!(matches!(
        run(input).failure(),
        Some(VertexNeighborhoodFailure::Components { .. })
    ));
}
#[test]
fn shared_knot_root_is_one_crossing() {
    let mut input = line();
    input.curve = VertexNeighborhoodCurve::Quadratic {
        poles: vec![
            [0.; 3],
            [1., 0., 0.],
            [2., 0., 0.],
            [3., 0., 0.],
            [4., 0., 0.],
        ],
        flat_knots: vec![0., 0., 0., 1., 1., 2., 2., 2.],
    };
    input.interval = [0., 2.];
    input.radius = 2.;
    let report = run(input);
    assert_eq!(report.certificate().unwrap().crossings().len(), 1);
}
#[test]
fn extra_components_refuse_despite_declared_endpoint_incidence() {
    let mut input = line();
    input.curve = VertexNeighborhoodCurve::Quadratic {
        poles: vec![[0.; 3], [4., 0., 0.], [0.; 3], [4., 0., 0.], [0.; 3]],
        flat_knots: vec![0., 0., 0., 1., 1., 2., 2., 2.],
    };
    input.interval = [0., 2.];
    input.ends = VertexNeighborhoodEnds::Both;
    assert!(matches!(
        run(input).failure(),
        Some(VertexNeighborhoodFailure::Components { .. })
    ));
}
#[test]
fn cancelled_and_exhausted_requests_keep_inputs_without_certificate() {
    let input = line();
    let cancelled =
        certify_given_vertex_neighborhood(input.clone(), VertexNeighborhoodPolicy::v1(9), || true);
    assert_eq!(
        cancelled.failure(),
        Some(&VertexNeighborhoodFailure::Cancelled)
    );
    assert_eq!(cancelled.input(), &input);
    let exhausted =
        certify_given_vertex_neighborhood(input, VertexNeighborhoodPolicy::v1(0), || false);
    assert_eq!(
        exhausted.failure(),
        Some(&VertexNeighborhoodFailure::WorkLimit { limit: 0 })
    );
}
#[test]
fn placement_products_are_exact_and_input_is_owned() {
    let mut input = line();
    let mut placement = VertexPlacement::identity();
    placement.scale = 2.;
    placement.translation = [3., 0., 0.];
    input.placements.push(placement.clone());
    input.point_placements.push(placement);
    let report = run(input.clone());
    input.placements[0].scale = 100.;
    assert!(report.certificate().is_some());
    assert_eq!(report.input().placements[0].scale, 2.);
}
#[test]
fn malformed_and_nonfinite_input_has_a_typed_refusal() {
    let mut input = line();
    input.radius = f64::NAN;
    assert!(matches!(
        run(input).failure(),
        Some(VertexNeighborhoodFailure::Input {
            field: "positive radius",
            ..
        })
    ));
    let mut input = line();
    input.curve = VertexNeighborhoodCurve::Quadratic {
        poles: vec![],
        flat_knots: vec![],
    };
    assert!(matches!(
        run(input).failure(),
        Some(VertexNeighborhoodFailure::Spline(_))
    ));
}
#[test]
fn publication_types_are_send_sync_and_threads_are_deterministic() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<GivenVertexNeighborhoodReport>();
    send_sync::<GivenVertexNeighborhoodCertificate>();
    let expected = run(line());
    let actual = std::thread::spawn(|| run(line())).join().unwrap();
    assert_eq!(expected, actual);
}
