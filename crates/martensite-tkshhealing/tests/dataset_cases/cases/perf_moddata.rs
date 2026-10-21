//! tests/perf/moddata case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 owned by
//! TKShHealing whose inputs are in the public dataset.
use crate::support::{
    chrono::Dchrono,
    log,
    restore::restore_as,
    session::{Session, set},
    swdraw::fixshape,
};

/// OCC26339: `fixshape r f 1e-5` is timed (the script's `max_time` is never compared).
pub fn bug26339(s: &mut dyn Session) {
    for line in ["============", "OCC26339", "============", ""] {
        log::puts(line);
    }
    let f = restore_as(s, "bug26339_a_1886.brep", "f").shape();
    let chrono = Dchrono::restart();
    set(s, "r", fixshape(&f, &["1e-5"]));
    chrono.stop_counter("fixshape");
}
