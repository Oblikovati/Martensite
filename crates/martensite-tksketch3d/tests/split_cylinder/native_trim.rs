// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Vector2;
use std::f64::consts::TAU;

#[test]
fn literal_seam_gap_and_same_carrier_clipped_chart_are_refused() {
    for [du, dv] in [[TAU.next_down() - TAU, 0.], [0., 0.5]] {
        let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
        let seam = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .map(|item| topods::edge(&item).unwrap())
            .find(|edge| BRepTool::is_closed_on_face(edge, &face).unwrap())
            .unwrap();
        let directional = if face.orientation() == Orientation::Forward {
            Orientation::Forward
        } else {
            Orientation::Reversed
        };
        let seam = topods::edge(&seam.oriented(directional)).unwrap();
        let trace = BRepTool::curve_on_face(&seam, &face).unwrap();
        assert!(trace.is_stored);
        trace
            .pcurve
            .get()
            .unwrap()
            .translate(Vector2::new(du, dv))
            .unwrap();
        assert!(matches!(
            capture_cylindrical_split_domain(
                &body,
                &face,
                STAMP,
                FaceCaptureLimits::bounded(),
                BOUND,
                || false
            ),
            Err(FaceError::UnsupportedTrim { .. })
        ));
    }
}

#[test]
fn native_trace_limit_and_source_capture_budget_are_separate_refusals() {
    let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
    let limits = FaceCaptureLimits::bounded();
    assert!(matches!(
        capture_cylindrical_split_domain(&body, &face, STAMP, limits, 0., || false),
        Err(FaceError::TraceError { upper_bound, limit: 0. }) if upper_bound > 0.
    ));
    assert!(matches!(
        capture_cylindrical_split_domain(&body, &face, STAMP, limits, f64::INFINITY, || false),
        Err(FaceError::TraceLimit(value)) if value == f64::INFINITY
    ));
    let small = FaceCaptureLimits {
        entries: 1,
        ..limits
    };
    assert!(matches!(
        capture_cylindrical_split_domain(&body, &face, STAMP, small, BOUND, || false),
        Err(FaceError::Source(FaceAttachmentError::Capture(_)))
    ));
    assert_eq!(
        capture_cylindrical_split_domain(&body, &face, STAMP, limits, BOUND, || true).unwrap_err(),
        FaceError::Cancelled
    );
}

#[test]
fn a_degenerate_actual_rim_cannot_pass_even_with_a_wide_metric_limit() {
    let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
    let circle = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| topods::edge(&shape).unwrap())
        .map(|edge| {
            BRepTool::curve(&edge)
                .unwrap()
                .0
                .downcast::<martensite_tkg3d::geom::GeomCircle>()
        })
        .find(|circle| !circle.is_null())
        .unwrap();
    circle.get().unwrap().set_radius(0.).unwrap();
    assert!(matches!(
        capture_cylindrical_split_domain(
            &body,
            &face,
            STAMP,
            FaceCaptureLimits::bounded(),
            10.,
            || false
        ),
        Err(FaceError::UnsupportedTrim { .. })
    ));
}
