//! tests/bugs/modalg_8 case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 owned by
//! TKShHealing whose inputs are in the public dataset.
use crate::support::{
    commands::dump_surface,
    log,
    restore::{Restored, restore_as},
    session::Session,
    swdraw::getanasurf,
};

/// 0033104: a surface close to a cylinder is recognized as a cylinder.
pub fn bug33104(s: &mut dyn Session) {
    let rule = "================================";
    log::puts(rule);
    log::puts(
        "0033104: Modeling Algorithms - Checking for canonical geometry: surface, close to a cylinder, is not recognized",
    );
    log::puts(rule);
    let f = restore_as(s, "bug33104.brep", "f").shape();
    let (found, text) = getanasurf(&f, "cyl", 0.1);
    log::result(&text);
    let Some(asurf) = found else {
        log::puts("Error: required surface is not got");
        return;
    };
    s.set("asurf", &Restored::Surface(asurf.clone()));
    let dump = dump_surface("asurf", &asurf);
    log::result(&dump);
    if !dump.contains("CylindricalSurface") {
        log::puts("Error: surface is not a cylindrical surface");
    }
}
