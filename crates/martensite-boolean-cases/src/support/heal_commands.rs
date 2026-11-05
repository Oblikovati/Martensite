//! The commands and `end` scripts of `tests/heal`, `tests/mkface` and `tests/bugs/heal`. One owner.
use super::{
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview},
    commands::{bound_shape, distinct_parts, explode},
    nbshapes::{Counting, shape_counts},
    session::Session,
    tolerance_check::tolerance_report,
};
use crate::case_bopcheck::{complete_response, configured_checker};
use martensite_tkbo::bop_algo::tools::{edges_to_wires, wires_to_faces};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::History,
    topods::{Compound, Shape},
    toptools::MapOfShape,
};
use martensite_tkernel::{message::ProgressRange, precision, standard::Handle};
use martensite_tkg3d::{
    geom::{GeomCurve, GeomLine},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Axis1, Axis3, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere};
use martensite_tkshhealing::{
    shape_fix::ShapeFix,
    shape_upgrade::{ShapeUpgradeShapeDivideArea, ShapeUpgradeUnifySameDomain},
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use std::cell::RefCell;

/// The scripts' `*pi/180`.
fn radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

pub mod unify_end;
pub use unify_end::unify_same_domain_end;

thread_local! {
    /// The session's history: what the last `unifysamedom` left, for `savehistory` to read.
    static SESSION_HISTORY: RefCell<Option<Handle<History>>> = const { RefCell::new(None) };
}

/// `psphere name radius angle1 angle2`; `psphere_band(s, "a", 10., 0., 90.)`.
pub fn psphere_band(s: &mut dyn Session, name: &str, radius: f64, first: f64, second: f64) {
    let mut maker = BRepPrimAPIMakeSphere::with_latitudes(radius, radians(first), radians(second))
        .unwrap_or_else(|failure| panic!("psphere {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `pcylinder name radius height angle`; `pcylinder_wedge(s, "a", 10., 30., 180.)`.
pub fn pcylinder_wedge(s: &mut dyn Session, name: &str, radius: f64, height: f64, angle: f64) {
    let mut maker = BRepPrimAPIMakeCylinder::with_angle(radius, height, radians(angle))
        .unwrap_or_else(|failure| panic!("pcylinder {name}: {failure:?}"));
    s.bind(name, maker.shape().unwrap());
}

/// `plane pp x y z dx dy dz`, then `pcylinder name pp radius height`.
/// Example: `pcylinder_on(s, "b1", ([-20., 0., 10.], [1., 0., 0.]), 3., 40.)`.
pub fn pcylinder_on(
    s: &mut dyn Session,
    name: &str,
    plane: ([f64; 3], [f64; 3]),
    radius: f64,
    height: f64,
) {
    let (at, normal) = plane;
    let along = Direction3::new(normal[0], normal[1], normal[2])
        .unwrap_or_else(|failure| panic!("plane {normal:?}: {failure:?}"));
    let placed = Axis3::from_normal(Point3::new(at[0], at[1], at[2]), along)
        .and_then(|frame| frame.ax2())
        .unwrap_or_else(|failure| panic!("plane {at:?} {normal:?}: {failure:?}"));
    let mut maker = BRepPrimAPIMakeCylinder::with_axes(placed, radius, height);
    s.bind(name, maker.shape().unwrap());
}

/// `DTSplitByNumber result face parts`, or with the numbers along U and along V.
/// Example: `dt_split_by_number(s, "result", "b_1", (10, None))`.
pub fn dt_split_by_number(
    s: &mut dyn Session,
    result: &str,
    face: &str,
    numbers: (i32, Option<i32>),
) {
    let input = bound_shape(s, face);
    let tool = ShapeUpgradeShapeDivideArea::new(input);
    tool.set_splitting_by_number(true);
    match numbers {
        (parts, None) => tool.set_nb_parts(parts),
        (u, Some(v)) => tool
            .set_numbers_uv_splits(u, v)
            .unwrap_or_else(|failure| panic!("DT_SplitByNumber {u} {v}: {failure:?}")),
    }
    tool.perform(true)
        .unwrap_or_else(|failure| panic!("DT_SplitByNumber {face}: {failure:?}"));
    let split = tool.result();
    ShapeFix::same_parameter(
        &split,
        false,
        0.0,
        &ProgressRange::default(),
        &Handle::default(),
    )
    .unwrap_or_else(|failure| panic!("DT_SplitByNumber {face}: {failure:?}"));
    s.bind(result, &split);
}

/// Beyond the script, which counts only faces: every count of the reference run. A sphere's pole
/// holds one vertex on the reference run. The Rust split leaves one per part touching the pole, so
/// this fails until `ShapeUpgradeShapeDivideArea` merges them; the row stays on its old host.
/// Example: `native_split_counts(s, [15, 30, 10, 10, 1, 0, 0, 0, 66])`.
pub fn native_split_counts(s: &dyn Session, reference: [usize; 9]) {
    let counts = shape_counts(s, "result", Counting::Typed);
    assert_eq!(
        counts, reference,
        "known difference from the reference run: vertices at the sphere's pole are not merged"
    );
}

/// The options of `unifysamedom` these scripts use.
#[derive(Clone, Copy, Debug)]
pub struct UnifyOptions {
    /// Without `-e`.
    pub unify_edges: bool,
    /// `+i`.
    pub allow_internal: bool,
}

impl Default for UnifyOptions {
    fn default() -> Self {
        Self {
            unify_edges: true,
            allow_internal: false,
        }
    }
}

/// `unifysamedom result shape [keep...] [-e] [+i]`: the result, and the session's history.
/// Example: `unifysamedom(s, "result", "a", &["a_3"], UnifyOptions::default())`.
pub fn unifysamedom(
    s: &mut dyn Session,
    result: &str,
    name: &str,
    keep: &[&str],
    options: UnifyOptions,
) {
    let shape = bound_shape(s, name);
    let mut kept = MapOfShape::default();
    for part in keep {
        kept.add(bound_shape(s, part));
    }
    let unifier = ShapeUpgradeUnifySameDomain::new(&shape, options.unify_edges, true, false);
    let refused = |failure| panic!("unifysamedom {result}: {failure:?}");
    unifier.keep_shapes(&kept).unwrap_or_else(refused);
    unifier.set_safe_input_mode(true);
    unifier.allow_internal_edges(options.allow_internal);
    unifier.set_linear_tolerance(precision::confusion());
    unifier.set_angular_tolerance(precision::angular());
    unifier.build().unwrap_or_else(refused);
    s.bind(result, &unifier.shape());
    SESSION_HISTORY.with(|history| *history.borrow_mut() = Some(unifier.history()));
}

/// `savehistory name`: the session's history.
/// Example: `let history = savehistory();`.
pub fn savehistory() -> Handle<History> {
    SESSION_HISTORY
        .with(|history| history.borrow().clone())
        .unwrap_or_else(|| panic!("savehistory: expected a history in the session"))
}

/// `isdeleted history name`: writes and returns the answer; `isdeleted(s, &history, "a_9")`.
pub fn isdeleted(s: &dyn Session, history: &Handle<History>, name: &str) -> &'static str {
    let shape = bound_shape(s, name);
    let removed = history
        .get()
        .map_or(Ok(false), |h| h.is_removed(&shape))
        .unwrap_or_else(|failure| panic!("isdeleted {name}: {failure:?}"));
    let answer = if removed { "Deleted." } else { "Not deleted." };
    println!("{answer}");
    answer
}

/// `modified result history name`: writes and returns the answer, empty when `result` is bound.
/// Example: `modified(s, "res", &history, "a_9")`.
pub fn modified(
    s: &mut dyn Session,
    result: &str,
    history: &Handle<History>,
    name: &str,
) -> &'static str {
    let shape = bound_shape(s, name);
    let shapes = history
        .get()
        .map_or(Ok(Vec::new()), |h| h.modified(&shape))
        .unwrap_or_else(|failure| panic!("modified {name}: {failure:?}"));
    if shapes.is_empty() {
        println!("The shape has not been modified.");
        return "The shape has not been modified.";
    }
    let bound = if shapes.len() == 1 {
        shapes[0].clone()
    } else {
        compound_of(&shapes)
    };
    s.bind(result, &bound);
    ""
}

/// A compound of `parts`; `compound_of(&[a, b])`.
pub fn compound_of(parts: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut whole = Compound::default();
    builder.make_compound(&mut whole);
    let mut whole = whole.into_shape();
    for part in parts {
        builder
            .add(&mut whole, part)
            .unwrap_or_else(|failure| panic!("compound: {failure:?}"));
    }
    whole
}

/// `eval compound [explode name kind] whole`: the parts' names as `explode` leaves them bound.
/// Example: `compound_exploded(s, "b1", ShapeEnum::Edge, "c")`.
pub fn compound_exploded(
    s: &mut dyn Session,
    name: &str,
    kind: ShapeEnum,
    whole: &str,
) -> Vec<Shape> {
    let parts = exploded_parts(s, name, kind);
    s.bind(whole, &compound_of(&parts));
    parts
}

/// `explode name kind`, with the parts it bound.
/// Example: `exploded_parts(s, "b1", ShapeEnum::Edge)`.
pub fn exploded_parts(s: &mut dyn Session, name: &str, kind: ShapeEnum) -> Vec<Shape> {
    explode(s, name, Some(kind));
    distinct_parts(&bound_shape(s, name), kind)
}

/// `edgestofaces result edges`, at its default tolerances: a failure writes the command's line.
/// Example: `edgestofaces(s, "result", "c")`.
pub fn edgestofaces(s: &mut dyn Session, result: &str, edges: &str) {
    let source = bound_shape(s, edges);
    let mut wires = Shape::default();
    let code = edges_to_wires(&source, &mut wires, false, 1.0e-8)
        .unwrap_or_else(|failure| panic!("edgestofaces {edges}: {failure:?}"));
    if code != 0 {
        println!("Unable to build wires from given edges");
        return;
    }
    let mut faces = Shape::default();
    let built = wires_to_faces(&wires, &mut faces, 1.0e-8)
        .unwrap_or_else(|failure| panic!("edgestofaces {edges}: {failure:?}"));
    if !built {
        println!("Unable to build faces from wires");
        return;
    }
    s.bind(result, &faces);
}

/// `line l x y z dx dy dz`, then `mkedge name l first last`.
/// Example: `mkedge_line(s, "el", [0.; 3], [1., 0., 0.], (-15., 15.))`.
pub fn mkedge_line(
    s: &mut dyn Session,
    name: &str,
    at: [f64; 3],
    along: [f64; 3],
    range: (f64, f64),
) {
    let direction = Direction3::new(along[0], along[1], along[2])
        .unwrap_or_else(|failure| panic!("line {along:?}: {failure:?}"));
    let axis = Axis1::new(Point3::new(at[0], at[1], at[2]), direction);
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(axis)).cast();
    let maker = BRepBuilderAPIMakeEdge::from_curve_params(&line, range.0, range.1)
        .unwrap_or_else(|failure| panic!("mkedge {name} {range:?}: {failure:?}"));
    s.bind(name, &maker.edge().unwrap().into_shape());
}

/// `bopcheck name`: writes the command's answer and returns whether the script's regexp finds it OK.
/// Example: `if !bopcheck_is_ok(s, "r") { println!("Error: invalid shape"); }`.
pub fn bopcheck_is_ok(s: &dyn Session, name: &str) -> bool {
    let mut checker = configured_checker(&bound_shape(s, name));
    checker.perform();
    let response = complete_response(&checker);
    println!("{response}");
    response.contains("This shape seems to be OK")
}

/// `setflags name locked`: the shape and its vertices, edges and faces.
/// Example: `setflags_locked(s, "a")`.
pub fn setflags_locked(s: &dyn Session, name: &str) {
    let shape = bound_shape(s, name);
    let locked = |part: &Shape| {
        part.tshape()
            .get()
            .unwrap_or_else(|| panic!("setflags {name}: expected a bound shape"))
            .set_locked(true);
    };
    locked(&shape);
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        distinct_parts(&shape, kind).iter().for_each(locked);
    }
}

