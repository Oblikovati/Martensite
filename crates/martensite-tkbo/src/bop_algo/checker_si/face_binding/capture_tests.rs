// SPDX-License-Identifier: GPL-2.0-only
//! Actual released BRep wrappers exercise capture, branch selection and isolation.
use super::{fixture, *};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{self, Builder, Edge, Face, Wire},
};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomCurve, GeomLine, GeomSurface, GeomSurfaceOfLinearExtrusion},
    top_abs::Orientation,
};
use martensite_tkmath::{
    gp::{Axis1, Axis2d, Direction2, Direction3, Point2, Point3, Transform3, Vector3},
    toploc::Location,
};

pub(super) fn basis() -> Handle<GeomBSplineCurve> {
    let owned = fixture::spline();
    let points = owned
        .poles
        .iter()
        .map(|p| Point3::new(p[0], p[1], p[2]))
        .collect::<Vec<_>>();
    Handle::new(
        GeomBSplineCurve::new(
            &Array1::from_shared(&points, 1),
            &Array1::from_shared(&owned.knots, 1),
            &Array1::from_shared(&owned.multiplicities, 1),
            2,
            false,
        )
        .unwrap(),
    )
}
fn line(origin: [f64; 2], direction: [f64; 2]) -> Handle<dyn Geom2dCurve> {
    Handle::new(Geom2dLine::new(Axis2d::new(
        Point2::new(origin[0], origin[1]),
        Direction2::new(direction[0], direction[1]).unwrap(),
    )))
    .cast()
}
fn face_fixture() -> (Face, Handle<GeomBSplineCurve>, Edge) {
    let basis = basis();
    let mut face = face_on_basis(&basis);
    let seam = make_seam(&face);
    let upper = make_cap(&face, &basis, 0.);
    let lower = make_cap(&face, &basis, -2.);
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    for edge in [
        seam.clone().into_shape(),
        seam.reversed(),
        upper.reversed(),
        lower.into_shape(),
    ] {
        Builder.add(&mut wire, &edge).unwrap();
    }
    Builder.add(&mut face, &wire).unwrap();
    (face, basis, seam)
}
pub(super) fn face_on_basis(basis: &Handle<GeomBSplineCurve>) -> Face {
    let surface = Handle::new(
        GeomSurfaceOfLinearExtrusion::new(&basis.cast::<dyn GeomCurve>(), Direction3::Z).unwrap(),
    )
    .cast::<dyn GeomSurface>();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &surface, 0.25)
        .unwrap();
    face
}

pub(super) fn make_seam(face: &Face) -> Edge {
    let curve = Handle::new(GeomLine::new(Axis1::new(
        Point3::new(1., 0., 0.),
        Direction3::new(0., 0., -1.).unwrap(),
    )))
    .cast::<dyn GeomCurve>();
    let mut edge = Edge::default();
    BRepBuilder::default()
        .make_edge_with_curve(&mut edge, &curve, 0.25)
        .unwrap();
    BRepBuilder::default()
        .update_edge_pcurves_on_face(
            &edge,
            (&line([0., 0.], [0., -1.]), &line([4., 0.], [0., -1.])),
            face,
            0.25,
        )
        .unwrap();
    BRepBuilder::default().range(&edge, 0., 2., false).unwrap();
    edge
}
pub(super) fn make_cap(face: &Face, basis: &Handle<GeomBSplineCurve>, v: f64) -> Edge {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(0., 0., v));
    let mut edge = Edge::default();
    BRepBuilder::default()
        .make_edge_with_located_curve(&mut edge, &basis.cast(), &Location::new(transform), 0.25)
        .unwrap();
    BRepBuilder::default()
        .update_edge_pcurve_on_face(&edge, &line([0., v], [1., 0.]), face, 0.25)
        .unwrap();
    BRepBuilder::default().range(&edge, 0., 4., false).unwrap();
    edge
}
#[test]
fn forward_and_reversed_actual_faces_have_same_canonical_strip_proof() {
    let (face, _, _) = face_fixture();
    let forward = capture_face(&face).unwrap();
    let reversed = capture_face(&topods::face(&face.reversed()).unwrap()).unwrap();
    assert!(
        inspect_owned_strip(&forward, || false)
            .prerequisites
            .is_ok()
    );
    assert!(
        inspect_owned_strip(&reversed, || false)
            .prerequisites
            .is_ok()
    );
    assert_eq!(forward.edges[0].stored_branch, 1);
    assert_eq!(reversed.edges[0].stored_branch, 1);
    assert_eq!(forward.face_orientation, Orientation::Forward);
    assert_eq!(reversed.face_orientation, Orientation::Reversed);
}
#[test]
fn captured_numeric_arrays_do_not_follow_mutation_of_actual_edge_source() {
    let (face, basis, _) = face_fixture();
    let packet = capture_face(&face).unwrap();
    basis
        .get()
        .unwrap()
        .set_pole(2, Point3::new(20., 20., 20.))
        .unwrap();
    assert!(inspect_owned_strip(&packet, || false).prerequisites.is_ok());
    assert!(
        inspect_owned_strip(&capture_face(&face).unwrap(), || false)
            .prerequisites
            .is_err()
    );
}
#[test]
fn missing_stored_seam_curve_refuses_without_projection_substitution() {
    let (face, _, seam) = face_fixture();
    BRepBuilder::default()
        .update_edge_pcurve_on_face(&seam, &Handle::default(), &face, 0.25)
        .unwrap();
    assert!(capture_face(&face).is_err());
}
#[test]
fn different_location_seam_use_cannot_be_mistaken_for_same_topological_seam() {
    let (face, _, seam) = face_fixture();
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(1., 0., 0.));
    let moved = topods::edge(&seam.located(&Location::new(transform), false).unwrap()).unwrap();
    let relations = super::super::capture_edges::relations(&[seam, moved]);
    assert!(!relations[0].same);
    assert!(!relations[0].location_equal);
    assert!(capture_face(&face).is_ok());
}

#[test]
fn actual_parent_and_edge_identity_survives_canonical_occurrence_capture() {
    let (face, _, seam) = face_fixture();
    let packet = capture_face(&face).unwrap();
    let first = &packet.edges[0].context;
    let second = &packet.edges[1].context;
    assert_eq!(first.face.tshape_token, face.tshape_address());
    assert_eq!(first.edge.tshape_token, seam.tshape_address());
    assert_eq!(first.edge.tshape_token, second.edge.tshape_token);
    assert_eq!(first.edge.location_datums, second.edge.location_datums);
    assert_ne!(first.edge.orientation, second.edge.orientation);
    assert_eq!(first.wire.tshape_token, second.wire.tshape_token);
    assert_eq!((first.edge_position, second.edge_position), (0, 1));
}
