//! The commands that change a shape's tolerances or rebuild it, beside the Boolean ones.
use crate::support::{
    commands::{bound_shape, distinct_parts},
    session::Session,
};
use martensite_tkbo::{
    bop_algo::BOPAlgoMakerVolume,
    bop_tools::{orient_edges_on_wire, reduce_vertex_tolerances},
    brep_algo_api::BRepAlgoAPICut,
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools::{self, History, HistoryAlgorithm},
    topods::{self, CompSolid, Compound, Shape, ShapeIterator, Solid},
};
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::List,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkmath::gp::{Axis1, Axis3, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeCone, BRepPrimAPIMakeHalfSpace, BRepPrimAPIMakeRevol,
};
use martensite_tkshhealing::shape_fix::ShapeFixShape;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPISewing},
    brep_class3d::BRepClass3dSolidClassifier,
    brep_lib::{self, BRepLibFuseEdges},
};
use std::f64::consts::PI;

/// `settolerance name value`: forces every tolerance of the shape; writes the command's line.
/// Example: `settolerance(s, "r_1", 1.0e-7)`.
pub fn settolerance(s: &dyn Session, name: &str, value: f64) {
    use martensite_tkernel::storage::format_general;
    use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
    println!("Setting Tolerance to {}\n", format_general(value, 17));
    ShapeFixShapeTolerance::new()
        .set_tolerance(&bound_shape(s, name), value, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("settolerance {name}: {failure:?}"));
}

/// `fsameparameter name`; `fsameparameter(s, "a")`.
pub fn fsameparameter(s: &mut dyn Session, name: &str) {
    let shape = bound_shape(s, name);
    brep_lib::same_parameter_shape(&shape, 1.0e-7, true)
        .unwrap_or_else(|failure| panic!("fsameparameter {name}: {failure:?}"));
    s.bind(name, &shape);
}

/// `updatetolerance name`; `updatetolerance(s, "a")`.
pub fn updatetolerance(s: &mut dyn Session, name: &str) {
    let shape = bound_shape(s, name);
    brep_lib::update_tolerances(&shape, false)
        .unwrap_or_else(|failure| panic!("updatetolerance {name}: {failure:?}"));
    s.bind(name, &shape);
}

/// `sewing result tolerance shapes`; `sewing(s, "s1", 1.0e-6, &["f1", "f2"])`.
pub fn sewing(s: &mut dyn Session, result: &str, tolerance: f64, parts: &[&str]) {
    let mut maker = BRepBuilderAPISewing::default();
    maker.init(tolerance, true, true, true, false);
    for part in parts {
        maker.add(&bound_shape(s, part)).unwrap();
    }
    maker
        .perform(&ProgressRange::default())
        .unwrap_or_else(|failure| panic!("sewing {result}: {failure:?}"));
    let mut dump = Vec::new();
    maker.dump(&mut dump).unwrap();
    print!("{}", String::from_utf8_lossy(&dump));
    let sewn = maker.sewed_shape().clone();
    assert!(!sewn.is_null(), "sewing {result}: no sewn shape");
    s.bind(result, &sewn);
}

/// `ssolid shell solid`: a solid of the shell, reversed when it would hold the infinite point.
/// Example: `ssolid(s, "s1", "s1")`.
pub fn ssolid(s: &mut dyn Session, shell: &str, solid: &str) {
    let shape = bound_shape(s, shell);
    assert_eq!(
        shape.shape_type().unwrap(),
        ShapeEnum::Shell,
        "ssolid {shell}"
    );
    if let Some(tshape) = shape.tshape().get()
        && !tshape.free()
    {
        println!("Shape non Free -> Freeing");
        tshape.set_free(true);
    }
    let mut shell_part = topods::shell(&shape).unwrap();
    let builder = BRepBuilder::default();
    let mut made = Solid::default();
    builder.make_solid(&mut made);
    builder.add(&mut made, &shell_part).unwrap();
    let mut classifier = BRepClass3dSolidClassifier::with_shape(made.as_ref()).unwrap();
    classifier.perform_infinite_point(1.0e-7).unwrap();
    if classifier.state() == State::In {
        println!("NB : Shell to be reversed");
        let mut reversed = Solid::default();
        builder.make_solid(&mut reversed);
        shell_part.reverse();
        builder.add(&mut reversed, &shell_part).unwrap();
        made = reversed;
    }
    s.bind(solid, made.as_ref());
}

