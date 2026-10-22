//! The tests/sewing commands and checks: BRepTest `sewing result tol shapes...`
//! (BRepBuilderAPI_Sewing with its Dump), SWscenario `tolerance shape`
//! (ShapeAnalysis_ShapeTolerance) and `freebounds shape tol` (ShapeAnalysis_FreeBounds),
//! and the CheckCommands.tcl procs `checkmaxtol`, `checkfreebounds` and `checkfaults`
//! built on them.
use super::checkshape::checkshape;
use super::commands::explode_type;
use super::log;
use super::logged_checks::checkreal;
use super::printed::g17;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{message::ProgressRange, precision};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_analysis::{
    ShapeAnalysisFreeBounds, ShapeAnalysisShapeTolerance,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPISewing;

/// `sewing result tol shapes...` with the default switches: Init(tol, sewing, analysis,
/// cutting, no non-manifold), same parameter on, no floating edges, faces on, the minimal
/// tolerance 1e-4 of tol (at least Precision::Confusion()) and no maximal one; its Dump is
/// logged. `None` when the sewed shape is null (the command then sets nothing).
pub fn sewing(tolerance: f64, shapes: &[&Shape]) -> Option<Shape> {
    sewing_with(tolerance, shapes, &[])
}

/// `sewing result tol shapes... switches`: `+x`/`-x` turn the sewing (s), analysis (a),
/// cutting (c), non-manifold (n), same parameter (p), floating edges (e) and face (f)
/// modes on or off.
pub fn sewing_with(tolerance: f64, shapes: &[&Shape], switches: &[&str]) -> Option<Shape> {
    let mut modes = SewingModes::default();
    for switch in switches {
        modes.apply(switch);
    }
    let tolerance = tolerance.max(precision::confusion());
    let min_tolerance = (tolerance * 1e-4).max(precision::confusion());
    let mut sewer = BRepBuilderAPISewing::new(
        tolerance,
        modes.sewing,
        modes.analysis,
        modes.cutting,
        modes.non_manifold,
    );
    sewer.set_same_parameter_mode(modes.same_parameter);
    sewer.set_floating_edges_mode(modes.floating_edges);
    sewer.set_face_mode(modes.faces);
    sewer.set_min_tolerance(min_tolerance);
    sewer.set_max_tolerance(precision::infinite());
    for shape in shapes {
        sewer.add(shape).unwrap();
    }
    sewer
        .perform(&ProgressRange::default())
        .unwrap_or_else(|failure| panic!("sewing: {failure:?}"));
    let mut dump = Vec::new();
    sewer.dump(&mut dump).unwrap();
    log::puts(String::from_utf8_lossy(&dump).trim_end_matches('\n'));
    let sewed = sewer.sewed_shape().clone();
    (!sewed.is_null()).then_some(sewed)
}

/// The `sewing` mode switches and their defaults.
struct SewingModes {
    sewing: bool,
    analysis: bool,
    cutting: bool,
    non_manifold: bool,
    same_parameter: bool,
    floating_edges: bool,
    faces: bool,
}

impl Default for SewingModes {
    fn default() -> Self {
        Self {
            sewing: true,
            analysis: true,
            cutting: true,
            non_manifold: false,
            same_parameter: true,
            floating_edges: false,
            faces: true,
        }
    }
}

impl SewingModes {
    fn apply(&mut self, switch: &str) {
        let on = switch.starts_with('+');
        match switch.as_bytes().get(1).map(u8::to_ascii_lowercase) {
            Some(b's') => self.sewing = on,
            Some(b'a') => self.analysis = on,
            Some(b'c') => self.cutting = on,
            Some(b'n') => self.non_manifold = on,
            Some(b'p') => self.same_parameter = on,
            Some(b'e') => self.floating_edges = on,
            Some(b'f') => self.faces = on,
            _ => panic!("sewing switch {switch:?}: the -mint/-maxt values are not ported here"),
        }
    }
}

/// `tolerance shape`: the MAX/AVG/MIN of all sub-shapes, then of the faces, edges and
/// vertices, each `%.17g` (Draw_Interpretor::Append of a real).
pub fn tolerance_text(shape: &Shape) -> String {
    let rows = [
        ("Tolerance MAX=", ShapeEnum::Shape),
        ("FACE    : MAX=", ShapeEnum::Face),
        ("EDGE    : MAX=", ShapeEnum::Edge),
        ("VERTEX  : MAX=", ShapeEnum::Vertex),
    ];
    let mut analysis = ShapeAnalysisShapeTolerance::new();
    let mut text = String::new();
    for (title, kind) in rows {
        analysis.init_tolerance();
        analysis.add_tolerance(shape, kind).unwrap();
        let g = |mode| g17(analysis.global_tolerance(mode));
        text.push_str(&format!("{title}{} AVG={} MIN={}\n", g(1), g(0), g(-1)));
    }
    text
}

/// `checkmaxtol shape -ref value`: the `Tolerance MAX=` of `tolerance shape` checked by
/// `checkreal "Max tolerance" max ref 0.0001 0.01`; returns the maximum.
pub fn checkmaxtol(shape: &Shape, name: &str, reference: f64) -> f64 {
    log::puts(&format!("checkmaxtol {name} -ref {reference}"));
    let max = max_of(shape, name);
    checkreal("Max tolerance", max, reference, 0.0001, 0.01);
    max
}

/// `checkmaxtol shape` without options: the `Tolerance MAX=` of `tolerance shape`.
pub fn checkmaxtol_value(shape: &Shape, name: &str) -> f64 {
    log::puts(&format!("checkmaxtol {name} "));
    max_of(shape, name)
}

/// `checkmaxtol shape -min_tol t`: an Error when the maximum exceeds `t`.
pub fn checkmaxtol_min(shape: &Shape, name: &str, min_tol: f64) -> f64 {
    log::puts(&format!("checkmaxtol {name} -min_tol {min_tol}"));
    let max = max_of(shape, name);
    if min_tol != 0. && max > min_tol {
        log::puts(&format!(
            "Error: tolerance of \"{name}\" ({max}) is greater than checking tolerance ({min_tol})"
        ));
    }
    max
}

fn max_of(shape: &Shape, name: &str) -> f64 {
    let text = tolerance_text(shape);
    log::result(&text);
    text.strip_prefix("Tolerance MAX=")
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|word| word.parse::<f64>().ok())
        .unwrap_or_else(|| panic!("Error: cannot get tolerances of shape \"{name}\""))
}

