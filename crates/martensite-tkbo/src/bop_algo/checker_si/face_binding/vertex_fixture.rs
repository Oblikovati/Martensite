// SPDX-License-Identifier: GPL-2.0-only
//! Named constructive endpoint parents: no coordinate lookup establishes identity.
use super::super::super::*;
use super::super::capture_tests;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTVertex},
    topods::{self, Builder, Edge, Face, Vertex, Wire},
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::{
    gp::{Point3, Transform3, Vector3},
    toploc::Location,
};

pub(super) fn packet(raw_tolerance: f64, offset: f64) -> OwnedFacePacket {
    construct(raw_tolerance, offset, UpperIdentity::Shared)
}
#[derive(Clone, Copy)]
enum UpperIdentity {
    Shared,
    DistinctVertex,
    IdentityDatum,
}
pub(super) fn split_vertex() -> OwnedFacePacket {
    construct(0.25, 0., UpperIdentity::DistinctVertex)
}
pub(super) fn split_location() -> OwnedFacePacket {
    construct(0.25, 0., UpperIdentity::IdentityDatum)
}
fn construct(raw_tolerance: f64, offset: f64, upper_identity: UpperIdentity) -> OwnedFacePacket {
    let basis = capture_tests::basis();
    let mut face = capture_tests::face_on_basis(&basis);
    let mut seam = capture_tests::make_seam(&face);
    let mut top = capture_tests::make_cap(&face, &basis, 0.);
    let mut bottom = capture_tests::make_cap(&face, &basis, -2.);
    let upper = vertex(raw_tolerance, offset);
    let lower = lower(&upper);
    endpoints(&mut seam, &upper, &lower);
    let cap_upper = cap_upper(&upper, raw_tolerance, offset, upper_identity);
    endpoints(&mut top, &cap_upper, &cap_upper);
    endpoints(&mut bottom, &lower, &lower);
    wire(&mut face, seam, top, bottom);
    capture_face(&face).unwrap()
}
fn cap_upper(upper: &Vertex, tolerance: f64, offset: f64, identity: UpperIdentity) -> Vertex {
    match identity {
        UpperIdentity::Shared => upper.clone(),
        UpperIdentity::DistinctVertex => vertex(tolerance, offset),
        UpperIdentity::IdentityDatum => topods::vertex(
            &upper
                .located(&Location::new(Transform3::default()), false)
                .unwrap(),
        )
        .unwrap(),
    }
}

fn vertex(tolerance: f64, offset: f64) -> Vertex {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(1., offset, 0.), 0.25)
        .unwrap();
    vertex
        .tshape()
        .cast::<BRepTVertex>()
        .get()
        .unwrap()
        .set_tolerance(tolerance);
    vertex
}
fn lower(upper: &Vertex) -> Vertex {
    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(0., 0., -2.));
    topods::vertex(&upper.located(&Location::new(translation), false).unwrap()).unwrap()
}
fn endpoints(edge: &mut Edge, first: &Vertex, last: &Vertex) {
    Builder
        .add(edge, &first.oriented(Orientation::Forward))
        .unwrap();
    Builder
        .add(edge, &last.oriented(Orientation::Reversed))
        .unwrap();
}
fn wire(face: &mut Face, seam: Edge, top: Edge, bottom: Edge) {
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    for edge in [
        seam.clone().into_shape(),
        seam.reversed(),
        top.reversed(),
        bottom.into_shape(),
    ] {
        Builder.add(&mut wire, &edge).unwrap();
    }
    Builder.add(face, &wire).unwrap();
}
