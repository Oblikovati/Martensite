// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp,
    topods::Edge,
};
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkmath::gp::{Direction3, Point3};

const POLICY: HelixFitOptions = HelixFitOptions {
    max_turns: 8.,
    approximation_tolerance: 1e-8,
    max_segments: 128,
    max_evaluations: 32_768,
    max_topology_tolerance: 1e-6,
    trace: HelixTraceLimits {
        max_position_error: 1e-6,
        max_native_spans: 96,
        max_cells: 4096,
        max_operations: 5_000_000,
    },
};

fn table() -> HelixSequence {
    HelixSequence::pitch_table(
        cylinder(HelixRotation::Positive),
        &[row(9, 2., 1.), row(2, 4., 0.5)],
    )
    .unwrap()
}

fn check_trim(edge: &Edge, receipt: HelixSequenceSpanReceipt) {
    let actual = BRepTool::curve_with_location(edge).unwrap();
    assert!(actual.curve.downcast_ref::<GeomBSplineCurve>().is_some());
    assert_eq!([actual.first, actual.last], receipt.span.parameters);
    assert!(receipt.trace.position_error_upper <= POLICY.trace.max_position_error);
    assert!(receipt.evaluations > 0);
}

#[test]
fn pitch_table_retains_ordered_c0_edges_with_shared_literal_native_station() {
    let law = table();
    let result = law.fit(POLICY, || false).unwrap();
    let detached = result.export_brep(|| false).unwrap();
    let [left, right] = &*detached.edges else {
        panic!("two table rows")
    };
    let [left_receipt, right_receipt] = result.receipts() else {
        panic!("two trace receipts")
    };
    check_trim(left, *left_receipt);
    check_trim(right, *right_receipt);
    assert!(left_receipt.trace.axial_derivative_lower > 0.);
    assert!(right_receipt.trace.axial_derivative_lower > 0.);
    let (_, join) = topexp::vertices(left, true);
    let (same_join, last) = topexp::vertices(right, true);
    assert!(join.is_same(&same_join));
    near(BRepTool::pnt(&join).unwrap().z(), 2.);
    let last = BRepTool::pnt(&last).unwrap();
    near(last.x(), -4.);
    near(last.z(), 4.);

    let left_curve = BRepTool::curve_with_location(left).unwrap();
    let right_curve = BRepTool::curve_with_location(right).unwrap();
    let a = left_curve.curve.get().unwrap().eval_d1(TAU).unwrap();
    let b = right_curve.curve.get().unwrap().eval_d1(TAU).unwrap();
    verify_clamped_join(left, right, a.point, b.point);
    assert!((a.d1.z() - FRAC_1_PI).abs() < 1e-10);
    assert!((b.d1.z() - 2. * FRAC_1_PI).abs() < 1e-10);
}

#[test]
fn flat_wire_retains_exact_planar_coefficients_and_native_axial_join_behavior() {
    let law = HelixSequence::flat_ends(two_flat_ends()).unwrap();
    let result = law.fit(POLICY, || false).unwrap();
    let detached = result.export_brep(|| false).unwrap();
    assert_eq!(detached.edges.len(), 5);
    for (edge, receipt) in detached.edges.iter().zip(result.receipts()) {
        check_trim(edge, *receipt);
    }
    for (index, elevation) in [(0, 0.), (4, 2.)] {
        let actual = BRepTool::curve_with_location(&detached.edges[index]).unwrap();
        let spline = actual.curve.downcast_ref::<GeomBSplineCurve>().unwrap();
        assert!(spline.poles_array(|poles| poles.iter().all(|pole| pole.z() == elevation)));
    }
    for pair in detached.edges.windows(2) {
        let left = BRepTool::curve_with_location(&pair[0]).unwrap();
        let right = BRepTool::curve_with_location(&pair[1]).unwrap();
        assert_eq!(left.last, right.first);
        let a = left.curve.get().unwrap().eval_d2(left.last).unwrap();
        let b = right.curve.get().unwrap().eval_d2(right.first).unwrap();
        verify_clamped_join(&pair[0], &pair[1], a.point, b.point);
        assert!((a.d1.z() - b.d1.z()).abs() < 1e-10);
        assert!((a.d2.z() - b.d2.z()).abs() < 1e-10);
    }
    for (index, expected_z) in [(1, 3. / 64.), (3, 2. - 3. / 64.)] {
        let curve = BRepTool::curve_with_location(&detached.edges[index]).unwrap();
        let p = curve
            .curve
            .get()
            .unwrap()
            .d0(curve.first.midpoint(curve.last))
            .unwrap();
        assert!((p.z() - expected_z).abs() <= result.receipts()[index].trace.position_error_upper);
    }
}

