//! The construction commands of the scripts; each binds its result under the script's name.
use crate::support::session::Session;
use martensite_tkbo::bop_tools::{
    add_missing_pcurves, reduce_tolerances, reduce_vertex_tolerances,
};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::{standard::Failure, storage::format_general};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeRevol};
use martensite_tkshhealing::shape_fix::ShapeFixShape;
use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire, BRepBuilderAPINurbsConvert,
    BRepBuilderAPITransform,
};
use martensite_tktopalgo::brep_lib;

/// The answer of `validrange` for an edge that has none.
pub const NO_VALID_RANGE: &str = "edge has no valid range";

fn bound(s: &dyn Session, command: &str, name: &str) -> Shape {
    s.shape(name)
        .unwrap_or_else(|| panic!("{command} {name}: expected a bound shape"))
        .clone()
}

/// `vertex name x y z`; `vertex(s, "v1", [0., 0., 0.])`.
pub fn vertex(s: &mut dyn Session, name: &str, at: [f64; 3]) {
    let maker = BRepBuilderAPIMakeVertex::new(Point3::new(at[0], at[1], at[2]))
        .unwrap_or_else(|failure| panic!("vertex {name} {at:?}: {failure:?}"));
    s.bind(name, &maker.vertex().unwrap().into_shape());
}

/// `edge name first second`; `edge(s, "e1", "v1", "v2")`.
pub fn edge(s: &mut dyn Session, name: &str, first: &str, second: &str) {
    let first = topods::vertex(&bound(s, "edge", first)).unwrap();
    let second = topods::vertex(&bound(s, "edge", second)).unwrap();
    let maker = BRepBuilderAPIMakeEdge::from_vertices(&first, &second)
        .unwrap_or_else(|failure| panic!("edge {name}: {failure:?}"));
    s.bind(name, &maker.edge().unwrap().into_shape());
}

/// `wire name e1 e2 ...`, edges and wires in the script's order; `wire(s, "w", &["e1", "e2"])`.
pub fn wire(s: &mut dyn Session, name: &str, parts: &[&str]) {
    let mut maker = BRepBuilderAPIMakeWire::new();
    for part in parts {
        let shape = bound(s, "wire", part);
        if let Ok(as_edge) = topods::edge(&shape) {
            maker.add_edge(&as_edge).unwrap();
        } else if let Ok(as_wire) = topods::wire(&shape) {
            maker.add_wire(&as_wire).unwrap();
        }
    }
    s.bind(name, &maker.wire().unwrap().into_shape());
}

/// `mkplane name wire`; `mkplane(s, "f", "w")`.
pub fn mkplane(s: &mut dyn Session, name: &str, boundary: &str) {
    plane_face(s, name, boundary, false);
}

/// `mkplane name wire 1`: the plane is the only surface to look for.
/// Example: `mkplane_only_plane(s, "f", "w")`.
pub fn mkplane_only_plane(s: &mut dyn Session, name: &str, boundary: &str) {
    plane_face(s, name, boundary, true);
}

