//! Four original spatial geometry cases exercise production dump output in its owning layer.
#![allow(clippy::approx_constant)] // Original bug30342 cylinder bound is the literal 3.14.
use martensite_tkernel::{ncollection::Array1, standard::Handle, storage::IoStream};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomCurve, GeomCylindricalSurface, GeomRectangularTrimmedSurface,
    GeomSurface, GeomToroidalSurface,
};
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::gp::{Axis3, Point2, Point3};
use std::io::{Read, Seek, SeekFrom};

const SEGMENT_POLES: [(f64, f64); 8] = [
    (117.9, 11.6),
    (116.7, 11.),
    (115.8, 10.5),
    (114.4, 9.7),
    (113.9, 9.4),
    (113.3, 9.1),
    (113.1, 9.),
    (113., 8.9),
];
const SEGMENT_KNOTS: [f64; 4] = [2., 2.5, 2.9, 3.];
const SEGMENT_MULTS: [i32; 4] = [4, 2, 2, 4];

fn dump_geometry(name: &str, print: impl FnOnce(&mut IoStream)) -> String {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    stream.insert_bytes(format!("\n\n*********** Dump of {name} *************\n").as_bytes());
    print(&mut stream);
    let mut device = stream.into_device().unwrap();
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut bytes = Vec::new();
    device.read_to_end(&mut bytes).unwrap();
    String::from_utf8(bytes).unwrap()
}

fn dump_surface(name: &str, surface: &Handle<dyn GeomSurface>) -> String {
    dump_geometry(name, |stream| {
        geom_tools::dump_surface(surface, stream).unwrap()
    })
}

fn trim_surface(
    basis: &Handle<dyn GeomSurface>,
    u: (f64, f64),
    v: (f64, f64),
    senses: (bool, bool),
) -> Handle<dyn GeomSurface> {
    Handle::new(GeomRectangularTrimmedSurface::new(basis, u, v, senses).unwrap()).cast()
}

fn trim_direction(
    basis: &Handle<dyn GeomSurface>,
    bounds: (f64, f64),
    along_u: bool,
    sense: bool,
) -> Handle<dyn GeomSurface> {
    Handle::new(
        GeomRectangularTrimmedSurface::with_one_direction(
            basis, bounds.0, bounds.1, along_u, sense,
        )
        .unwrap(),
    )
    .cast()
}

#[test]
fn tkg3d_1_sequential_cylinder_trims_dump_identically() {
    let c1: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 10.).unwrap()).cast();
    let c3 = trim_surface(&c1, (0., 3.14), (0., 50.), (true, true));
    let expected = dump_surface("c3", &c3);
    let c2 = trim_direction(&c1, (0., 3.14), true, true);
    let c3 = trim_direction(&c2, (0., 50.), false, true);
    assert_eq!(dump_surface("c3", &c3), expected, "Error: invalid result");
}

#[test]
fn tkg3d_2_reversed_torus_trims_dump_identically() {
    let t: Handle<dyn GeomSurface> =
        Handle::new(GeomToroidalSurface::new(Axis3::default(), 1., 0.5).unwrap()).cast();
    let t2 = trim_surface(&t, (0., 2.), (0., 4.), (false, false));
    let expected = dump_surface("t2", &t2);
    let t1 = trim_direction(&t, (0., 2.), true, false);
    let t2 = trim_direction(&t1, (0., 4.), false, false);
    assert_eq!(dump_surface("t2", &t2), expected, "Error: invalid result");
}

fn original_cubic3d() -> Handle<GeomBSplineCurve> {
    let poles = SEGMENT_POLES.map(|(x, z)| Point3::new(x, 0., z));
    Handle::new(
        GeomBSplineCurve::with_weights(
            &Array1::from_shared(&poles, 1),
            &Array1::from_shared(&[1.; 8], 1),
            &Array1::from_shared(&SEGMENT_KNOTS, 1),
            &Array1::from_shared(&SEGMENT_MULTS, 1),
            3,
            false,
        )
        .unwrap(),
    )
}

fn original_cubic2d() -> Handle<Geom2dBSplineCurve> {
    let poles = SEGMENT_POLES.map(|(x, y)| Point2::new(x, y));
    Handle::new(
        Geom2dBSplineCurve::with_weights(
            &Array1::from_shared(&poles, 1),
            &Array1::from_shared(&[1.; 8], 1),
            &Array1::from_shared(&SEGMENT_KNOTS, 1),
            &Array1::from_shared(&SEGMENT_MULTS, 1),
            3,
            false,
        )
        .unwrap(),
    )
}

