//! tests/cr/bugs case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 owned by
//! TKShHealing whose inputs are in the public dataset.
use crate::support::{
    build::brestore,
    commands::dump_surface,
    log,
    logged_checks::checkreal,
    restore::Restored,
    session::{Session, set},
    swdraw::getanasurf,
    text::numbers_after,
};
use martensite_tkbrep::topods::Shape;

/// `set log [getanasurf name f pln tol]` then the `isdraw`/`dump`/`regexp {Plane}`
/// block; returns the log.
fn plane_of(s: &mut dyn Session, name: &str, f: &Shape, tolerance: f64) -> String {
    let (found, text) = getanasurf(f, "pln", tolerance);
    log::result(&text);
    let Some(surface) = found else {
        log::puts("Error: required surface is not got");
        return text;
    };
    s.set(name, &Restored::Surface(surface.clone()));
    let dump = dump_surface(name, &surface);
    log::result(&dump);
    if !dump.contains("Plane") {
        log::puts("Error: surface is not a plane");
    }
    text
}

/// 0033170: plane detection with tolerances 0.006 and 1 finds the gap 0.00514953...
/// (the second check reads `$log1` again, as the script does).
pub fn bug33170(s: &mut dyn Session) {
    log::puts("============");
    log::puts(
        "0033170: Modeling Algorithms - Checking for canonical geometry: plane detection problems",
    );
    log::puts("============");
    log::puts("");
    let expect_gap = 0.0051495320504590563;
    let f = set(s, "f", brestore("bug33170.brep"));
    let log1 = plane_of(s, "asurf1", &f, 0.006);
    let gap1 = numbers_after(&log1, "Gap = ", 1, "").expect("regexp {Gap = +(...)}")[0];
    checkreal("FoundGap1", gap1, expect_gap, 1.0e-9, 0.0);
    plane_of(s, "asurf2", &f, 1.);
    let gap2 = numbers_after(&log1, "Gap = ", 1, "").expect("regexp {Gap = +(...)}")[0];
    checkreal("FoundGap2", gap2, expect_gap, 1.0e-9, 0.0);
}
