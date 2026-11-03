//! Expanded stored-input capture is explicit and retains every support reference.
use super::*;
use martensite_tkernel::ncollection::Array2;
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomPlane};
use martensite_tkmath::{geom_abs::Shape as Continuity, gp::Axis3};

fn tensor_poles() -> Array2<'static, Point3> {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for u in 1..=2 {
        for v in 1..=2 {
            poles
                .set_value(u, v, Point3::new(f64::from(u), f64::from(v), 0.))
                .unwrap();
        }
    }
    poles
}
fn tensor() -> Handle<GeomBSplineSurface> {
    let poles = tensor_poles();
    let knots = Array1::from_shared(&[0., 1.], 1);
    let mults = Array1::from_shared(&[2, 2], 1);
    Handle::new(
        GeomBSplineSurface::new(
            &poles,
            (&knots, &mults),
            (&knots, &mults),
            (1, 1),
            (false, false),
        )
        .unwrap(),
    )
}
fn surface_face() -> (Face, Handle<BRepTFace>, Handle<GeomBSplineSurface>) {
    let (face, fields) = face_fixture();
    let surface = tensor();
    fields.set_surface(&surface.cast());
    (face, fields, surface)
}
fn query(face: &Face) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    capture_face_input_with_behavior(
        face,
        FaceCaptureLimits::bounded(),
        FaceCaptureBehavior::StoredSurfaceQueriesV2,
        || false,
    )
}
#[test]
fn tensor_capture_requires_explicit_new_behavior_and_preserves_owned_bits() {
    let (face, _, surface) = surface_face();
    assert!(matches!(
        capture_face_input(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::Unsupported { .. })
    ));
    let snapshot = query(&face).unwrap();
    surface.set_pole(1, 1, Point3::new(9., 8., 7.)).unwrap();
    let input = snapshot
        .cells()
        .iter()
        .find_map(|(_, cell)| match cell {
            CapturedFaceCell::SplineSurface(input) => Some(input),
            _ => None,
        })
        .unwrap();
    assert_eq!(input.poles()[0], [1., 1., 0.]);
    assert_eq!(input.axes()[0].flat_knots(), &[0., 0., 1., 1.]);
}
fn regularity(face: &Face, surface: &Handle<GeomBSplineSurface>) -> Handle<BRepCurveOn2Surfaces> {
    let plane = Handle::new(GeomPlane::new(Axis3::default()));
    let record = Handle::new(BRepCurveOn2Surfaces::new(
        &surface.cast(),
        &plane.cast(),
        &Location::default(),
        &Location::default(),
        Continuity::G1,
    ));
    let edge = Handle::new(BRepTEdge::new());
    edge.change_curves(|curves| curves.extend([record.cast(), record.cast()]));
    representations::append_owner(face, edge.cast());
    record
}
#[test]
fn regularity_retains_both_supports_and_duplicate_stored_uses() {
    let (face, _, surface) = surface_face();
    let _record = regularity(&face, &surface);
    let snapshot = query(&face).unwrap();
    let supports = regularity_supports(&snapshot);
    assert_ne!(supports[0], supports[1]);
    assert!(snapshot.cells().iter().any(|(_, cell)| matches!(cell, CapturedFaceCell::Edge { representations, .. } if representations.len() == 2 && representations[0] == representations[1])));
    assert_support_types(&snapshot, supports);
}
fn regularity_supports(snapshot: &FaceInputSnapshot) -> [usize; 2] {
    snapshot
        .cells()
        .iter()
        .find_map(|(_, cell)| match cell {
            CapturedFaceCell::Regularity {
                surfaces,
                continuity,
                ..
            } => {
                assert_eq!(*continuity, Continuity::G1);
                Some(*surfaces)
            }
            _ => None,
        })
        .unwrap()
}
fn assert_support_types(snapshot: &FaceInputSnapshot, supports: [usize; 2]) {
    assert!(
        snapshot
            .cells()
            .iter()
            .any(|(id, cell)| *id == supports[0]
                && matches!(cell, CapturedFaceCell::SplineSurface(_)))
    );
    assert!(
        snapshot
            .cells()
            .iter()
            .any(|(id, cell)| *id == supports[1] && matches!(cell, CapturedFaceCell::Plane(_)))
    );
}

#[test]
fn regularity_writer_refuses_complete_capture_and_releases_other_leases() {
    let (face, fields, surface) = surface_face();
    let record = regularity(&face, &surface);
    let writer = record.fields.write();
    assert!(matches!(
        query(&face),
        Err(FaceCaptureIncomplete::Read {
            cause: SharedReadFailure::Busy,
            ..
        })
    ));
    assert!(fields.fields.try_write().is_some());
    drop(writer);
    assert!(query(&face).is_ok());
}
#[test]
fn expanded_capture_still_obeys_declared_entry_budget_and_cancellation() {
    let (face, _, _) = surface_face();
    let limits = FaceCaptureLimits {
        entries: 1,
        ..FaceCaptureLimits::bounded()
    };
    assert!(matches!(
        capture_face_input_with_behavior(
            &face,
            limits,
            FaceCaptureBehavior::StoredSurfaceQueriesV2,
            || false
        ),
        Err(FaceCaptureIncomplete::Limit { .. })
    ));
    assert!(matches!(
        cancelled_query(&face, limits),
        Err(FaceCaptureIncomplete::Cancelled)
    ));
}

fn cancelled_query(
    face: &Face,
    limits: FaceCaptureLimits,
) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    capture_face_input_with_behavior(
        face,
        limits,
        FaceCaptureBehavior::StoredSurfaceQueriesV2,
        || true,
    )
}
#[test]
fn old_capture_keeps_regularity_outside_its_admitted_domain() {
    let (face, _) = face_fixture();
    let _record = regularity(&face, &tensor());
    assert!(matches!(
        capture_face_input(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::Unsupported { .. })
    ));
    assert!(query(&face).is_ok());
}
#[test]
fn finite_linear_spatial_spline_is_explicitly_new_input_capture_only() {
    let (face, _) = face_fixture();
    let curve = linear_spline();
    let representation = Handle::new(BRepCurve3D::new(&curve.cast(), &Location::default()));
    let edge = Handle::new(BRepTEdge::new());
    edge.change_curves(|curves| curves.push(representation.cast()));
    representations::append_owner(&face, edge.cast());
    assert!(capture_face_input(&face, FaceCaptureLimits::bounded(), || false).is_err());
    assert!(
        query(&face)
            .unwrap()
            .cells()
            .iter()
            .any(|(_, cell)| matches!(cell,CapturedFaceCell::Spline(input) if input.degree() == 1))
    );
}
fn linear_spline() -> Handle<GeomBSplineCurve> {
    let poles = Array1::from_shared(&[Point3::default(), Point3::new(1., 0., 0.)], 1);
    let knots = Array1::from_shared(&[0., 1.], 1);
    let mults = Array1::from_shared(&[2, 2], 1);
    Handle::new(GeomBSplineCurve::new(&poles, &knots, &mults, 1, false).unwrap())
}