/// `mkvolume result shapes`; `mkvolume(s, "s", &["b1", "b2"])`.
pub fn mkvolume(s: &mut dyn Session, result: &str, parts: &[&str]) {
    mkvolume_intersecting(s, result, parts, true);
}

/// `mkvolume result shapes -ni`; `mkvolume_as_given(s, "r", &["f1", "f2"])`.
pub fn mkvolume_as_given(s: &mut dyn Session, result: &str, parts: &[&str]) {
    mkvolume_intersecting(s, result, parts, false);
}

fn mkvolume_intersecting(s: &mut dyn Session, result: &str, parts: &[&str], intersect: bool) {
    let arguments: List<Shape> = parts.iter().map(|name| bound_shape(s, name)).collect();
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&arguments);
    maker.set_intersect(intersect);
    maker.options_mut().set_run_parallel(false);
    maker.options_mut().set_use_obb(false);
    maker.perform();
    crate::common_operation::report_result_alerts(maker.options());
    assert!(!maker.options().has_errors(), "mkvolume {result}: errors");
    s.bind(result, maker.shape());
}

/// `halfspace name shape x y z`, of a face or a shell; `halfspace_of(s, "hs", "shell", [0., 0., 1.])`.
pub fn halfspace_of(s: &mut dyn Session, name: &str, boundary: &str, at: [f64; 3]) {
    let boundary = bound_shape(s, boundary);
    let point = Point3::new(at[0], at[1], at[2]);
    let maker = match boundary.shape_type().unwrap() {
        ShapeEnum::Face => {
            BRepPrimAPIMakeHalfSpace::with_face(&topods::face(&boundary).unwrap(), point)
        }
        ShapeEnum::Shell => {
            BRepPrimAPIMakeHalfSpace::with_shell(&topods::shell(&boundary).unwrap(), point)
        }
        other => panic!("halfspace {name}: expected a face or a shell, got {other:?}"),
    }
    .unwrap_or_else(|failure| panic!("halfspace {name}: {failure:?}"));
    s.bind(name, &maker.solid().unwrap().clone().into_shape());
}

/// `revol name base 0 0 0 dx dy dz degrees`, then `savehistory`: returns the revolution's history.
/// Example: `let history = revol_with_history(s, "r", "f12", [0., 0., 1.], 180.);`.
pub fn revol_with_history(
    s: &mut dyn Session,
    name: &str,
    base: &str,
    axis: [f64; 3],
    degrees: f64,
) -> Handle<History> {
    let shape = bound_shape(s, base);
    let line = Axis1::new(
        Point3::new(0., 0., 0.),
        Direction3::new(axis[0], axis[1], axis[2]).unwrap(),
    );
    let mut maker = BRepPrimAPIMakeRevol::new(&shape, line, degrees * (PI / 180.), false)
        .unwrap_or_else(|failure| panic!("revol {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
    Handle::new(History::from_algorithm(&[shape], &mut MakerHistory(&mut maker)).unwrap())
}

/// A shape maker as a history source; `History::from_algorithm(&a, &mut MakerHistory(&mut maker))`.
struct MakerHistory<'a, T: BRepBuilderAPIMakeShape>(&'a mut T);

impl<T: BRepBuilderAPIMakeShape> HistoryAlgorithm for MakerHistory<'_, T> {
    fn is_deleted(&mut self, shape: &Shape) -> Result<bool, Failure> {
        self.0.is_deleted(shape)
    }

    fn modified(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.modified(shape)?.iter().cloned().collect())
    }

    fn generated(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.generated(shape)?.iter().cloned().collect())
    }
}

