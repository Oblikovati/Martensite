//! The result checks the cases call. A check that would print
//! `Error` (which the test grid's parse rules turn into a failure) panics instead.
use super::commands::{length2d, length3d};
use martensite_tkbrep::{brep_tools::BRepToolsShapeSet, topods::Shape, toptools::ShapeSet};
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomCurve;

/// `checklength` passes this epsilon to `length` unless `-eps` is given.
pub const CHECKLENGTH_EPSILON: f64 = 1.0e-4;

/// `checkreal name value expected tol_abs tol_rel`.
pub fn checkreal(name: &str, value: f64, expected: f64, tol_abs: f64, tol_rel: f64) {
    assert!(
        (value - expected).abs() <= tol_abs + tol_rel * expected.abs(),
        "Error: {name} = {value} is not equal to expected {expected}"
    );
}

/// `checklength shape -l expected`: nonzero, and within 1% of `expected`.
pub fn check_length(measured: f64, expected: f64) {
    assert!(
        measured != 0.0,
        "Error : The command is not valid. The length is 0."
    );
    assert!(
        (expected - measured).abs() / expected <= 0.01,
        "Error : The length of result shape is {measured}; expected {expected} within 1%"
    );
}

/// `checklength c2d -l expected` on a 2D curve.
pub fn checklength2d(curve: &Handle<dyn Geom2dCurve>, expected: f64) {
    check_length(length2d(curve, CHECKLENGTH_EPSILON), expected);
}

/// `checklength c -l expected` on a 3D curve.
pub fn checklength3d(curve: &Handle<dyn GeomCurve>, expected: f64) {
    check_length(length3d(curve, CHECKLENGTH_EPSILON), expected);
}

/// The `checknbshapes` entities, in the order it reads them from `nbshapes`.
pub const NBSHAPES_ENTITIES: [&str; 9] = [
    "VERTEX",
    "EDGE",
    "WIRE",
    "FACE",
    "SHELL",
    "SOLID",
    "COMPSOLID",
    "COMPOUND",
    "SHAPE",
];

/// DBRep.cxx `nbshapes shape` (without `-t`): `BRepTools_ShapeSet::DumpExtent` of a set
/// holding `shape`; an empty string for a null shape.
pub fn nbshapes(shape: &Shape) -> String {
    if shape.is_null() {
        return String::new();
    }
    let mut set = BRepToolsShapeSet::new(true, false);
    set.add(shape).unwrap();
    let mut text = AsciiString::default();
    set.dump_extent_string(&mut text).unwrap();
    format!(
        "Number of shapes in shape\n{}",
        String::from_utf8_lossy(text.as_bytes())
    )
}

/// `checknbshapes shape -vertex n ... -shape n`: each count of `expected`, in
/// [`NBSHAPES_ENTITIES`] order, read from `nbshapes` with `ENTITY +: +(n)`.
pub fn checknbshapes(shape: &Shape, expected: [usize; 9]) {
    let nb_info = nbshapes(shape);
    for (entity, expected) in NBSHAPES_ENTITIES.iter().zip(expected) {
        let found = nbshapes_count(&nb_info, entity)
            .unwrap_or_else(|| panic!("Error : command \"nbshapes\" gives an empty result"));
        assert_eq!(found, expected, "Error : number of {entity} entities");
    }
}

fn nbshapes_count(nb_info: &str, entity: &str) -> Option<usize> {
    let at = nb_info.find(&format!("{entity} "))?;
    let rest = nb_info[at + entity.len()..].trim_start_matches(' ');
    let rest = rest.strip_prefix(':')?.trim_start_matches(' ');
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}
