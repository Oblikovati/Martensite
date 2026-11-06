//! The commands that read a variable or change what it holds, with no modelling of their own.
use super::{
    commands::explode,
    feat_script::{FeatScript, checked, command_failed},
    session::Session,
};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools::History, topods::Compound, topods::Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkfeat::brep_feat::BRepFeatMakeCylindricalHole;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};

/// `renamevar from to`: `to` holds what `from` held; `renamevar(s, "c_1", "f")`.
pub fn renamevar(s: &mut dyn Session, from: &str, to: &str) {
    let shape = s
        .shape(from)
        .unwrap_or_else(|| panic!("renamevar {from}: expected a bound shape"))
        .clone();
    s.bind(to, &shape);
}

/// `whatis name`: the words the command answers. A harness echo carries them, so nothing is
/// written; a test reads them when a script does. Example: `whatis(s, "E").contains("SOLID")`.
pub fn whatis(s: &dyn Session, name: &str) -> String {
    let Some(shape) = s.shape(name) else {
        return String::new();
    };
    let kind = format!("{:?}", shape.shape_type().unwrap()).to_uppercase();
    let orientation = format!("{:?}", shape.orientation()).to_uppercase();
    let mut words = format!("shape {kind} {orientation}");
    for (flag, word) in [
        (shape.free().unwrap(), " Free"),
        (shape.modified().unwrap(), " Modified"),
        (shape.orientable().unwrap(), " Orientable"),
        (shape.closed().unwrap(), " Closed"),
        (shape.infinite().unwrap(), " Infinite"),
        (shape.convex().unwrap(), " Convex"),
    ] {
        if flag {
            words.push_str(word);
        }
    }
    words
}

/// `llength [explode name kind]`; `count_of(s, "result", ShapeEnum::face)`.
pub fn count_of(s: &mut dyn Session, name: &str, kind: ShapeEnum) -> usize {
    explode(s, name, Some(kind)).len()
}

/// The session's history; `savehistory(s)`.
pub fn savehistory(s: &FeatScript<'_>) -> Handle<History> {
    s.history
        .clone()
        .unwrap_or_else(|| panic!("savehistory: expected a history in the session"))
}

/// `isdeleted history shape`: its answer. A harness echo carries it, so nothing is written.
/// Example: `isdeleted(s, &h, "S")`.
pub fn isdeleted(s: &dyn Session, history: &Handle<History>, name: &str) -> &'static str {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("isdeleted {name}: expected a bound shape"));
    let removed = history.get().map_or(Ok(false), |h| h.is_removed(shape));
    if checked(removed) {
        "Deleted."
    } else {
        "Not deleted."
    }
}

/// Writes `empty` when `found` holds nothing, and binds it otherwise: one shape as itself, more
/// as a compound, as the history queries do.
fn published_query(s: &mut dyn Session, name: &str, found: Vec<Shape>, empty: &str) -> String {
    if found.is_empty() {
        println!("{empty}");
        return empty.to_owned();
    }
    if let [only] = found.as_slice() {
        s.bind(name, only);
        return String::new();
    }
    let builder = BRepBuilder::default();
    let mut whole = Compound::default();
    builder.make_compound(&mut whole);
    let mut whole = whole.into_shape();
    for shape in &found {
        checked(builder.add(&mut whole, shape));
    }
    s.bind(name, &whole);
    String::new()
}

/// `modified name history shape`: returns what it writes; `modified(s, "m1", &h, "S")`.
pub fn modified(s: &mut dyn Session, name: &str, history: &Handle<History>, shape: &str) -> String {
    let initial = s
        .shape(shape)
        .unwrap_or_else(|| panic!("modified {shape}: expected a bound shape"))
        .clone();
    let found = history
        .get()
        .map_or(Ok(Vec::new()), |h| h.modified(&initial));
    published_query(s, name, checked(found), "The shape has not been modified.")
}

/// `generated name history shape`: returns what it writes; `generated(s, "g1", &h, "e")`.
pub fn generated(
    s: &mut dyn Session,
    name: &str,
    history: &Handle<History>,
    shape: &str,
) -> String {
    let initial = s
        .shape(shape)
        .unwrap_or_else(|| panic!("generated {shape}: expected a bound shape"))
        .clone();
    let found = history
        .get()
        .map_or(Ok(Vec::new()), |h| h.generated(&initial));
    published_query(
        s,
        name,
        checked(found),
        "No shapes were generated from the shape.",
    )
}

/// `maxtolerance name` as a script line: it writes its answer and returns it.
/// Example: `maxtolerance_line(s, "w")`.
pub fn maxtolerance_line(s: &dyn Session, name: &str) -> String {
    let answer = super::maxtolerance::maxtolerance(s, name);
    println!("{answer}");
    answer
}

/// `regexp { +kind +: +Min +x +Max +(x)} text`: the largest tolerance of a `maxtolerance` row.
/// Example: `max_tolerance_of("    Edge: Min 1.0e-07    Max 2.0e-07 \n", "Edge")`.
pub fn max_tolerance_of(answer: &str, kind: &str) -> f64 {
    let row = answer
        .lines()
        .find(|line| line.trim_start().starts_with(kind))
        .unwrap_or_else(|| panic!("maxtolerance: expected a {kind} row in {answer:?}"));
    let (_, value) = row
        .rsplit_once("Max")
        .unwrap_or_else(|| panic!("maxtolerance: expected Max in {row:?}"));
    value
        .trim()
        .parse()
        .unwrap_or_else(|cause| panic!("maxtolerance {row:?}: {cause}"))
}

/// `firsthole result shape x y z dx dy dz radius`.
/// Example: `firsthole(s, "r", "a", [5., 0., 22.], [0., 0., -1.], 2.)`.
pub fn firsthole(
    s: &mut FeatScript<'_>,
    result: &str,
    shape: &str,
    at: [f64; 3],
    along: [f64; 3],
    radius: f64,
) {
    let axis = Axis1::new(
        Point3::new(at[0], at[1], at[2]),
        checked(Direction3::new(along[0], along[1], along[2])),
    );
    let source = s.shape_or_null(shape);
    let mut hole = BRepFeatMakeCylindricalHole::new();
    hole.init(&source, axis);
    checked(hole.perform_thru_next(radius, true));
    checked(hole.build());
    if hole.builder().has_errors() {
        command_failed(&format!(
            "Echec de MakeCylindricalHole  Error Status : {:?}",
            hole.status()
        ));
    }
    let made = hole.shape().clone();
    s.bind(result, &made);
}
