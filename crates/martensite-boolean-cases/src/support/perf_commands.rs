//! The commands of `tests/perf`, and `dchrono` where another group's script calls it.
use super::{
    checkshape::checkshape, commands::bound_shape, script_error::script_error, session::Session,
};
use crate::common_operation::{performed_direct_parallel, report_result_alerts};
use martensite_tkbo::{bop_algo::Operation, bop_tools::compute_curve_tolerance};
use martensite_tkbrep::topods;
use martensite_tkernel::{
    ncollection::Array1, osd::Timer, precision, standard::Handle, storage::format_general,
};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomBezierCurve, GeomCurve};
use martensite_tkgeombase::geom_convert::curve_to_bspline_curve;
use martensite_tkmath::{convert::ParameterisationType, gp::Point3};

/// `dchrono name`: the timer of a script, which only reports on what it times.
/// Example: `let mut chrono = Chrono::started();`.
pub struct Chrono {
    timer: Timer,
}

impl Chrono {
    /// `dchrono name reset`, `dchrono name start`; `Chrono::started()`.
    pub fn started() -> Self {
        let mut timer = Timer::new(false);
        timer.reset();
        timer.start();
        Self { timer }
    }

    /// `dchrono name stop`; `chrono.stop()`.
    pub fn stop(&mut self) {
        self.timer.stop();
    }

    /// `dchrono name start`, after a stop; `chrono.resume()`.
    pub fn resume(&mut self) {
        self.timer.start();
    }

    /// `dchrono name counter label`: the CPU time; `chrono.counter("Bug25950IntSS")`.
    pub fn counter(&self, label: &str) {
        println!(
            "COUNTER {label}: {}",
            format_general(self.cpu_seconds(), 17)
        );
    }

    /// The CPU time of `dchrono name`, for what a script reads back; `chrono.cpu_seconds()`.
    pub fn cpu_seconds(&self) -> f64 {
        self.timer.show_values().3
    }

    /// `dchrono name show`: the three lines it writes; `chrono.show()`.
    pub fn show(&self) {
        print!("{}", self.timer.show_to());
    }
}

/// `tests/perf/bop/end`: `checkshape result`, when bound.
/// Example: `bop_end(session)`.
pub fn bop_end(session: &mut dyn Session) {
    if let Some(result) = session.shape("result") {
        checkshape(result);
    }
}

/// `brunparallel 1`, then `bcut result first second`; `bcut_parallel(s, "r", "b1", "b2")`.
pub fn bcut_parallel(s: &mut dyn Session, result: &str, first: &str, second: &str) {
    let (first, second) = (bound_shape(s, first), bound_shape(s, second));
    let operation =
        performed_direct_parallel(&first, &second, Operation::Cut, precision::confusion());
    report_result_alerts(operation.options());
    if operation.options().has_errors() {
        return;
    }
    if operation.shape().is_null() {
        println!(" null shape");
        return;
    }
    s.bind(result, operation.shape());
}

/// `beziercurve c 4 ...`, `convert name c`, then the script's `insertknot` of each multiple of `step`.
/// Example: `bezier_as_spline(98, 0.01)`.
pub fn bezier_as_spline(count: i32, step: f64) -> Handle<GeomBSplineCurve> {
    let points = [
        Point3::new(0., 0., 0.),
        Point3::new(1., 1., 0.),
        Point3::new(2., 1., 0.),
        Point3::new(3., 0., 0.),
    ];
    let bezier: Handle<dyn GeomCurve> =
        Handle::new(GeomBezierCurve::new(&Array1::from_shared(&points, 1)).unwrap()).cast();
    let curve = curve_to_bspline_curve(&bezier, ParameterisationType::TgtThetaOver2)
        .unwrap_or_else(|failure| panic!("convert: {failure:?}"));
    for i in 1..=count {
        let at = step * f64::from(i);
        curve
            .insert_knot(at, 1, precision::p_confusion(), true)
            .unwrap_or_else(|failure| panic!("insertknot {at}: {failure:?}"));
    }
    curve
}

/// `xdistef edge face`: writes what it measures.
/// Example: `xdistef(s, "p1_3", "p1")`.
pub fn xdistef(s: &dyn Session, edge: &str, face: &str) {
    let on = topods::edge(&bound_shape(s, edge))
        .unwrap_or_else(|failure| panic!("xdistef {edge}: {failure:?}"));
    let of = topods::face(&bound_shape(s, face))
        .unwrap_or_else(|failure| panic!("xdistef {face}: {failure:?}"));
    let (mut distance, mut parameter) = (0.0, 0.0);
    let measured = compute_curve_tolerance(&of, &on, &mut distance, &mut parameter)
        .unwrap_or_else(|failure| panic!("xdistef {edge} {face}: {failure:?}"));
    if !measured {
        println!("Tolerance cannot be computed");
        script_error(&format!(
            "xdistef {edge} {face}: Tolerance cannot be computed"
        ));
    }
    println!(
        "Max Distance = {}; Parameter on curve = {}",
        format_general(distance, 17),
        format_general(parameter, 17)
    );
}
