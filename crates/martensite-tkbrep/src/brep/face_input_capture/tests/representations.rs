//! Stored seam cells and actual bindings are retained without curve evaluation.
use super::*;
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::geom::GeomLine;
use martensite_tkmath::gp::{Axis1, Axis2d};

pub(super) fn append_owner(face: &Face, owner: Handle<dyn TShape>) {
    let occurrence = Shape::from_parts(owner, Location::default(), Orientation::Forward);
    face.tshape()
        .get()
        .unwrap()
        .tshape_base()
        .append_child(occurrence);
}
pub(super) fn seam_edge(surface: &Handle<GeomSurfaceOfLinearExtrusion>) -> Handle<BRepTEdge> {
    let first = Handle::new(Geom2dLine::new(Axis2d::default()));
    let second = Handle::new(Geom2dLine::new(Axis2d::default()));
    let seam = BRepCurveOnClosedSurface::new(
        &first.cast(),
        &second.cast(),
        &surface.cast(),
        &Location::default(),
        martensite_tkmath::geom_abs::Shape::C0,
    )
    .unwrap();
    seam.parent.gcurve.store_range(2., 5.);
    let line = Handle::new(GeomLine::new(Axis1::default()));
    let spatial = Handle::new(BRepCurve3D::new(&line.cast(), &Location::default()));
    let edge = Handle::new(BRepTEdge::new());
    edge.change_curves(|curves| curves.extend([spatial.cast(), Handle::new(seam).cast()]));
    edge
}
#[test]
fn real_seam_retains_two_distinct_pcurve_bindings_and_actual_range() {
    let (face, fields) = face_fixture();
    let surface = fields.surface().cast::<GeomSurfaceOfLinearExtrusion>();
    append_owner(&face, seam_edge(&surface).cast());
    let snapshot = captured(&face);
    assert!(
        snapshot
            .cells()
            .iter()
            .any(|(_, cell)| matches!(cell, CapturedFaceCell::Range([2., 5.])))
    );
    let (first, second) = seam_bindings(&snapshot);
    assert_ne!(first, second);
    assert_eq!(planar_count(&snapshot), 2);
}
#[test]
fn nonempty_vertex_parameter_representations_refuse_instead_of_disappearing() {
    let (face, _) = face_fixture();
    let vertex = Handle::new(BRepTVertex::new());
    let line = Handle::new(GeomLine::new(Axis1::default()));
    let point = Handle::new(BRepPointOnCurve::new(
        0.,
        &line.cast(),
        &Location::default(),
    ));
    vertex.fields.write().points.push(point.cast());
    append_owner(&face, vertex.cast());
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::Unsupported { .. })
    ));
}
#[test]
fn nonfinite_actual_face_fields_are_reported_without_numerical_execution() {
    let (face, fields) = face_fixture();
    fields.set_tolerance(f64::NAN);
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::NonFinite {
            field: "face tolerance",
            ..
        })
    ));
}

fn seam_bindings(snapshot: &FaceInputSnapshot) -> (usize, usize) {
    let first = snapshot
        .cells()
        .iter()
        .find_map(|(_, cell)| match cell {
            CapturedFaceCell::Parametric { curve, .. } => Some(*curve),
            _ => None,
        })
        .unwrap();
    let second = snapshot
        .cells()
        .iter()
        .find_map(|(_, cell)| match cell {
            CapturedFaceCell::Seam { curve, .. } => Some(*curve),
            _ => None,
        })
        .unwrap();
    (first, second)
}

fn planar_count(snapshot: &FaceInputSnapshot) -> usize {
    snapshot
        .cells()
        .iter()
        .filter(|(_, cell)| matches!(cell, CapturedFaceCell::PlanarLine(_)))
        .count()
}

fn attached_seam(face: &Face, fields: &BRepTFace) -> Handle<BRepCurveOnClosedSurface> {
    let surface = fields.surface().cast::<GeomSurfaceOfLinearExtrusion>();
    let edge = seam_edge(&surface);
    let seam = edge.curves()[1].cast::<BRepCurveOnClosedSurface>();
    append_owner(face, edge.cast());
    seam
}
#[test]
fn embedded_range_writer_is_part_of_the_required_read_closure() {
    let (face, fields) = face_fixture();
    let seam = attached_seam(&face, &fields);
    let writer = seam.parent.gcurve.range.write();
    assert_busy(capture_closed_extrusion_face(
        &face,
        FaceCaptureLimits::bounded(),
        || false,
    ));
    drop(writer);
    assert!(fields.fields.try_write().is_some());
    assert!(capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false).is_ok());
}
#[test]
fn embedded_location_writer_is_part_of_the_required_read_closure() {
    let (face, fields) = face_fixture();
    let seam = attached_seam(&face, &fields);
    let writer = seam.parent.gcurve.representation.location.write();
    assert_busy(capture_closed_extrusion_face(
        &face,
        FaceCaptureLimits::bounded(),
        || false,
    ));
    drop(writer);
    assert!(fields.fields.try_write().is_some());
    assert!(capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false).is_ok());
}
