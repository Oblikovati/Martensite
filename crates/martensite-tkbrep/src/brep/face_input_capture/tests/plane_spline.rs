//! Actual plane and planar-spline references survive complete edge traversal.
use super::*;
use crate::brep::{BRepCurveOnSurface, BRepCurveRepresentation, BRepTEdge, BRepTFace};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::gp::{Axis2d, Axis3, Direction3, Plane3, Point2, Point3};

fn plane_face() -> (Face, Handle<BRepTFace>, Handle<GeomPlane>) {
    let plane = Handle::new(GeomPlane::new(Axis3::default()));
    let fields = Handle::new(BRepTFace::new());
    fields.set_surface(&plane.cast());
    let shape = Shape::from_parts(fields.cast(), Location::default(), Orientation::Forward);
    (crate::topods::face(&shape).unwrap(), fields, plane)
}

fn planar_spline() -> Handle<Geom2dBSplineCurve> {
    let pole_values = [
        Point2::new(0., 0.),
        Point2::new(1., 2.),
        Point2::new(3., 1.),
    ];
    let poles = Array1::from_shared(&pole_values, 1);
    let weights = Array1::from_shared(&[1., 0.5, 2.], 1);
    let knots = Array1::from_shared(&[2., 5.], 1);
    let mults = Array1::from_shared(&[3, 3], 1);
    Handle::new(
        Geom2dBSplineCurve::with_weights(&poles, &weights, &knots, &mults, 2, false).unwrap(),
    )
}

fn overridden_planar_spline() -> Handle<Geom2dBSplineCurve> {
    let spline = planar_spline();
    let replacement = Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    let descriptor = martensite_tkg2d::geom2d_eval::RepCurveDescFull::new(replacement);
    spline
        .set_eval_representation(Handle::new(descriptor).cast())
        .unwrap();
    spline
}

fn append_curve(
    edge: &Handle<BRepTEdge>,
    curve: Handle<dyn Geom2dCurve>,
    plane: &Handle<GeomPlane>,
) {
    let surface: Handle<dyn GeomSurface> = plane.clone().cast();
    let rep = BRepCurveOnSurface::new(&curve, &surface, &Location::default()).unwrap();
    edge.change_curves(|curves| {
        curves.push(Handle::new(rep).cast::<dyn BRepCurveRepresentation>())
    });
}

#[test]
fn plane_and_every_planar_representation_are_captured_with_aliases_preserved() {
    let (face, _, plane) = plane_face();
    let spline = planar_spline();
    let line = Handle::new(Geom2dLine::new(Axis2d::default()));
    let edge = Handle::new(BRepTEdge::new());
    append_curve(&edge, spline.clone().cast(), &plane);
    append_curve(&edge, line.cast(), &plane);
    append_curve(&edge, spline.cast(), &plane);
    super::representations::append_owner(&face, edge.cast());

    let snapshot = captured(&face);
    assert_eq!(
        snapshot
            .cells()
            .iter()
            .filter(|(_, cell)| matches!(cell, CapturedFaceCell::Plane(_)))
            .count(),
        1
    );
    assert_eq!(
        snapshot
            .cells()
            .iter()
            .filter(|(_, cell)| matches!(cell, CapturedFaceCell::PlanarSpline(_)))
            .count(),
        1
    );
    assert_eq!(
        snapshot
            .cells()
            .iter()
            .filter(|(_, cell)| matches!(cell, CapturedFaceCell::PlanarLine(_)))
            .count(),
        1
    );
    assert!(snapshot.cells().iter().any(|(_, cell)| matches!(
        cell,
        CapturedFaceCell::PlanarSpline(input)
            if input.poles() == [[0., 0.], [1., 2.], [3., 1.]]
                && input.weights() == [1., 0.5, 2.]
                && input.knots() == [2., 5.]
                && input.multiplicities() == [3, 3]
                && input.flat_knots() == [2., 2., 2., 5., 5., 5.]
                && input.domain() == [2., 5.]
                && input.lower_bounds() == [1; 5]
    )));
    let mut bindings = snapshot.cells().iter().filter_map(|(_, cell)| match cell {
        CapturedFaceCell::Parametric { curve, .. } => Some(*curve),
        _ => None,
    });
    let first = bindings.next().unwrap();
    let second = bindings.next().unwrap();
    let third = bindings.next().unwrap();
    assert_eq!(bindings.next(), None);
    assert_eq!(first, third);
    assert_ne!(first, second);
    assert!(snapshot
        .cells()
        .iter()
        .any(|(_, cell)| matches!(cell, CapturedFaceCell::Edge { representations, .. } if representations.len() == 3)));
}