fn plane_face(s: &mut dyn Session, name: &str, boundary: &str, only_plane: bool) {
    let boundary = topods::wire(&bound(s, "mkplane", boundary)).unwrap();
    let mut maker = BRepBuilderAPIMakeFace::from_wire(&boundary, only_plane)
        .unwrap_or_else(|failure| panic!("mkplane {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `revol name base x y z dx dy dz degrees`; `revol(s, "r", "f", [0.; 3], [0., 0., 1.], 90.)`.
pub fn revol(s: &mut dyn Session, name: &str, base: &str, at: [f64; 3], axis: [f64; 3], deg: f64) {
    let base = bound(s, "revol", base);
    let direction = Direction3::new(axis[0], axis[1], axis[2])
        .unwrap_or_else(|failure| panic!("revol {name} axis {axis:?}: {failure:?}"));
    let line = Axis1::new(Point3::new(at[0], at[1], at[2]), direction);
    let radians = deg * (std::f64::consts::PI / 180.0);
    let mut maker = BRepPrimAPIMakeRevol::new(&base, line, radians, false)
        .unwrap_or_else(|failure| panic!("revol {name} {deg}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `nurbsconvert result name`; `nurbsconvert(s, "b1", "b1")`.
pub fn nurbsconvert(s: &mut dyn Session, result: &str, name: &str) {
    let shape = bound(s, "nurbsconvert", name);
    let mut maker = BRepBuilderAPINurbsConvert::with_shape(&shape, false)
        .unwrap_or_else(|failure| panic!("nurbsconvert {name}: {failure:?}"));
    s.bind(result, maker.shape().unwrap());
}

/// `copy name other`: a second name of the same shape; `copy(s, "b_1", "b1")`.
pub fn copy(s: &mut dyn Session, name: &str, other: &str) {
    let shape = bound(s, "copy", name);
    s.bind(other, &shape);
}

/// `tcopy name other`: a copy of the geometry and topology; `tcopy(s, "b", "c")`.
pub fn tcopy(s: &mut dyn Session, name: &str, other: &str) {
    let shape = bound(s, "tcopy", name);
    let mut maker = BRepBuilderAPICopy::with_shape(&shape, true, false)
        .unwrap_or_else(|failure| panic!("tcopy {name}: {failure:?}"));
    s.bind(other, maker.shape().unwrap());
}

/// `tscale name x y z factor`; `tscale(s, "p", [0., 0., 0.], 100.)`.
pub fn tscale(s: &mut dyn Session, name: &str, at: [f64; 3], factor: f64) {
    let shape = bound(s, "tscale", name);
    let mut transform = Transform3::default();
    transform
        .set_scale(Point3::new(at[0], at[1], at[2]), factor)
        .unwrap_or_else(|failure| panic!("tscale {name} {factor}: {failure:?}"));
    let mut maker = BRepBuilderAPITransform::with_shape(&shape, transform, false, false)
        .unwrap_or_else(|failure| panic!("tscale {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// What `validrange edge` answers, in the command's words; a shape that is no edge is its error.
/// Example: `valid_range_answer(&edge)?`.
pub fn valid_range_answer(shape: &Shape) -> Result<String, Failure> {
    let edge = topods::edge(shape)?;
    let range = brep_lib::find_valid_range_of_edge(&edge)?;
    let (true, Some(first), Some(last)) = (range.is_valid, range.first, range.last) else {
        return Ok(NO_VALID_RANGE.to_owned());
    };
    Ok(format!(
        "{} {}",
        format_general(first, 17),
        format_general(last, 17)
    ))
}

/// `treverse name`: the same shape with the other orientation; `treverse(s, "fa")`.
pub fn treverse(s: &mut dyn Session, name: &str) {
    let shape = bound(s, "treverse", name).reversed();
    s.bind(name, &shape);
}

/// `prism name base dx dy dz SemiInf`: unbounded along the vector's direction only.
/// Example: `prism_semi_infinite(s, "p", "f", [1., 0., 0.])`.
pub fn prism_semi_infinite(s: &mut dyn Session, name: &str, base: &str, along: [f64; 3]) {
    let base = bound(s, "prism", base);
    let direction = Direction3::from_vector(Vector3::new(along[0], along[1], along[2]))
        .unwrap_or_else(|failure| panic!("prism {name} {along:?}: {failure:?}"));
    let mut maker = BRepPrimAPIMakePrism::infinite(&base, direction, false, false, true)
        .unwrap_or_else(|failure| panic!("prism {name} semi-infinite: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `updatetolerance name [anything]`: with the extra argument, face tolerances are verified too.
/// Example: `updatetolerance(s, "tool", true)`.
pub fn updatetolerance(s: &dyn Session, name: &str, verify_faces: bool) {
    brep_lib::update_tolerances(&bound(s, "updatetolerance", name), verify_faces)
        .unwrap_or_else(|failure| panic!("updatetolerance {name}: {failure:?}"));
}

/// `fsameparameter name`: forces same-parameter at the command's default tolerance.
/// Example: `fsameparameter(s, "a")`.
pub fn fsameparameter(s: &dyn Session, name: &str) {
    brep_lib::same_parameter_shape(&bound(s, "fsameparameter", name), 1.0e-7, true)
        .unwrap_or_else(|failure| panic!("fsameparameter {name}: {failure:?}"));
}

/// `settolerance name value`: forces every tolerance; `settolerance(s, "a", 0.1)`.
pub fn settolerance(s: &dyn Session, name: &str, value: f64) {
    println!("Setting Tolerance to {}", format_general(value, 17));
    ShapeFixShapeTolerance::new()
        .limit_tolerance(
            &bound(s, "settolerance", name),
            value,
            value,
            ShapeEnum::Shape,
        )
        .unwrap_or_else(|failure| panic!("settolerance {name}: {failure:?}"));
}

/// `normals name`: draws face normals, which a later `checkview` erases; what stays is the legend
/// the command writes. Example: `normals(s, "result")`.
pub fn normals(_s: &dyn Session, _name: &str) {
    println!("rouge  FORWARD");
    println!("bleu   REVERSED");
    println!("rose   EXTERNAL");
    println!("orange INTERNAL");
}

/// `btolx name`: reduces the tolerances, then forces same-parameter; `btolx(s, "tool")`.
pub fn btolx(s: &dyn Session, name: &str) {
    let shape = bound(s, "btolx", name);
    reduce_tolerances(&shape, 1.0e-7).unwrap_or_else(|failure| panic!("btolx {name}: {failure:?}"));
    brep_lib::same_parameter_shape(&shape, 1.0e-7, true)
        .unwrap_or_else(|failure| panic!("btolx {name}: {failure:?}"));
}

/// `breducetolerance name`: reduces the vertex tolerances; `breducetolerance(s, "a")`.
pub fn breducetolerance(s: &dyn Session, name: &str) {
    reduce_vertex_tolerances(&bound(s, "breducetolerance", name))
        .unwrap_or_else(|failure| panic!("breducetolerance {name}: {failure:?}"));
}

/// `bopaddpcs name`: adds the p-curves its faces' edges lack; `bopaddpcs(s, "a")`.
pub fn bopaddpcs(s: &dyn Session, name: &str) {
    add_missing_pcurves(&bound(s, "bopaddpcs", name))
        .unwrap_or_else(|failure| panic!("bopaddpcs {name}: {failure:?}"));
}

/// `setflags name locked`: the root and each vertex, edge and face occurrence.
/// Example: `setflags_locked(s, "b")`.
pub fn setflags_locked(s: &dyn Session, name: &str) {
    let shape = bound(s, "setflags", name);
    shape.set_locked(true).unwrap();
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for current in Explorer::new(&shape, kind, ShapeEnum::Shape).unwrap() {
            current.set_locked(true).unwrap();
        }
    }
}

/// `fixshape result name`: the shape repaired by its healing defaults; `fixshape(s, "b", "b")`.
pub fn fixshape(s: &mut dyn Session, result: &str, name: &str) {
    let repair = ShapeFixShape::new(bound(s, "fixshape", name));
    repair
        .perform()
        .unwrap_or_else(|failure| panic!("fixshape {name}: {failure:?}"));
    s.bind(result, &repair.shape());
}
