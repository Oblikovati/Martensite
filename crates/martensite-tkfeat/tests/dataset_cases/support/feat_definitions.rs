//! `featprism`, `featdprism`, `featrevol`, `featlf`, `featrf` and `addslide`: each keeps its feature
//! for the `featperform` that follows, as the command file's statics do.
use super::feat_script::{FeatScript, checked, command_failed};
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkfeat::brep_feat::{
    self as brep_feat, BRepFeatMakeDPrism, BRepFeatMakeLinearForm, BRepFeatMakePrism,
    BRepFeatMakeRevol, BRepFeatMakeRevolutionForm, StatusError,
};
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Vector3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::f64::consts::PI;

/// The definitions a script made, and the inputs a history reads.
#[derive(Default)]
pub struct Features {
    pub prism: Option<BRepFeatMakePrism>,
    pub dprism: Option<BRepFeatMakeDPrism>,
    pub revol: Option<BRepFeatMakeRevol>,
    pub linear_form: Option<BRepFeatMakeLinearForm>,
    pub revolution_form: Option<BRepFeatMakeRevolutionForm>,
    /// `sbase`, `pbase` and `skface`.
    pub inputs: [Shape; 3],
}

fn direction(along: [f64; 3]) -> Direction3 {
    checked(Direction3::new(along[0], along[1], along[2]))
}

fn axis(at: [f64; 3], along: [f64; 3]) -> Axis1 {
    Axis1::new(Point3::new(at[0], at[1], at[2]), direction(along))
}

/// The basis shape: a name that holds none is the command's own refusal.
fn basis(s: &FeatScript<'_>, name: &str) -> Shape {
    let shape = s.shape_or_null(name);
    if shape.is_null() {
        command_failed("null basis shape");
    }
    shape
}

/// The profile and sketch face every sweep takes, read in the order the command reads them.
fn sweep_inputs(s: &FeatScript<'_>, profile: &str, sketch: &str) -> (Shape, Face) {
    let pbase = s.shape_or_null(profile);
    if pbase.is_null() {
        command_failed("null shape to transform");
    }
    let face = s.shape_or_null(sketch);
    if face.is_null() || !matches!(face.shape_type(), Ok(ShapeEnum::Face)) {
        command_failed("null face of Sketch");
    }
    (pbase, checked(topods::face(&face)))
}

fn keep_inputs(s: &mut FeatScript<'_>, sbase: &Shape, pbase: &Shape, skface: &Face) {
    s.features.inputs = [sbase.clone(), pbase.clone(), skface.clone().into_shape()];
}

/// `featprism base profile sketch dx dy dz fuse modify`.
/// Example: `featprism(s, "E", "cont", "E_10", [0., 0., 1.], 0, 1)`.
pub fn featprism(
    s: &mut FeatScript<'_>,
    base: &str,
    profile: &str,
    sketch: &str,
    along: [f64; 3],
    fuse: i32,
    modify: i32,
) {
    let sbase = basis(s, base);
    let (pbase, skface) = sweep_inputs(s, profile, sketch);
    keep_inputs(s, &sbase, &pbase, &skface);
    let mut feature = BRepFeatMakePrism::new();
    checked(feature.init(&sbase, &pbase, &skface, direction(along), fuse, modify != 0));
    s.features.prism = Some(feature);
}

/// `featdprism base profile sketch angle fuse modify`: the angle in the command's half degrees.
/// Example: `featdprism(s, "cyl", "cont", "cont", -10., 0, 1)`.
pub fn featdprism(
    s: &mut FeatScript<'_>,
    base: &str,
    profile: &str,
    sketch: &str,
    angle: f64,
    fuse: i32,
    modify: i32,
) {
    let sbase = basis(s, base);
    let (pbase, skface) = sweep_inputs(s, profile, sketch);
    if !matches!(pbase.shape_type(), Ok(ShapeEnum::Face)) {
        command_failed("Invalid DPrism base");
    }
    keep_inputs(s, &sbase, &pbase, &skface);
    let mut feature = BRepFeatMakeDPrism::new();
    let base_face = checked(topods::face(&pbase));
    let half = angle * PI / 360.0;
    checked(feature.init(&sbase, &base_face, &skface, half, fuse, modify != 0));
    s.features.dprism = Some(feature);
}

/// `featrevol base profile sketch x y z dx dy dz fuse modify`.
/// Example: `featrevol(s, "boite", "cont", "boite_4", [0., 0., 200.], [0., 1., 0.], 0, 1)`.
#[allow(clippy::too_many_arguments)] // One argument per script word.
pub fn featrevol(
    s: &mut FeatScript<'_>,
    base: &str,
    profile: &str,
    sketch: &str,
    at: [f64; 3],
    along: [f64; 3],
    fuse: i32,
    modify: i32,
) {
    let sbase = basis(s, base);
    let (pbase, skface) = sweep_inputs(s, profile, sketch);
    keep_inputs(s, &sbase, &pbase, &skface);
    let mut feature = BRepFeatMakeRevol::new();
    checked(feature.init(&sbase, &pbase, &skface, axis(at, along), fuse, modify != 0));
    s.features.revol = Some(feature);
}