/// `freebounds shape tol`: `name_c` (closed wires) and `name_o` (open wires) of
/// ShapeAnalysis_FreeBounds, actual bounds for tol <= 0 and forecast ones otherwise.
pub fn freebounds(shape: &Shape, name: &str, tolerance: f64) -> (Shape, Shape) {
    let bounds = if tolerance <= 0. {
        ShapeAnalysisFreeBounds::from_shape(shape, false, false, false)
    } else {
        ShapeAnalysisFreeBounds::forecast(shape, tolerance, false, false)
    }
    .unwrap_or_else(|failure| panic!("freebounds: {failure:?}"));
    log::result(&format!("{name}_c - closed wires\n{name}_o - open wires\n"));
    (
        bounds.closed_wires().clone().into_shape(),
        bounds.open_wires().clone().into_shape(),
    )
}

/// `checkfreebounds shape ref`: `freebounds shape -0.01`, then the number of edges of
/// `shape_c` (the closed wires, the default `-type`) must be `ref` (-1: UNSTABLE).
pub fn checkfreebounds(shape: &Shape, name: &str, reference: i64) {
    log::puts(&format!("checkfreebounds {name} {reference} "));
    let (closed, _open) = freebounds(shape, name, -0.01);
    let edges = explode_type(&closed, ShapeEnum::Edge).len();
    log::result(
        &(1..=edges)
            .map(|i| format!("{name}_c_{i}"))
            .collect::<Vec<_>>()
            .join(" "),
    );
    if reference == -1 {
        log::puts("Error : Number of free edges is UNSTABLE");
        return;
    }
    if edges as i64 != reference {
        log::puts("Error : Number of free edges is not equal to reference data");
    } else {
        log::puts(&format!("OK : Number of free edges is {edges}"));
    }
}

/// `checkfaults shape source ref`: the faulty-shape counts of `checkshape` of both,
/// reported; an Error when the result has more than the source (ref -1: UNSTABLE).
pub fn checkfaults(shape: &Shape, name: &str, source: &Shape, source_name: &str, reference: i64) {
    log::puts(&format!("checkfaults {name} {source_name} {reference}"));
    let nb_a = checkshape(source).nb_faulty;
    let nb_r = checkshape(shape).nb_faulty;
    log::puts(&format!(
        "Number of faults for the initial shape is {nb_a}."
    ));
    log::puts(&format!(
        "Number of faults for the resulting shape is {nb_r}."
    ));
    if reference == -1 {
        log::puts("Error : Number of faults is UNSTABLE");
        return;
    }
    if nb_r > nb_a {
        log::puts(&format!("Error : Number of faults is {nb_r}"));
    }
}