#[test]
fn cancellation_limits_and_detached_vertex_mutation_are_isolated() {
    let law = table();
    assert!(matches!(
        law.fit(POLICY, || true),
        Err(HelixSequenceError::Helix(HelixFailure::Cancelled(
            HelixStage::Admission
        )))
    ));
    assert!(matches!(
        law.fit(HelixFitOptions { max_evaluations: 1, ..POLICY }, || false),
        Err(HelixSequenceError::SpanFailure { cause, .. }) if matches!(*cause, HelixFailure::EvaluationLimit { .. })
    ));
    let checked = law.fit(POLICY, || false).unwrap();
    let one = checked.export_brep(|| false).unwrap();
    let (_, join) = topexp::vertices(&one.edges[0], true);
    let (same_join, _) = topexp::vertices(&one.edges[1], true);
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&join, Point3::default(), 0.01)
        .unwrap();
    assert_eq!(BRepTool::pnt(&same_join).unwrap(), Point3::default());
    let other = checked.export_brep(|| false).unwrap();
    let (independent, _) = topexp::vertices(&other.edges[1], true);
    assert!(!independent.is_same(&same_join));
    near(BRepTool::pnt(&independent).unwrap().z(), 2.);
}

#[test]
fn flat_law_remains_evaluable_when_noncardinal_native_plane_is_refused() {
    let mut request = two_flat_ends();
    let frame = Axis2::new(Point3::default(), Direction3::new(1., 1., 1.).unwrap()).unwrap();
    request.placement.frame = frame;
    let law = HelixSequence::flat_ends(request).unwrap();
    assert!(law.evaluate(0., HelixParameterSide::After).is_ok());
    assert!(matches!(
        law.fit(POLICY, || false),
        Err(HelixSequenceError::NativeFlatFrame)
    ));
}

fn verify_clamped_join(left: &Edge, right: &Edge, left_point: Point3, right_point: Point3) {
    let a = BRepTool::curve_with_location(left).unwrap();
    let b = BRepTool::curve_with_location(right).unwrap();
    let a = a.curve.downcast_ref::<GeomBSplineCurve>().unwrap();
    let b = b.curve.downcast_ref::<GeomBSplineCurve>().unwrap();
    let a = a.pole(a.nb_poles()).unwrap();
    let b = b.pole(1).unwrap();
    let (_, station) = topexp::vertices(left, true);
    assert_eq!(a, b);
    assert_eq!(BRepTool::pnt(&station).unwrap(), a);
    for i in 0..3 {
        near(left_point.coordinates()[i], a.coordinates()[i]);
        near(right_point.coordinates()[i], b.coordinates()[i]);
    }
}

#[test]
fn ordered_native_helix_replays_across_threads_and_separate_processes() {
    use crate::authored_helix::replay::{assert_replay, native_wire_bytes};
    fn owned<T: Send + Sync>() {}
    owned::<(HelixSequence, FittedHelixSequence, DetachedHelixSequence)>();

    let law = HelixSequence::flat_ends(two_flat_ends()).unwrap();
    let baseline = law.fit(POLICY, || false).unwrap();
    let wire_bytes =
        |fit: &FittedHelixSequence| native_wire_bytes(&fit.export_brep(|| false).unwrap().wire);
    let expected = wire_bytes(&baseline);
    std::thread::scope(|scope| {
        let handles = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    assert_eq!(wire_bytes(&baseline), expected);
                    let fit = law.fit(POLICY, || false).unwrap();
                    assert_eq!(fit.receipts(), baseline.receipts());
                    assert_eq!(wire_bytes(&fit), expected);
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().unwrap();
        }
    });
    assert_replay(
        "ORDERED_HELIX_BREP=",
        "helix_sequence::native::ordered_native_helix_replays_across_threads_and_separate_processes",
        || expected,
    );
}
