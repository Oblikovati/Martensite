//! `tests/perf/moddata/bug26884`: a cylinder pair whose intersection is timed and comes out empty.
use crate::support::{
    bopcurves::{BopCurvesOptions, bopcurves_bound},
    perf_commands::Chrono,
    restore::restore,
    session::Session,
};

pub fn bug26884(s: &mut dyn Session) {
    println!("================");
    println!("OCC26884");
    println!("================");
    println!();
    restore(s, "bug26884-f1.brep", "f1");
    restore(s, "bug26884-f2.brep", "f2");
    let mut chrono = Chrono::started();
    let log = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::TWO_D);
    chrono.stop();
    chrono.counter("bopcurves");
    if !log.lines.is_empty() || !log.point_names.is_empty() {
        println!("Error: wrong output");
    }
}
