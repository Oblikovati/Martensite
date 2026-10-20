//! `bugs_modalg_5::bug23706_10` .. bug23706_15 at
//!: extrema between B-splines with
//! repeated poles, in 3D (`extrema`) and 2D (`2dextrema`).
use crate::cases::script::{banner, captured};
use crate::cases::script_words::{last_word, number_after, words};
use crate::support::commands::creation::{bsplinecurve, bsplinecurve2d};
use crate::support::commands::extrema::extrema;
use crate::support::commands::geometry::length;
use crate::support::commands::planar::extrema2d;
use crate::support::numbers::script_real;
use crate::support::process::cpulimit;
use crate::support::script::{Reply, Script, ScriptStep};

const R1: &[f64] = &[
    2., 5., 1., 3., 2., 1., 3., 1., 4., 1., 5., 3., 2., 5., 3., 1., 3., 7., 3., 1., 4., 8., 3., 1.,
    4., 8., 3., 1., 5., 9., 3., 1., 9., 7., 3., 1.,
];
const R2: &[f64] = &[
    2., 5., 2., 3., 2.5, 1., 3., 1., 3.5, 1., 4., 3., -1., 2., 3., 1., 1., 11., 3., 1., 3., 9., 3.,
    1., 3., 9., 3., 1., 3., 9., 3., 1., 5., 7., 3., 1., 7., 4., 3., 1.,
];
const R3: &[f64] = &[
    2., 6., 1., 3., 2., 1., 3., 1., 4., 1., 5., 1., 6., 3., 2., 5., 3., 1., 3., 7., 3., 1., 4., 8.,
    3., 1., 4., 8., 3., 1., 4., 8., 3., 1., 5., 9., 3., 1., 9., 7., 3., 1.,
];
const R4: &[f64] = &[
    2., 6., 2., 3., 2.5, 1., 3., 1., 3.5, 1., 4., 1., 4.5, 3., -1., 2., 3., 1., 1., 11., 3., 1.,
    3., 9., 3., 1., 3., 9., 3., 1., 3., 9., 3., 1., 5., 7., 3., 1., 7., 4., 3., 1.,
];
const R9: &[f64] = &[
    2., 6., 1., 3., 2., 1., 3., 1., 4., 1., 5., 1., 6., 3., 4., -3., 3., 1., 6., 8., 3., 1., 10.,
    11., 3., 1., 10., 11., 3., 1., 10., 11., 3., 1., 14., 14., 3., 1., 5., 8., 3., 1.,
];
const R10: &[f64] = &[
    2., 6., 2., 3., 2.5, 1., 3., 1., 3.5, 1., 4., 1., 4.5, 3., 5., 20., 3., 1., 8., 15., 3., 1.,
    12., 18., 3., 1., 12., 18., 3., 1., 12., 18., 3., 1., 16., 21., 3., 1., 7., 12., 3., 1.,
];
const B3: &[f64] = &[
    2., 6., 1., 3., 2., 1., 3., 1., 4., 1., 5., 1., 6., 3., 2., 5., 1., 3., 7., 1., 4., 8., 1., 4.,
    8., 1., 4., 8., 1., 5., 9., 1., 9., 7., 1.,
];
const B4: &[f64] = &[
    2., 6., 2., 3., 2.5, 1., 3., 1., 3.5, 1., 4., 1., 4.5, 3., -1., 2., 1., 1., 11., 1., 3., 9.,
    1., 3., 9., 1., 3., 9., 1., 5., 7., 1., 7., 4., 1.,
];
const B7: &[f64] = &[
    2., 5., 1., 3., 2., 1., 3., 1., 4., 1., 5., 3., 4., -3., 1., 6., 8., 1., 10., 11., 1., 10.,
    11., 1., 14., 14., 1., 5., 8., 1.,
];
const B8: &[f64] = &[
    2., 5., 2., 3., 2.5, 1., 3., 1., 3.5, 1., 4., 3., 5., 20., 1., 8., 15., 1., 12., 18., 1., 12.,
    18., 1., 16., 21., 1., 7., 12., 1.,
];
const B9: &[f64] = &[
    2., 8., 1., 2., 2., 1., 3., 1., 4., 1., 5., 1., 6., 1., 7., 1., 8., 2., 4., -3., 1., 6., 8.,
    1., 10., 11., 1., 10., 11., 1., 10., 11., 1., 14., 14., 1., 5., 8., 1.,
];
const B10: &[f64] = &[
    2., 8., 2., 2., 2.5, 1., 3., 1., 3.5, 1., 4., 1., 4.5, 1., 5., 1., 5.5, 2., 5., 20., 1., 8.,
    15., 1., 12., 18., 1., 12., 18., 1., 12., 18., 1., 16., 21., 1., 7., 12., 1.,
];

