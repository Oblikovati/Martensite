//! `pcurve`, `2dtranslate` and `checklength`: the two-dimensional curves of edges on a face.
use crate::support::{
    commands::{bound_shape, distinct_parts},
    session::Session,
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkernel::{standard::Handle, storage::format_general};
use martensite_tkg2d::{
    geom2d::{Geom2dCurve, Geom2dTrimmedCurve},
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::gp::{Transform2, Vector2};

/// `shape` itself when it is of `kind`, else its first part of that kind.
fn first_part(shape: &Shape, kind: ShapeEnum) -> Shape {
    if shape.shape_type().unwrap() == kind {
        return shape.clone();
    }
    distinct_parts(shape, kind)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("pcurve: expected a part of kind {kind:?}"))
}

/// The trimmed p-curve of `edge` on `face`.
fn trimmed_pcurve(edge: &Shape, face: &Shape) -> Handle<dyn Geom2dCurve> {
    let (edge, face) = (topods::edge(edge).unwrap(), topods::face(face).unwrap());
    let found = BRepTool::curve_on_face(&edge, &face)
        .unwrap_or_else(|failure| panic!("pcurve: {failure:?}"));
    let trimmed = Geom2dTrimmedCurve::new(&found.pcurve, found.first, found.last, true, true)
        .unwrap_or_else(|failure| panic!("pcurve {} {}: {failure:?}", found.first, found.last));
    Handle::new(trimmed).cast()
}

/// `pcurve name edge face`, for the first edge in `edges`; `pcurve_of_edge(s, "ee", "ff")`.
pub fn pcurve_of_edge(s: &dyn Session, edges: &str, faces: &str) -> Handle<dyn Geom2dCurve> {
    let edge = first_part(&bound_shape(s, edges), ShapeEnum::Edge);
    let face = first_part(&bound_shape(s, faces), ShapeEnum::Face);
    trimmed_pcurve(&edge, &face)
}

/// `pcurve face`: its legend, then each edge's p-curve, in the order of the explorer.
/// Example: `let curves = pcurves_of_face(s, "ff1");`.
pub fn pcurves_of_face(s: &dyn Session, name: &str) -> Vec<Handle<dyn Geom2dCurve>> {
    println!("rouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL");
    let face = first_part(&bound_shape(s, name), ShapeEnum::Face);
    let mut curves = Vec::new();
    let mut explorer =
        martensite_tkbrep::topexp::Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        curves.push(trimmed_pcurve(explorer.current(), &face));
        explorer.next().unwrap();
    }
    curves
}

/// `2dtranslate curve dx dy`; `translate_pcurve(&curve, std::f64::consts::PI, 0.)`.
pub fn translate_pcurve(
    curve: &Handle<dyn Geom2dCurve>,
    dx: f64,
    dy: f64,
) -> Handle<dyn Geom2dCurve> {
    let mut transform = Transform2::default();
    transform.set_translation(Vector2::new(dx, dy));
    curve
        .get()
        .unwrap()
        .transformed(transform)
        .unwrap_or_else(|failure| panic!("2dtranslate {dx} {dy}: {failure:?}"))
        .cast()
}

/// `checklength name -l expected`: 1% of relative difference, as the procedure allows.
/// Example: `checklength("ff1_6", &curve, 7.167)`.
pub fn checklength(name: &str, curve: &Handle<dyn Geom2dCurve>, expected: f64) {
    let adaptor = Geom2dAdaptorCurve::with_curve(curve).unwrap();
    let length = AbscissaPoint::length_with_tolerance(&adaptor, 1.0e-4).unwrap();
    println!("The length {name} is {}", format_general(length, 17));
    println!("The expected length is {expected}");
    if length == 0. || ((expected - length) / expected).abs() > 0.01 {
        println!(
            "Error : The length of result shape is {}",
            format_general(length, 17)
        );
    }
}