/// The profile wire of a rib or slot, in the order the command reads its arguments.
fn rib_wire(s: &FeatScript<'_>, wire: &str) -> martensite_tkbrep::topods::Wire {
    let shape = s.shape_or_null(wire);
    if shape.is_null() || !matches!(shape.shape_type(), Ok(ShapeEnum::Wire)) {
        command_failed("null profile for rib or slot");
    }
    checked(topods::wire(&shape))
}

fn rib_plane(s: &FeatScript<'_>, plane: &str) -> Handle<GeomPlane> {
    let surface = s.surface(plane).unwrap_or_default().cast::<GeomPlane>();
    if surface.is_null() {
        command_failed("null plane to transform");
    }
    surface
}

/// What the command writes and returns when a form's `init` is not done.
fn status_stop(error: StatusError) -> ! {
    let mut text = Vec::new();
    brep_feat::print(error, &mut text).expect("a vector takes the status text");
    command_failed(&format!("{}\n", String::from_utf8_lossy(&text)));
}

/// `featlf base wire plane dx dy dz fx fy fz fuse modify`.
/// Example: `featlf(s, "b", "pp", "pl", [0.3, 0., 0.], [-0.2, 0., 0.], 1, 1)`.
#[allow(clippy::too_many_arguments)] // One argument per script word.
pub fn featlf(
    s: &mut FeatScript<'_>,
    base: &str,
    wire: &str,
    plane: &str,
    direct: [f64; 3],
    other: [f64; 3],
    fuse: i32,
    modify: i32,
) {
    let sbase = basis(s, base);
    let (wire, plane) = (rib_wire(s, wire), rib_plane(s, plane));
    let mut feature = BRepFeatMakeLinearForm::new();
    let first = Vector3::new(direct[0], direct[1], direct[2]);
    let second = Vector3::new(other[0], other[1], other[2]);
    checked(feature.init(&sbase, &wire, &plane, first, second, fuse, modify != 0));
    if !feature.is_done() {
        status_stop(feature.rib().current_status_error());
    }
    s.features.linear_form = Some(feature);
}

/// `featrf base wire plane x y z dx dy dz h1 h2 fuse modify`.
/// Example: `featrf(s, "s", "w", "pl", [0.; 3], [0., 0., 1.], [0.2, 0.2], 1, 1)`.
#[allow(clippy::too_many_arguments)] // One argument per script word.
pub fn featrf(
    s: &mut FeatScript<'_>,
    base: &str,
    wire: &str,
    plane: &str,
    at: [f64; 3],
    along: [f64; 3],
    heights: [f64; 2],
    fuse: i32,
    modify: i32,
) {
    let sbase = basis(s, base);
    let (wire, plane) = (rib_wire(s, wire), rib_plane(s, plane));
    let mut feature = BRepFeatMakeRevolutionForm::new();
    let (h1, h2) = (heights[0], heights[1]);
    checked(feature.init(
        &sbase,
        &wire,
        &plane,
        axis(at, along),
        h1,
        h2,
        fuse,
        modify != 0,
    ));
    if !feature.is_done() {
        status_stop(feature.rib().current_status_error());
    }
    s.features.revolution_form = Some(feature);
}

/// `addslide prism edge face`; `addslide_prism(s, "cont_1", "E_3")`.
pub fn addslide_prism(s: &mut FeatScript<'_>, edge: &str, face: &str) {
    let (edge, face) = slide_pair(s, edge, face);
    let Some(feature) = s.features.prism.as_mut() else {
        script_stop();
    };
    checked(feature.add(&edge, &face));
}

/// `addslide revol edge face`; `addslide_revol(s, "p_2", "c1_1")`.
pub fn addslide_revol(s: &mut FeatScript<'_>, edge: &str, face: &str) {
    let (edge, face) = slide_pair(s, edge, face);
    let Some(feature) = s.features.revol.as_mut() else {
        script_stop();
    };
    checked(feature.add(&edge, &face));
}

fn script_stop() -> ! {
    command_failed("");
}

fn slide_pair(
    s: &FeatScript<'_>,
    edge: &str,
    face: &str,
) -> (martensite_tkbrep::topods::Edge, Face) {
    let (edge, face) = (s.shape_or_null(edge), s.shape_or_null(face));
    if edge.is_null() || face.is_null() {
        script_stop();
    }
    (checked(topods::edge(&edge)), checked(topods::face(&face)))
}