fn header(d: &mut Script) {
    banner(d, "============", "OCC23706");
}

/// `cpulimit 1000` in the script: a new process limit and its message.
fn script_cpulimit(d: &mut Script) -> ScriptStep {
    d.command(&["cpulimit", "1000"], |d| {
        cpulimit(1000);
        d.cout("CPU and elapsed time limit set to 1000 seconds\n");
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// bug23706_10/11: `ext_1` must exist and its length must not exceed `limit`.
fn first_extremum_length(d: &mut Script, curves: [(&str, &[f64]); 2], limit: f64) -> ScriptStep {
    header(d);
    bsplinecurve(d, curves[0].0, curves[0].1)?;
    bsplinecurve(d, curves[1].0, curves[1].1)?;
    let info = extrema(d, curves[0].0, curves[1].0)?;
    if !info.contains("ext_1") {
        d.puts("Error: Extrema is not found");
        return Ok(());
    }
    let measured = length(d, "ext_1", None)?;
    if script_real(last_word(&measured)) > limit {
        d.puts("Error: Extrema distance is too big");
    }
    Ok(())
}

pub fn bug23706_10(d: &mut Script) -> ScriptStep {
    first_extremum_length(d, [("r3", R3), ("r4", R4)], 5.0e-11)
}

pub fn bug23706_11(d: &mut Script) -> ScriptStep {
    first_extremum_length(d, [("r1", R1), ("r2", R2)], 1.0e-8)
}

pub fn bug23706_12(d: &mut Script) -> ScriptStep {
    header(d);
    script_cpulimit(d)?;
    bsplinecurve(d, "r9", R9)?;
    bsplinecurve(d, "r10", R10)?;
    let info = extrema(d, "r9", "r10")?;
    if words(&info).len() != 1 {
        d.puts("Error : Extrema is wrong");
    } else {
        d.puts("OK: Extrema is valid");
    }
    Ok(())
}

/// The 2D cases: `2dextrema`, then `regexp "dist 1: +(...)" $info full pp`.
fn first_planar_distance(d: &mut Script, curves: [(&str, &[f64]); 2]) -> ScriptStep<f64> {
    bsplinecurve2d(d, curves[0].0, curves[0].1)?;
    bsplinecurve2d(d, curves[1].0, curves[1].1)?;
    let info = extrema2d(d, curves[0].0, curves[1].0)?;
    Ok(script_real(captured(number_after(&info, "dist 1:"), "pp")?))
}

fn planar_status(d: &mut Script, failed: bool) {
    if failed {
        d.puts("Error : Extrema is wrong");
    } else {
        d.puts("OK: Extrema is valid");
    }
}

pub fn bug23706_13(d: &mut Script) -> ScriptStep {
    header(d);
    script_cpulimit(d)?;
    let pp = first_planar_distance(d, [("b3", B3), ("b4", B4)])?;
    let failed = pp > 1.0e-7;
    if failed {
        d.puts("Error : Extrema is wrong");
    }
    planar_status(d, failed);
    Ok(())
}

pub fn bug23706_14(d: &mut Script) -> ScriptStep {
    header(d);
    let pp = first_planar_distance(d, [("b9", B9), ("b10", B10)])?;
    let failed = (pp - 3.6710534171261284).abs() > 1.0e-7;
    if failed {
        d.puts("Error : Extrema is wrong on dist 1");
    }
    planar_status(d, failed);
    Ok(())
}

pub fn bug23706_15(d: &mut Script) -> ScriptStep {
    header(d);
    let pp = first_planar_distance(d, [("b7", B7), ("b8", B8)])?;
    // The script rejects a distance equal to the old value.
    let failed = (pp - 2.3246777409727225).abs() < 1.0e-7;
    if failed {
        d.puts("Error : Extrema is wrong");
    }
    planar_status(d, failed);
    Ok(())
}
