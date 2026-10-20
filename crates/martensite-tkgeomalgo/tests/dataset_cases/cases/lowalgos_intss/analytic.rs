//! `lowalgos_intss` cases on surfaces the
//! scripts build: two cylinders and two trimmed planes.
use crate::cases::script::banner;
use crate::cases::script_words::words;
use crate::support::commands::creation::cylinder;
use crate::support::commands::geometry::{plane, trim_surface};
use crate::support::commands::intersect::{IntersectOptions, intersect_with};
use crate::support::commands::variables::whatis;
use crate::support::commands::views::{checkview_screenshot, fit, smallview};
use crate::support::script::{Script, ScriptError, ScriptStep};

pub fn bug21750(d: &mut Script) -> ScriptStep {
    banner(d, "============", "OCC21750");
    let c1 = [
        1031.3339148728076,
        -113.25868616662650,
        56.152387686082520,
        -0.86602540378443815,
        0.50000000000000089,
        0.0,
        -0.50000000000000089,
        -0.86602540378443815,
        0.0,
        76.499999999999986,
    ];
    let c2 = [
        1017.0706583606553,
        -103.24955863669388,
        56.152387686082548,
        0.86602540378443915,
        -0.49999999999999933,
        0.0,
        0.49999999999999933,
        0.86602540378443915,
        0.0,
        76.499999999999986,
    ];
    cylinder(d, "c1", c1)?;
    cylinder(d, "c2", c2)?;
    let options = IntersectOptions {
        start: None,
        tolerance: Some(1e-7),
    };
    let info = match intersect_with(d, "r", ["c1", "c2"], options) {
        Ok(info) => info,
        Err(_) => {
            d.puts("Error: The intersection of two cylindrical surfaces causes an exception");
            // `info` stays unset, and the next line reads it.
            return Err(ScriptError("can't read \"info\": no such variable".into()));
        }
    };
    if words(&info).len() != 2 {
        // The script's `Error: Incorrect number of solutions.` is no command.
        return Err(ScriptError("invalid command name \"Error:\"".into()));
    }
    smallview(d, "AXON")?;
    fit(d)?;
    checkview_screenshot(d, "bug21750")
}

pub fn bug27623(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC27623 ALL: ERROR: OCC27623 is reproduced.");
    banner(d, "========", "OCC27623");
    plane(d, "s1", &[0., 0., 0., 0., 0., 1., 1., 0., 0.])?;
    trim_surface(d, "s1", "s1", [-10., 10., -10., 10.])?;
    plane(d, "s2", &[0., 0., 10., 1., 0., 0., 0., 0., 1.])?;
    // `-10+1.0e-5`: the real parser evaluates the sum.
    trim_surface(d, "s2", "s2", [-10. + 1.0e-5, 10., -10. + 1.0e-5, 10.])?;
    let options = IntersectOptions {
        start: None,
        tolerance: Some(4.0e-4),
    };
    intersect_with(d, "ii", ["s1", "s2"], options)?;
    // `string length [string trim [whatis ii]]` is 7 for `ii is a` (no variable `ii`).
    if whatis(d, "ii")?.trim().chars().count() == 7 {
        d.puts("ERROR: OCC27623 is reproduced. Intersection of trimmed planar surface has not been detected.");
    }
    Ok(())
}
