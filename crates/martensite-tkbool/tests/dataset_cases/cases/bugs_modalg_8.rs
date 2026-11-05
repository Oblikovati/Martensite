//! `bugs_modalg_8`: a face bounded by two Bezier wires converted to lines and arcs.
use crate::support::{
    beyond_script::printed_mass,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    session::Session,
    tolerance_check::tolerance_report,
};
use martensite_tkbrep::{brep::BRepBuilder, topods};
use martensite_tkernel::{ncollection::Array1, standard::Handle, storage::format_general};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire},
    brep_lib,
};

/// The tolerance `arclinconvert` reports choosing from the face.
const CONVERSION_TOLERANCE: f64 = 0.01;
const OUTER_POLES: [[f64; 3]; 9] = [
    [3., 3., 0.],
    [0., 10., 0.],
    [-3., 3., 0.],
    [-10., 0., 0.],
    [-3., -3., 0.],
    [0., -10., 0.],
    [3., -3., 0.],
    [10., 0., 0.],
    [3., 3., 0.],
];
const OUTER_WEIGHTS: [f64; 9] = [10., 100., 10., 100., 10., 100., 10., 100., 10.];
const INNER_POLES: [[f64; 3]; 5] = [
    [3., 0., 0.],
    [0., 3., 0.],
    [-3., 0., 0.],
    [0., -3., 0.],
    [3., 0., 0.],
];

fn poles_of(points: &[[f64; 3]]) -> Vec<Point3> {
    points
        .iter()
        .map(|p| Point3::new(p[0], p[1], p[2]))
        .collect()
}

fn bezier(points: &[[f64; 3]], weights: Option<&[f64]>) -> Handle<dyn GeomCurve> {
    let poles = poles_of(points);
    let poles = Array1::from_shared(&poles, 1);
    let curve = match weights {
        Some(weights) => {
            GeomBezierCurve::with_weights(&poles, &Array1::from_shared(weights, 1)).unwrap()
        }
        None => GeomBezierCurve::new(&poles).unwrap(),
    };
    Handle::new(curve).cast()
}

fn wire_of(curve: &Handle<dyn GeomCurve>) -> topods::Wire {
    let edge = BRepBuilderAPIMakeEdge::from_curve(curve)
        .unwrap()
        .edge()
        .unwrap();
    BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap()
}

/// `mkplane a w1`, `add w2 a` with `w2` reversed; binds the face.
fn face_with_hole(s: &mut dyn Session) {
    let outer = wire_of(&bezier(&OUTER_POLES, Some(&OUTER_WEIGHTS)));
    let inner = wire_of(&bezier(&INNER_POLES, None)).into_shape().reversed();
    let mut face = BRepBuilderAPIMakeFace::from_wire(&outer, false)
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    BRepBuilder::default().add(&mut face, &inner).unwrap();
    s.bind("a", &face);
}

/// `arclinconvert result a`, then `build3d result`.
fn converted(s: &mut dyn Session) {
    let tolerance = format_general(CONVERSION_TOLERANCE, 6);
    println!("Info: tolerance is set to {tolerance}");
    let face = topods::face(s.shape("a").unwrap()).unwrap();
    let result = martensite_tkbool::brep_algo::convert_face(&face, CONVERSION_TOLERANCE)
        .unwrap()
        .into_shape();
    s.bind("result", &result);
    if !brep_lib::build_curves3d_default(&result).unwrap() {
        println!(" one of the computation failed");
    }
}

pub fn bug32214_3(s: &mut dyn Session) {
    println!("=========================================");
    println!("OCC32214: 2d Offset produces wrong result");
    println!("=========================================");
    face_with_hole(s);
    converted(s);
    checkshape_named(s, "result");
    let counts = [
        NbCount::Vertices(170),
        NbCount::Edges(170),
        NbCount::Wires(2),
        NbCount::Faces(1),
    ];
    checknbshapes(s, "result", Counting::Typed, &counts);
    let reached: f64 = tolerance_report(s, "result").parse().unwrap();
    if reached > 1.001e-7 {
        println!("Error: bad tolerance of result");
    }
    checkprops(s, "result", Prop::Area, Expected::Mass(106.6));
    // Beyond the script: the exact print of the reference run, not only within 1%.
    printed_mass(s, "result", Prop::Area, "106.6");
}
