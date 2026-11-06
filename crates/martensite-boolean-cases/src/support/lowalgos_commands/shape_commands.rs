//! The `lowalgos` commands on shapes: classifying a point, plane curves of a face, tolerances,
//! and the small shape commands that only these scripts call.
use super::{Curve, Session};
use crate::support::session::Drawing;
use martensite_raster_core::format_scientific;
use martensite_tkbo::int_tools::FClass2d;
use martensite_tkbrep::{
    bintools,
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{
    message::ProgressRange, precision, standard::Handle, storage::IoStream, storage::format_general,
};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
use martensite_tkgeombase::geom_tools::CurveSet;
use martensite_tkmath::gp::{Axis2, Direction3, Point2, Point3, Transform3};
use martensite_tkshhealing::{
    shape_fix::ShapeFixShapeTolerance, shape_upgrade::ShapeUpgradeRemoveLocations,
};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_class::BRepClassFaceClassifier,
    brep_extrema::BRepExtremaDistShapeShape,
};
use std::io::{Read, Seek, SeekFrom};

fn bound_face(s: &dyn Session, name: &str) -> topods::Face {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("{name}: expected a bound face"));
    topods::face(shape).unwrap_or_else(|failure| panic!("{name}: {failure:?}"))
}

fn point(at: [f64; 3]) -> Point3 {
    Point3::new(at[0], at[1], at[2])
}

fn direction(along: [f64; 3]) -> Direction3 {
    Direction3::new(along[0], along[1], along[2])
        .unwrap_or_else(|failure| panic!("direction {along:?}: {failure:?}"))
}

fn state_answer(state: State) -> &'static str {
    match state {
        State::In => "The point is IN shape",
        State::Out => "The point is OUT of shape",
        State::On => "The point is ON shape",
        State::Unknown => "The point is UNKNOWN shape",
    }
}

/// `b2dclassifx face point`: its answer; `b2dclassifx(s, "b1", point)`.
pub fn b2dclassifx(s: &dyn Session, name: &str, at: Point2) -> &'static str {
    let face = bound_face(s, name);
    let tolerance = BRepTool::face_tolerance(&face).unwrap();
    let classifier = FClass2d::new(&face, tolerance).unwrap();
    let answer = state_answer(classifier.perform(at, true).unwrap());
    println!("{answer}");
    answer
}

/// `b2dclassify face point`: its answer; `b2dclassify(s, "b1", point)`.
pub fn b2dclassify(s: &dyn Session, name: &str, at: Point2) -> &'static str {
    let face = bound_face(s, name);
    let tolerance = BRepTool::face_tolerance(&face).unwrap();
    let mut classifier = BRepClassFaceClassifier::new();
    classifier
        .perform(&face, &at, tolerance, false, 0.1)
        .unwrap();
    let answer = state_answer(classifier.state());
    println!("{answer}");
    answer
}

fn pcurve_geometry(
    curve: &Handle<dyn Geom2dCurve>,
    first: f64,
    last: f64,
) -> Handle<dyn Geom2dCurve> {
    let trimmed = curve.cast::<Geom2dTrimmedCurve>();
    let basis = trimmed.get().map(Geom2dTrimmedCurve::basis_curve);
    let geometry = basis.as_ref().unwrap_or(curve).get().unwrap();
    let (from, to) = (geometry.first_parameter(), geometry.last_parameter());
    let beyond = from - first > precision::p_confusion() || last - to > precision::p_confusion();
    if !geometry.is_periodic() && beyond {
        return curve.clone();
    }
    Handle::new(Geom2dTrimmedCurve::new(curve, first, last, true, true).unwrap()).cast()
}

/// `pcurve face`: the color legend, then each edge's curve in the plane as `face_N`.
/// Example: `pcurve(s, "b1")`.
pub fn pcurve(s: &mut dyn Session, name: &str) -> Vec<Handle<dyn Geom2dCurve>> {
    println!("rouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL");
    let face = bound_face(s, name);
    let forward = topods::face(&face.oriented(Orientation::Forward)).unwrap();
    let mut edges = Explorer::new(forward.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let (mut index, mut curves) = (1, Vec::new());
    while edges.more() {
        let edge = topods::edge(edges.current()).unwrap();
        let on_face = BRepTool::curve_on_face(&edge, &forward).unwrap();
        if on_face.pcurve.is_null() {
            println!("Error: Edge {index} does not have pcurve");
        } else {
            let curve = pcurve_geometry(&on_face.pcurve, on_face.first, on_face.last);
            s.bind_drawing(&format!("{name}_{index}"), Drawing::PlaneCurve(&curve));
            curves.push(curve);
        }
        index += 1;
        edges.next().unwrap();
    }
    curves
}

/// `settolerance name value`: the line it writes, and every tolerance limited to the value.
/// Example: `settolerance(s, "b1_3", 2.0e-7)`.
pub fn settolerance(s: &dyn Session, name: &str, value: f64) {
    println!("Setting Tolerance to {}", format_general(value, 17));
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("settolerance {name}: expected a bound shape"));
    ShapeFixShapeTolerance::new()
        .limit_tolerance(shape, value, value, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("settolerance {name}: {failure:?}"));
}

