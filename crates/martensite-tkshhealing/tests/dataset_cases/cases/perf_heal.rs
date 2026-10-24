//! perf heal regression cases owned by
//! TKShHealing whose inputs are in the public dataset: the timed command runs between
//! `dchrono name restart` and `dchrono name stop counter text`, which prints its CPU time
//! (the perf group end only prints `TEST COMPLETED`).
use crate::support::{
    chrono::Dchrono, log, qa_projection::occ24008, restore::restore_as, session::Session,
};

/// OCC26871: projecting the curve on the surface (`OCC24008`) is timed.
pub fn bug26871(s: &mut dyn Session) {
    for line in ["=======", "OCC26871", "=======", ""] {
        log::puts(line);
    }
    let c3d = restore_as(s, "bug26871_curve3d", "c3d").curve();
    let surf = restore_as(s, "bug26871_surface", "surf").surface();
    let chrono = Dchrono::restart();
    log::result(&occ24008(&c3d, &surf));
    chrono.stop_counter("OCC24008");
}
