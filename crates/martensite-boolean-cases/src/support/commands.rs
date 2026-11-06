//! The shape-building commands of the case scripts: each binds its result as the script does.
use super::session::Session;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{Shape, ShapeIterator},
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakePrism,
    BRepPrimAPIMakeSphere, BRepPrimAPIMakeTorus,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform};

/// `box name x y z dx dy dz`; `make_box(s, "b1", [0., 0., 0.], [1., 1., 1.])`.
pub fn make_box(session: &mut dyn Session, name: &str, origin: [f64; 3], sizes: [f64; 3]) {
    let corner = Point3::new(origin[0], origin[1], origin[2]);
    let mut maker = BRepPrimAPIMakeBox::at_point(corner, sizes[0], sizes[1], sizes[2])
        .unwrap_or_else(|failure| panic!("box {name} {origin:?} {sizes:?}: {failure:?}"));
    session.bind(name, maker.shape().unwrap());
}

/// `pcylinder name radius height`; `pcylinder(s, "b1", 4., 8.)`.
pub fn pcylinder(session: &mut dyn Session, name: &str, radius: f64, height: f64) {
    let mut maker = BRepPrimAPIMakeCylinder::new(radius, height);
    session.bind(name, maker.shape().unwrap());
}

/// `pcone name radius1 radius2 height`; `pcone(s, "b1", 4., 2., 8.)`.
pub fn pcone(session: &mut dyn Session, name: &str, radius1: f64, radius2: f64, height: f64) {
    let mut maker = BRepPrimAPIMakeCone::new(radius1, radius2, height)
        .unwrap_or_else(|failure| panic!("pcone {name} {radius1} {radius2} {height}: {failure:?}"));
    session.bind(name, maker.shape().unwrap());
}

/// `psphere name radius`; `psphere(s, "b2", 4.)`.
pub fn psphere(session: &mut dyn Session, name: &str, radius: f64) {
    let mut maker = BRepPrimAPIMakeSphere::new(radius)
        .unwrap_or_else(|failure| panic!("psphere {name} {radius}: {failure:?}"));
    session.bind(name, maker.shape().unwrap());
}

fn transformed(session: &mut dyn Session, name: &str, transform: Transform3) {
    let shape = bound_shape(session, name);
    let mut operation = BRepBuilderAPITransform::new(transform);
    operation
        .perform(&shape, false, false)
        .unwrap_or_else(|failure| panic!("transform {name}: {failure:?}"));
    session.bind(name, operation.shape().unwrap());
}

/// `ttranslate name dx dy dz`; `ttranslate(s, "b2", [0., 0., 8.])`.
pub fn ttranslate(session: &mut dyn Session, name: &str, delta: [f64; 3]) {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(delta[0], delta[1], delta[2]));
    transformed(session, name, transform);
}

/// `trotate name x y z dx dy dz degrees`; `trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.)`.
pub fn trotate(session: &mut dyn Session, name: &str, at: [f64; 3], axis: [f64; 3], degrees: f64) {
    let direction = Direction3::new(axis[0], axis[1], axis[2])
        .unwrap_or_else(|failure| panic!("trotate {name} axis {axis:?}: {failure:?}"));
    let line = Axis1::new(Point3::new(at[0], at[1], at[2]), direction);
    let mut transform = Transform3::default();
    transform
        .set_rotation(line, degrees * (std::f64::consts::PI / 180.0))
        .unwrap_or_else(|failure| panic!("trotate {name} {degrees}: {failure:?}"));
    transformed(session, name, transform);
}

/// `ptorus name radius1 radius2`; `ptorus(s, "b2", 4., 1.)`.
pub fn ptorus(session: &mut dyn Session, name: &str, radius1: f64, radius2: f64) {
    let mut maker = BRepPrimAPIMakeTorus::new(radius1, radius2)
        .unwrap_or_else(|failure| panic!("ptorus {name} {radius1} {radius2}: {failure:?}"));
    session.bind(name, maker.shape().unwrap());
}