/// The `end` of `heal/split_number` and `heal/split_two_numbers`: `puts [checkshape result]`, the
/// command's own report and the same text written once more.
/// Example: `checkshape_result_end(session)`.
pub fn checkshape_result_end(session: &mut dyn Session) {
    let result = session
        .shape("result")
        .unwrap_or_else(|| panic!("grid end: result is not a topological shape"));
    checkshape(result);
    checkshape(result);
}

/// `tests/heal/end`: the view of `result`, or `r` where a script leaves that.
/// Example: `heal_end(session)`.
pub fn heal_end(session: &mut dyn Session) {
    if session.shape("result").is_some() {
        checkview(session, &CheckView::of("result"));
    } else if session.shape("r").is_some() {
        checkview(session, &CheckView::of("r"));
    } else {
        println!("Error : the resulting shape is not done.");
    }
    println!("TEST COMPLETED");
}

/// `tests/mkface/end`: no script here sets its tolerance variables, so only validity, the
/// tolerances written twice, and the view. Example: `mkface_end(session)`.
pub fn mkface_end(session: &mut dyn Session) {
    if let Some(result) = session.shape("result") {
        checkshape(result);
        checkshape(result);
        tolerance_report(session, "result");
        tolerance_report(session, "result");
        checkview(session, &CheckView::of("result"));
    } else {
        println!("Error : The mkface can not be built.");
    }
    println!("TEST COMPLETED");
}

/// `checkshape name` for each of `names`; `checkshape_all(s, &["ru1", "ru2"])`.
pub fn checkshape_all(s: &dyn Session, names: &[&str]) {
    for name in names {
        checkshape_named(s, name);
    }
}

/// `eval compound [explode b1 e] [explode b2 e] c`: the edges of each named shape, repeated
/// where the shapes share one, as the script's list holds them.
/// Example: `compound_exploded_many(s, &["b1", "b2"], "c")`.
pub fn compound_exploded_many(s: &mut dyn Session, names: &[&str], whole: &str) {
    let mut parts = Vec::new();
    for name in names {
        parts.extend(exploded_parts(s, name, ShapeEnum::Edge));
    }
    s.bind(whole, &compound_of(&parts));
}