/// `revol name base x y z dx dy dz degrees 1`: the base copied first.
/// Example: `revol_copied(s, "r1", "f", [0.; 3], [1., 0., 0.], 360.)`.
pub fn revol_copied(
    s: &mut dyn Session,
    name: &str,
    base: &str,
    at: [f64; 3],
    axis: [f64; 3],
    degrees: f64,
) {
    let shape = bound_shape(s, base);
    let line = Axis1::new(
        Point3::new(at[0], at[1], at[2]),
        Direction3::new(axis[0], axis[1], axis[2]).unwrap(),
    );
    let mut maker = BRepPrimAPIMakeRevol::new(&shape, line, degrees * (PI / 180.), true)
        .unwrap_or_else(|failure| panic!("revol {name} {degrees}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `revol name base 0 0 0 1 0 0 degrees 1`: an unbound name when the maker refuses, as the command.
/// Example: `revol_copy(s, "r10", "e", 10.)`.
pub fn revol_copy(s: &mut dyn Session, name: &str, base: &str, degrees: f64) {
    let shape = bound_shape(s, base);
    let line = Axis1::new(
        Point3::new(0., 0., 0.),
        Direction3::new(1., 0., 0.).unwrap(),
    );
    let Ok(mut maker) = BRepPrimAPIMakeRevol::new(&shape, line, degrees * (PI / 180.), true) else {
        return;
    };
    if let Ok(result) = maker.shape() {
        s.bind(name, result);
    }
}

/// `build3d name`; `build3d(s, "s1")`.
pub fn build3d(s: &mut dyn Session, name: &str) {
    let shape = bound_shape(s, name);
    brep_lib::build_curves3d_default(&shape)
        .unwrap_or_else(|failure| panic!("build3d {name}: {failure:?}"));
    s.bind(name, &shape);
}

/// `shape name CS`: an empty compsolid; `empty_compsolid(s, "b1")`.
pub fn empty_compsolid(s: &mut dyn Session, name: &str) {
    let builder = BRepBuilder::default();
    let mut whole = CompSolid::default();
    builder.make_comp_solid(&mut whole);
    s.bind(name, &whole.into_shape());
}

/// `invert name`: the same shape, its direct children reversed.
/// Example: `invert(s, "b")`.
pub fn invert(s: &mut dyn Session, name: &str) {
    let shape = bound_shape(s, name);
    let mut inverted = shape.empty_copied().unwrap();
    if let (Some(from), Some(to)) = (shape.tshape().get(), inverted.tshape().get()) {
        to.set_closed(from.closed());
    }
    let builder = BRepBuilder::default();
    for child in children_of(&shape) {
        builder.add(&mut inverted, &child.reversed()).unwrap();
    }
    s.bind(name, &inverted);
}

/// `edgestowire wire edges`: the edges of the compound oriented along one wire.
/// Example: `edgestowire(s, "result", "edges")`.
pub fn edgestowire(s: &mut dyn Session, wire: &str, edges: &str) {
    let mut shape = bound_shape(s, edges);
    orient_edges_on_wire(&mut shape)
        .unwrap_or_else(|failure| panic!("edgestowire {wire}: {failure:?}"));
    s.bind(wire, &shape);
}

/// `pcone name plane r1 r2 h`; `pcone_on(s, "cone", frame, 3., 1., 10.)`.
pub fn pcone_on(s: &mut dyn Session, name: &str, frame: Axis3, r1: f64, r2: f64, height: f64) {
    let mut maker = BRepPrimAPIMakeCone::with_axes(frame.ax2().unwrap(), r1, r2, height)
        .unwrap_or_else(|failure| panic!("pcone {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `OCC25446 result first second 2`: the cut through the API; returns what the command writes.
/// Example: `occ25446(s, "r", "b", "c")`.
pub fn occ25446(s: &mut dyn Session, result: &str, first: &str, second: &str) -> String {
    let (first, second) = (bound_shape(s, first), bound_shape(s, second));
    let cut = BRepAlgoAPICut::from_shapes(&first, &second)
        .unwrap_or_else(|failure| panic!("OCC25446 {result}: {failure:?}"));
    let made = cut.shape().clone();
    s.bind(result, &made);
    println!("Test complete\n");
    "Test complete\n".to_owned()
}

/// `restore` of a script input kept under the test data's `github-issues` folder.
/// Example: `restore_issue_input(s, "issue-folder/solid.brep.txt", "body")`.
pub fn restore_issue_input(s: &mut dyn Session, file: &str, name: &str) {
    let root = std::env::var_os("MARTENSITE_TEST_DATA_DIR").map_or_else(
        || {
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../external/martensite-test-data")
        },
        std::path::PathBuf::from,
    );
    let path = root.join("github-issues").join(file);
    let bytes = std::fs::read(&path).unwrap_or_else(|cause| panic!("{path:?}: {cause}"));
    let end = bytes.iter().position(u8::is_ascii_whitespace).unwrap_or(0);
    let mut shape = Shape::default();
    let mut stream = martensite_tkernel::storage::IoStream::from_bytes(bytes[end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap_or_else(|failure| panic!("restore {file}: {failure:?}"));
    s.bind(name, &shape);
}

/// `breducetolerance name`; `breducetolerance(s, "a_2")`.
pub fn breducetolerance(s: &mut dyn Session, name: &str) {
    let shape = bound_shape(s, name);
    reduce_vertex_tolerances(&shape)
        .unwrap_or_else(|failure| panic!("breducetolerance {name}: {failure:?}"));
    s.bind(name, &shape);
}

/// `removeinternals name [force]`; `removeinternals(s, "r", false)`.
pub fn removeinternals(s: &mut dyn Session, name: &str, force: bool) {
    let mut shape = bound_shape(s, name);
    brep_tools::remove_internals(&mut shape, force)
        .unwrap_or_else(|failure| panic!("removeinternals {name}: {failure:?}"));
    s.bind(name, &shape);
}

/// `fixshape result name`: the default shape fix; `fixshape(s, "rr", "r_1")`.
pub fn fixshape(s: &mut dyn Session, result: &str, name: &str) {
    let fix = ShapeFixShape::default();
    fix.init(bound_shape(s, name));
    fix.perform_with_progress(&ProgressRange::default())
        .unwrap_or_else(|failure| panic!("fixshape {result}: {failure:?}"));
    s.bind(result, &fix.shape());
}

/// `fuseedge name`: binds `name_1`, as the command does.
/// Example: `fuseedge(s, "result")`.
pub fn fuseedge(s: &mut dyn Session, name: &str) {
    let mut fuser = BRepLibFuseEdges::new(&bound_shape(s, name), false)
        .unwrap_or_else(|failure| panic!("fuseedge {name}: {failure:?}"));
    fuser.set_concat_bspl(true);
    let vertices = fuser.nb_vertices().unwrap();
    if vertices == 0 {
        println!("no vertices to remove");
        return;
    }
    println!("{vertices} vertices to be removed");
    let target = format!("{name}_1");
    println!("{target} ");
    s.bind(&target, fuser.shape().unwrap());
}

/// The direct children of `shape`; `children_of(&compound)`.
pub fn children_of(shape: &Shape) -> Vec<Shape> {
    let mut iterator = ShapeIterator::new(shape, true, true);
    let mut children = Vec::new();
    while let Ok(child) = iterator.value() {
        children.push(child.clone());
        iterator.next();
    }
    children
}

/// The tolerance of a vertex; `vertex_tolerance(&shape)`.
pub fn vertex_tolerance(shape: &Shape) -> f64 {
    BRepTool::vertex_tolerance(&topods::vertex(shape).unwrap()).unwrap()
}

/// A compound of `parts`; `compound_of(&parts)`.
pub fn compound_of(parts: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut whole = Compound::default();
    builder.make_compound(&mut whole);
    let mut whole = whole.into_shape();
    for part in parts {
        builder.add(&mut whole, part).unwrap();
    }
    whole
}

/// The vertices of `shape`, as `explode name v` orders them; `vertices_of(&edge)`.
pub fn vertices_of(shape: &Shape) -> Vec<Shape> {
    distinct_parts(shape, ShapeEnum::Vertex)
}