#[test]
fn nonfinite_planar_spline_input_is_reported_without_evaluation() {
    let (face, _, plane) = plane_face();
    let spline = planar_spline();
    let edge = Handle::new(BRepTEdge::new());
    append_curve(&edge, spline.clone().cast(), &plane);
    super::representations::append_owner(&face, edge.cast());
    spline.set_pole(1, Point2::new(f64::NAN, 0.)).unwrap();
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::NonFinite {
            field: "planar spline pole",
            ..
        })
    ));
}

#[test]
fn planar_spline_array_sizes_are_checked_before_owned_copy() {
    let (face, _, plane) = plane_face();
    let spline = planar_spline();
    let edge = Handle::new(BRepTEdge::new());
    append_curve(&edge, spline.cast(), &plane);
    super::representations::append_owner(&face, edge.cast());
    let limits = FaceCaptureLimits {
        owners: 20,
        cells: 40,
        entries: 10,
    };
    assert!(matches!(
        capture_closed_extrusion_face(&face, limits, || false),
        Err(FaceCaptureIncomplete::Limit {
            dimension: "entries",
            ..
        })
    ));
}

#[test]
fn captured_plane_and_spline_are_detached_and_thread_safe() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<FaceInputSnapshot>();
    let (face, _, plane) = plane_face();
    let spline = planar_spline();
    let edge = Handle::new(BRepTEdge::new());
    append_curve(&edge, spline.clone().cast(), &plane);
    super::representations::append_owner(&face, edge.cast());
    let snapshot = captured(&face);
    let captured_pole = snapshot.cells().iter().find_map(|(_, cell)| match cell {
        CapturedFaceCell::PlanarSpline(input) => Some(input.poles()[0]),
        _ => None,
    });
    spline.set_pole(1, Point2::new(9., 8.)).unwrap();
    plane.set_pln(Plane3::new(Point3::new(4., 5., 6.), Direction3::Z).unwrap());
    assert_eq!(captured_pole, Some([0., 0.]));
    assert_eq!(spline.pole(1).unwrap(), Point2::new(9., 8.));
    assert!(snapshot
        .cells()
        .iter()
        .any(|(_, cell)| matches!(cell, CapturedFaceCell::Plane(axis) if axis.location() == Point3::default())));
}

#[test]
fn active_planar_evaluation_override_is_explicitly_incomplete() {
    let (face, _, plane) = plane_face();
    let spline = overridden_planar_spline();
    let edge = Handle::new(BRepTEdge::new());
    append_curve(&edge, spline.cast(), &plane);
    super::representations::append_owner(&face, edge.cast());
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::Unsupported {
            actual,
            expected: "stored planar B-spline without an external evaluator override",
            ..
        }) if actual == "active planar spline evaluation override"
    ));
}

#[test]
fn nonfinite_plane_origin_is_reported_without_geometry_evaluation() {
    let (face, _, plane) = plane_face();
    plane.set_pln(Plane3::new(Point3::new(f64::NAN, 0., 0.), Direction3::Z).unwrap());
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::NonFinite {
            field: "plane frame",
            ..
        })
    ));
}