/// `prism name base dx dy dz`; `prism(s, "pcy", "pc_2", [0., 0., 10.])`.
pub fn prism(session: &mut dyn Session, name: &str, base: &str, delta: [f64; 3]) {
    let base = bound_shape(session, base);
    let vector = Vector3::new(delta[0], delta[1], delta[2]);
    let mut maker = BRepPrimAPIMakePrism::new(&base, vector, false, true)
        .unwrap_or_else(|failure| panic!("prism {name} {delta:?}: {failure:?}"));
    session.bind(name, maker.shape().unwrap());
}

/// `explode name [kind]`: binds `name_1`, `name_2`, ... and returns those names. Without a kind
/// they are the direct children; with one, the distinct sub-shapes of that kind in the order
/// of a first visit, leaving out the shape itself. An unbound `name` gives no names.
/// Example: `explode(s, "r", Some(ShapeEnum::Shell))`.
pub fn explode(session: &mut dyn Session, name: &str, kind: Option<ShapeEnum>) -> Vec<String> {
    let Some(shape) = session.shape(name).cloned() else {
        return Vec::new();
    };
    let parts = kind.map_or_else(|| children(&shape), |kind| distinct_parts(&shape, kind));
    let mut names = Vec::with_capacity(parts.len());
    for (index, part) in parts.iter().enumerate() {
        names.push(format!("{name}_{}", index + 1));
        session.bind(&names[index], part);
    }
    names
}

fn children(shape: &Shape) -> Vec<Shape> {
    let mut iterator = ShapeIterator::new(shape, true, true);
    let mut parts = Vec::new();
    while let Ok(child) = iterator.value() {
        parts.push(child.clone());
        iterator.next();
    }
    parts
}

/// The sub-shapes of `kind`, each once, in the order of a first visit, without `shape` itself.
/// Example: `distinct_parts(&solid, ShapeEnum::Face)`.
pub fn distinct_parts(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut seen = vec![shape.clone()];
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let part = explorer.current();
        if !seen.iter().any(|known| known.is_same(part)) {
            seen.push(part.clone());
        }
        explorer.next().unwrap();
    }
    seen.remove(0);
    seen
}

/// `orientation name F|R|E|I`; `orientation(s, "r_2", Orientation::Reversed)`.
pub fn orientation(session: &mut dyn Session, name: &str, value: Orientation) {
    let mut shape = bound_shape(session, name);
    shape.set_orientation(value);
    session.bind(name, &shape);
}

/// `emptycopy copy original`: `copy` becomes the original without its children.
/// Example: `emptycopy(s, "b1", "b")`.
pub fn emptycopy(session: &mut dyn Session, copy: &str, original: &str) {
    let shape = bound_shape(session, original);
    let empty = shape
        .empty_copied()
        .unwrap_or_else(|failure| panic!("emptycopy {copy} {original}: {failure:?}"));
    session.bind(copy, &empty);
}

/// `compound names... name`: a compound of `names`, none for an empty one.
/// Example: `compound(s, &[], "c")`.
pub fn compound(session: &mut dyn Session, parts: &[&str], name: &str) {
    let builder = BRepBuilder::default();
    let mut whole = martensite_tkbrep::topods::Compound::default();
    builder.make_compound(&mut whole);
    let mut whole = whole.into_shape();
    for part in parts {
        let part = bound_shape(session, part);
        builder
            .add(&mut whole, &part)
            .unwrap_or_else(|failure| panic!("compound {name}: {failure:?}"));
    }
    session.bind(name, &whole);
}

/// `add part whole`: adds `part` to `whole` and rebinds `whole`; `add(s, "r_2", "b1")`.
pub fn add(session: &mut dyn Session, part: &str, whole: &str) {
    let component = bound_shape(session, part);
    let mut container = bound_shape(session, whole);
    BRepBuilder::default()
        .add(&mut container, &component)
        .unwrap_or_else(|failure| panic!("add {part} {whole}: {failure:?}"));
    session.bind(whole, &container);
}

/// The shape bound to `name`, which the script needs bound; `bound_shape(s, "b1")`.
pub fn bound_shape(session: &dyn Session, name: &str) -> Shape {
    session
        .shape(name)
        .unwrap_or_else(|| panic!("{name}: expected a bound shape"))
        .clone()
}
