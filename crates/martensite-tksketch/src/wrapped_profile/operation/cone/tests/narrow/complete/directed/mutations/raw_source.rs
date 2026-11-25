//! #562: equivalent raw sources cannot reuse an already issued native receipt.
use super::*;
use martensite_tkbo::body_check::{
    CertifiedConicalBody, ConicalNativeMember, StageVerdict, capture_conical_actual_family,
    validate_body_with_certified_conical,
};
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Edge, Face},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::{
    geom::{GeomBezierCurve, GeomCurve},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::Point2;
pub(super) fn check(
    result: &WrappedConeNormalResult,
    body: &CertifiedConicalBody,
    members: &[ConicalNativeMember],
) {
    let face = &members[0].sides[0].1;
    let edges = edges(face);
    range_zero(result, body, members, face, &edges[0]);
    planar_changes(result, body, members, face, &edges[0]);
    ruling_direction(body, members, &edges);
    body.rebind_family(members).unwrap();
}
fn edges(face: &Face) -> Vec<Edge> {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let mut edges = Vec::new();
    while explorer.more() {
        edges.push(topods::edge(explorer.current()).unwrap());
        explorer.next().unwrap();
    }
    edges
}
fn range_zero(
    result: &WrappedConeNormalResult,
    body: &CertifiedConicalBody,
    members: &[ConicalNativeMember],
    face: &Face,
    edge: &Edge,
) {
    let original = BRepTool::curve_on_face(edge, face).unwrap();
    assert_eq!(original.first.to_bits(), 0_f64.to_bits());
    let builder = BRepBuilder::default();
    builder
        .range_on_face(edge, face, -0., original.last)
        .unwrap();
    assert_eq!(
        BRepTool::curve_on_face(edge, face).unwrap().first.to_bits(),
        (-0_f64).to_bits()
    );
    refused(body, members);
    ordered_refused(result, body, &members[0]);
    builder
        .range_on_face(edge, face, original.first, original.last)
        .unwrap();
    body.rebind_family(members).unwrap();
}
fn planar_changes(
    result: &WrappedConeNormalResult,
    body: &CertifiedConicalBody,
    members: &[ConicalNativeMember],
    face: &Face,
    edge: &Edge,
) {
    let original = BRepTool::curve_on_face(edge, face).unwrap();
    let handle: Handle<Geom2dBezierCurve> = original.pcurve.cast();
    let curve = handle.get().unwrap();
    let changed = Geom2dBezierCurve::copy_of(curve).unwrap();
    flip_zero(&changed);
    attach(edge, face, Handle::new(changed).cast());
    refused(body, members);
    ordered_refused(result, body, &members[0]);
    let reversed = Geom2dBezierCurve::copy_of(curve).unwrap();
    reversed.reverse().unwrap();
    attach(edge, face, Handle::new(reversed).cast());
    refused(body, members);
    attach(edge, face, original.pcurve);
    body.rebind_family(members).unwrap();
}
fn flip_zero(curve: &Geom2dBezierCurve) {
    for index in 1..=curve.nb_poles() {
        if let Some(point) = flipped(curve.pole(index).unwrap()) {
            curve.set_pole(index, point).unwrap();
            return;
        }
    }
    panic!("admitted unit-rectangle side has no zero coefficient");
}
fn flipped(point: Point2) -> Option<Point2> {
    if point.x() == 0. {
        return Some(Point2::new(
            f64::from_bits(point.x().to_bits() ^ (1_u64 << 63)),
            point.y(),
        ));
    }
    if point.y() == 0. {
        return Some(Point2::new(
            point.x(),
            f64::from_bits(point.y().to_bits() ^ (1_u64 << 63)),
        ));
    }
    None
}

fn attach(edge: &Edge, face: &Face, curve: Handle<dyn Geom2dCurve>) {
    let tolerance = BRepTool::edge_tolerance(edge).unwrap();
    BRepBuilder::default()
        .update_edge_pcurve_on_face(edge, &curve, face, tolerance)
        .unwrap();
}
fn ruling_direction(body: &CertifiedConicalBody, members: &[ConicalNativeMember], edges: &[Edge]) {
    let (edge, handle) = ruling(edges);
    let original = BRepTool::curve_with_location(edge).unwrap();
    let changed = GeomBezierCurve::copy_of(handle.get().unwrap()).unwrap();
    changed.reverse().unwrap();
    let tolerance = BRepTool::edge_tolerance(edge).unwrap();
    let builder = BRepBuilder::default();
    builder
        .update_edge_curve(edge, &Handle::new(changed).cast(), tolerance)
        .unwrap();
    refused(body, members);
    builder
        .update_edge_located_curve(edge, &original.curve, &original.location, tolerance)
        .unwrap();
    builder
        .range(edge, original.first, original.last, true)
        .unwrap();
    body.rebind_family(members).unwrap();
}
fn ruling(edges: &[Edge]) -> (&Edge, Handle<GeomBezierCurve>) {
    for edge in edges {
        let source = BRepTool::curve_with_location(edge).unwrap();
        let curve: Handle<GeomBezierCurve> = source.curve.cast();
        if curve.get().unwrap().nb_poles() == 2 {
            return (edge, curve);
        }
    }
    panic!("actual side lacks its admitted two-pole ruling");
}
fn refused(body: &CertifiedConicalBody, members: &[ConicalNativeMember]) {
    // Equivalent geometric admission remains available; only the old authority is invalid.
    assert!(capture_conical_actual_family(members).is_ok());
    assert!(body.rebind_family(members).is_err());
}
fn ordered_refused(
    result: &WrappedConeNormalResult,
    body: &CertifiedConicalBody,
    member: &ConicalNativeMember,
) {
    let resolution = pipeline::resolution(result.options().spatial_error);
    let report = validate_body_with_certified_conical(member, &resolution, body).unwrap();
    assert_eq!(
        report.stage(ValidationStage::Topology).verdict,
        StageVerdict::Passed
    );
    assert_eq!(
        report.stage(ValidationStage::Geometry).verdict,
        StageVerdict::Passed
    );
    assert_eq!(
        report.stage(ValidationStage::SelfIntersection).verdict,
        StageVerdict::Inconclusive
    );
    assert_eq!(
        report.stage(ValidationStage::Tolerance).verdict,
        StageVerdict::NotRun
    );
}