/// The smallest and largest tolerance of the parts of `kind`, and how many distinct parts.
fn tolerance_span(shape: &Shape, kind: ShapeEnum) -> (f64, f64, usize) {
    let (mut min, mut max) = (f64::MAX, -f64::MAX);
    let mut seen: Vec<Shape> = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let part = explorer.current().clone();
        let tolerance = BRepTool::max_tolerance(&part, kind).unwrap();
        (min, max) = (min.min(tolerance), max.max(tolerance));
        if !seen.iter().any(|known| known.is_same(&part)) {
            seen.push(part);
        }
        explorer.next().unwrap();
    }
    (min, max, seen.len())
}

/// `maxtolerance name`: writes its report, and returns `Max` of the faces as it printed.
/// Example: `maxtolerance(s, "b1")`.
pub fn maxtolerance(s: &dyn Session, name: &str) -> String {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("maxtolerance {name}: expected a bound shape"));
    let faces = tolerance_span(shape, ShapeEnum::Face);
    let edges = tolerance_span(shape, ShapeEnum::Edge);
    let vertices = tolerance_span(shape, ShapeEnum::Vertex);
    println!(
        "\n## Tolerances on the shape {name}  (nbFaces:{}  nbEdges:{} nbVtx:{})",
        faces.2, edges.2, vertices.2
    );
    for (label, (min, max, _)) in [("Face  ", faces), ("Edge  ", edges), ("Vertex", vertices)] {
        if min <= max {
            println!(
                "\n    {label} : Min {}    Max  {} \n ",
                format_scientific(min, 5),
                format_scientific(max, 5)
            );
        }
    }
    format_scientific(faces.1, 5)
}

/// `dump curve`: the frame and the readable record at 15 digits, which it writes and returns.
/// Example: `dump_curve("c_5", &curve)`.
pub fn dump_curve(name: &str, curve: &Curve) -> String {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    CurveSet::print_curve(curve, &mut stream, false)
        .unwrap_or_else(|failure| panic!("dump {name}: {failure:?}"));
    let mut device = stream
        .into_device()
        .unwrap_or_else(|| panic!("dump {name}: expected the stream's bytes"));
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut text = String::new();
    device.read_to_string(&mut text).unwrap();
    println!("\n*********** Dump of {name} *************\n{text}");
    text
}

/// `mkplane name wire 1`: the face only if the wire lies on a plane; `mkplane_only(s, "f", "w")`.
pub fn mkplane_only(s: &mut dyn Session, name: &str, boundary: &str) {
    let shape = s
        .shape(boundary)
        .unwrap_or_else(|| panic!("mkplane {name}: expected the bound wire {boundary}"));
    let wire = topods::wire(shape).unwrap_or_else(|failure| panic!("mkplane {name}: {failure:?}"));
    let mut maker = BRepBuilderAPIMakeFace::from_wire(&wire, true)
        .unwrap_or_else(|failure| panic!("mkplane {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `tmirror name x y z dx dy dz`; `tmirror(s, "p", [0., -500., 0.], [0., 1., 0.])`.
pub fn tmirror(s: &mut dyn Session, name: &str, at: [f64; 3], normal: [f64; 3]) {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("tmirror {name}: expected a bound shape"))
        .clone();
    let plane = Axis2::new(point(at), direction(normal))
        .unwrap_or_else(|failure| panic!("tmirror {name} {at:?} {normal:?}: {failure:?}"));
    let mut transform = Transform3::default();
    transform.set_mirror_plane(plane);
    let mut maker = BRepBuilderAPITransform::with_shape(&shape, transform, false, false)
        .unwrap_or_else(|failure| panic!("tmirror {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `removeloc name name`: the shape without locations; `removeloc(s, "b2")`.
pub fn removeloc(s: &mut dyn Session, name: &str) {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("removeloc {name}: expected a bound shape"))
        .clone();
    let operation = ShapeUpgradeRemoveLocations::new();
    operation
        .remove(&shape)
        .unwrap_or_else(|failure| panic!("removeloc {name}: {failure:?}"));
    s.bind(name, &operation.get_result());
}

/// `binrestore file name`; `binrestore(s, "bug29807_f1.bin", "f1")`.
pub fn binrestore(s: &mut dyn Session, file: &str, name: &str) {
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(crate::support::restore::input_bytes(file));
    bintools::read(&mut shape, &mut stream, &ProgressRange::default())
        .unwrap_or_else(|failure| panic!("binrestore {file}: {failure:?}"));
    s.bind(name, &shape);
}

/// `distmini name first second`, then `dval name_val`: what the command writes, and the value.
/// Example: `distmini(s, "dd", "alone_1", "alone_2")`.
pub fn distmini(s: &dyn Session, name: &str, first: &str, second: &str) -> f64 {
    let shape_of = |part: &str| {
        s.shape(part)
            .unwrap_or_else(|| panic!("distmini {part}: expected a bound shape"))
    };
    let mut distance = BRepExtremaDistShapeShape::new();
    distance.load_s1(shape_of(first)).unwrap();
    distance.load_s2(shape_of(second)).unwrap();
    distance.set_deflection(precision::confusion());
    distance.set_multi_thread(false);
    distance.perform(&ProgressRange::default()).unwrap();
    assert!(
        distance.is_done(),
        "distmini {first} {second}: expected done"
    );
    let solutions = (1..=distance.nb_solution()).map(|index| match index {
        1 => name.to_owned(),
        _ => format!("{name}{index}"),
    });
    let words: Vec<String> = solutions.collect();
    println!("\"distmini\" command returns:");
    println!("{name}_val {}", words.join(" "));
    println!("Output is complete.");
    distance.value().unwrap()
}
