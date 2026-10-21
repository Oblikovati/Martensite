//! BRepTest_ExtremaCommands.cxx `distmini name s1 s2 [deflection]` and
//! BOPTest_LowCommands.cxx `bclassify`/`b2dclassify`.
use super::printed::g17;
use super::session::Session;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::{message::ProgressRange, precision};
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tktopalgo::{
    brep_class::BRepClassFaceClassifier,
    brep_class3d::BRepClass3dSolidClassifier,
    brep_extrema::BRepExtremaDistShapeShape,
    brep_lib::{BRepLibMakeEdge, BRepLibMakeVertex},
};

/// What `distmini` sets: `<name>_val` and the solution shapes `<name>`, `<name>2`, ...
#[derive(Clone, Debug)]
pub struct Distmini {
    pub value: f64,
    pub solutions: Vec<Shape>,
    /// The Tcl result text (`"distmini" command returns: ...`).
    pub text: String,
}

/// `distmini name s1 s2 [deflection]`: `BRepExtrema_DistShapeShape` with the deflection
/// (Precision::Confusion by default); each solution is a vertex at the first point when
/// the distance is at most 1e-9, else the edge between the two points. `None` is the
/// command's `problem: no distance is found`. The variables are set in `session`.
pub fn distmini(
    session: &mut dyn Session,
    name: &str,
    s1: &Shape,
    s2: &Shape,
    deflection: Option<f64>,
) -> Option<Distmini> {
    let mut dst = BRepExtremaDistShapeShape::new();
    dst.load_s1(s1).unwrap();
    dst.load_s2(s2).unwrap();
    dst.set_deflection(deflection.unwrap_or(precision::confusion()));
    dst.set_multi_thread(false);
    dst.perform(&ProgressRange::default()).unwrap();
    if !dst.is_done() {
        super::log::result("problem: no distance is found\n");
        return None;
    }
    let value = dst.value().unwrap();
    let mut text = format!("\"distmini\" command returns:\n{name}_val ");
    let mut solutions = Vec::new();
    for i in 1..=dst.nb_solution() {
        let p1 = dst.point_on_shape1(i).unwrap();
        let p2 = dst.point_on_shape2(i).unwrap();
        let shape = if value <= 1.0e-9 {
            BRepLibMakeVertex::new(p1)
                .unwrap()
                .vertex()
                .unwrap()
                .into_shape()
        } else {
            let edge = BRepLibMakeEdge::from_points(p1, p2).unwrap();
            edge.shape().unwrap().clone()
        };
        let var = if i == 1 {
            name.to_string()
        } else {
            format!("{name}{i}")
        };
        session.set(&var, &shape.clone().into());
        text.push_str(&var);
        text.push(' ');
        solutions.push(shape);
    }
    text.push_str("\nOutput is complete.\n");
    super::log::result(&text);
    println!("{name}_val = {}", g17(value));
    Some(Distmini {
        value,
        solutions,
        text,
    })
}

/// BOPTest_LowCommands.cxx `PrintState`.
pub fn state_text(state: State) -> &'static str {
    match state {
        State::In => "The point is IN shape\n",
        State::Out => "The point is OUT of shape\n",
        State::On => "The point is ON shape\n",
        _ => "The point is UNKNOWN shape\n",
    }
}

/// `bclassify solid point [tol = 1e-7]`: `BRepClass3d_SolidClassifier(S).Perform(P, tol)`.
pub fn bclassify(solid: &Shape, point: Point3, tolerance: f64) -> State {
    let mut classifier = BRepClass3dSolidClassifier::with_shape(solid).unwrap();
    classifier.perform(&point, tolerance).unwrap();
    let state = classifier.state();
    super::log::result(state_text(state));
    state
}

/// `bclassify solid point`.
pub fn bclassify_default(solid: &Shape, point: Point3) -> State {
    bclassify(solid, point, 1.0e-7)
}

/// `b2dclassify face point2d`: `BRepClass_FaceClassifier::Perform(F, P, tol =
/// BRep_Tool::Tolerance(F), UseBox = false, GapCheckTol = 0.1)`.
pub fn b2dclassify(face: &Shape, point: Point2) -> State {
    let face = topods::face(face).expect("b2dclassify: a face");
    let tolerance = martensite_tkbrep::brep::BRepTool::face_tolerance(&face).unwrap();
    let mut classifier = BRepClassFaceClassifier::new();
    classifier
        .perform(&face, &point, tolerance, false, 0.1)
        .unwrap();
    let state = classifier.state();
    super::log::result(state_text(state));
    state
}