fn knot_rows(text: &str) -> impl Iterator<Item = &str> {
    let (_, rows) = text
        .split_once("Knots :\n")
        .unwrap_or_else(|| panic!("dump={text:?}; expected Knots : newline section"));
    rows.lines()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
}

fn knot_row(line: &str) -> f64 {
    let (index, rest) = line
        .split_once(':')
        .unwrap_or_else(|| panic!("row={line:?}; expected index : knot multiplicity"));
    require_digits(index.trim(), "index");
    let fields: Vec<&str> = rest.split_whitespace().collect();
    assert_eq!(fields.len(), 2, "row={line:?}; expected knot multiplicity");
    require_digits(fields[1], "multiplicity");
    assert!(
        fields[0]
            .bytes()
            .all(|b| b.is_ascii_digit() || b"-+.eE".contains(&b)),
        "row={line:?}; expected signed decimal/scientific knot"
    );
    fields[0].parse().unwrap()
}

fn require_digits(text: &str, field: &str) {
    assert!(
        !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()),
        "{field}={text:?}; expected decimal digits"
    );
}

fn knots_are_different(text: &str, tolerance: f64) -> bool {
    let mut preceding: Vec<f64> = Vec::new();
    for row in knot_rows(text) {
        let knot = knot_row(row);
        if preceding
            .iter()
            .any(|prior| (prior - knot).abs() < tolerance)
        {
            return false;
        }
        preceding.push(knot);
    }
    assert!(
        preceding.len() >= 2,
        "dump={text:?}; expected at least two printed knots"
    );
    true
}

#[test]
fn tkg3d_2_segmented_cubic_dump_has_distinct_knots() {
    let result = original_cubic3d();
    let tolerance = 2.0e-9;
    result.segment(2.5, 2.900000001, tolerance).unwrap();
    let curve: Handle<dyn GeomCurve> = result.cast();
    let text = dump_geometry("result", |s| geom_tools::dump_curve(&curve, s).unwrap());
    assert!(
        knots_are_different(&text, tolerance),
        "ERROR: segment has knots too close: {text}"
    );
}

#[test]
fn tkg3d_3_segmented_planar_cubic_dump_has_distinct_knots() {
    let result = original_cubic2d();
    let tolerance = 1.0e-9;
    result.segment(2.5, 2.900000000001, tolerance).unwrap();
    let curve: Handle<dyn Geom2dCurve> = result.cast();
    let text = dump_geometry("result", |s| geom_tools::dump_curve2d(&curve, s).unwrap());
    assert!(
        knots_are_different(&text, tolerance),
        "ERROR: segment has knots too close: {text}"
    );
}

#[test]
fn dump_knot_predicate_keeps_strict_less_than_boundary() {
    let boundary = "Knots :\n   1 : 0 4\n   2 : 0.25 4\n\n";
    assert_eq!(
        knot_rows(boundary).map(knot_row).collect::<Vec<_>>(),
        [0., 0.25]
    );
    assert!(knots_are_different(boundary, 0.25));
    assert!(!knots_are_different(boundary, 0.250_000_000_000_000_06));
    assert!(!knots_are_different(
        "Knots :\n   1 : 2.5 4\n   2 : 2.5 4\n\n",
        1e-9
    ));
}

#[test]
fn dump_knot_predicate_stops_reading_after_first_duplicate() {
    let text = "Knots :\n   1 : 2.5 4\n   2 : 2.5 4\n unparsed trailing text\n\n";
    assert!(!knots_are_different(text, 1.0e-9));
}

#[test]
fn production_knot_header_blank_line_does_not_skip_duplicate_rows() {
    let text = "Knots :\n\n   1 :  2.5 4\n   2 :  2.5 4\n\n";
    assert_eq!(
        knot_rows(text).map(knot_row).collect::<Vec<_>>(),
        [2.5, 2.5]
    );
    assert!(!knots_are_different(text, 2.0e-9));
}

#[test]
#[should_panic(expected = "expected at least two printed knots")]
fn dump_knot_predicate_rejects_vacuous_empty_section() {
    knots_are_different("Knots :\n\n", 1.0e-9);
}
