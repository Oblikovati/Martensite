//! `lowalgos_intss` cases that sample
//! the distance from each intersection curve to both surfaces (`xdistcs`).
use crate::support::commands::checks::xdistcs;
use crate::support::commands::geometry::{bounds, trim_one_direction, trim_surface};
use crate::support::commands::intersect::intersect;
use crate::support::commands::qa::{check_intersection_result, qa_check_bends};
use crate::support::commands::topology::{explode, mksurface};
use crate::support::commands::variables::{copy, dump, renamevar, whatis};
use crate::support::commands::views::{checkview_screenshot, display, don, fit, smallview};
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};
use std::f64::consts::PI;

pub fn bug23981(d: &mut Script) -> ScriptStep {
    d.puts("=========");
    d.puts("CR23981");
    d.puts("=========");
    d.puts("");
    restore(d, "bug23981_s1.draw", "s1")?;
    restore(d, "bug23981_s2.draw", "s2")?;
    intersect(d, "i", "s1", "s2")?;
    d.puts("First test");
    xdistcs(d, "i_1", "s1", (0., 1.), 100, 1e-7)?;
    d.puts("Second test");
    xdistcs(d, "i_2", "s1", (0., 1.), 100, 1e-7)?;
    smallview(d, "AXON")?;
    fit(d)?;
    checkview_screenshot(d, "bug23981")
}

/// `bounds c U1 U2`, the range check and `xdistcs` against both surfaces.
pub fn sampled_curve(
    d: &mut Script,
    curve: &str,
    surfaces: [&str; 2],
    tolerance: f64,
    dump_bounds: bool,
) -> ScriptStep {
    let (u1, u2) = bounds(d, curve, ["U1", "U2"])?;
    if dump_bounds {
        dump(d, &["U1", "U2"])?;
    }
    if u2 - u1 < 1.0e-9 {
        d.puts("Error: Wrong curve's range!");
    }
    xdistcs(d, curve, surfaces[0], (u1, u2), 10, tolerance)?;
    xdistcs(d, curve, surfaces[1], (u1, u2), 10, tolerance)?;
    Ok(())
}

/// `set che [whatis result]` ... `renamevar result result_1`: a single curve is numbered.
pub fn number_single_curve(d: &mut Script, name: &str) -> ScriptStep {
    if whatis(d, name)?.contains("3d curve") {
        renamevar(d, name, &format!("{name}_1"))?;
    }
    Ok(())
}

/// The trimmed or untrimmed second face, one curve without a bend.
fn bent_curve(
    d: &mut Script,
    todo: &str,
    trim: bool,
    expected_text: &str,
    image: &str,
) -> ScriptStep {
    d.puts("0025952: Wrong intersection curve");
    d.puts(todo);
    restore(d, "bug25952_shape.brep", "q")?;
    explode(d, "q")?;
    copy(d, "q_1", "b1")?;
    copy(d, "q_2", "b2")?;
    mksurface(d, "s1", "b1")?;
    mksurface(d, "s2", "b2")?;
    let second = if trim {
        trim_one_direction(d, false, "s2t", "s2", (0.24999939999991502, 1.))?;
        "s2t"
    } else {
        "s2"
    };
    intersect(d, "res", "s1", second)?;
    number_single_curve(d, "res")?;
    let mut curves = Vec::new();
    loop {
        let curve = format!("res_{}", curves.len() + 1);
        if !whatis(d, &curve)?.contains("3d curve") {
            break;
        }
        qa_check_bends(d, &curve)?;
        curves.push(curve);
    }
    if curves.len() != 1 {
        d.puts(&format!(
            "Error: 1 {expected_text} is expected but {} ones are found",
            curves.len()
        ));
    } else {
        check_intersection_result(d, ["s1", "s2"], &curves, 100, [2.0e-7, 2.0e-7])?;
    }
    smallview(d, "AXON")?;
    don(d, "res_")?;
    fit(d)?;
    checkview_screenshot(d, image)
}

pub fn bug25952_2(d: &mut Script) -> ScriptStep {
    let todo = "TODO OCC25952 ALL: Error: The curve res_1 is possible to have a bend at parameter";
    bent_curve(d, todo, true, "curves", "bug25952_2")
}

pub fn bug25952_3(d: &mut Script) -> ScriptStep {
    let todo = "TODO OCC26510 ALL: Error: 1 curve is expected but 0 ones are found";
    bent_curve(d, todo, false, "curve", "bug25952_3")
}

pub fn bug26894(d: &mut Script) -> ScriptStep {
    d.puts("========");
    d.puts("OCC26894");
    d.puts("========");
    d.puts("");
    restore(d, "bug26884-f1.brep", "f1")?;
    restore(d, "bug26884-f2.brep", "f2")?;
    mksurface(d, "ss1", "f1")?;
    mksurface(d, "ss2", "f2")?;
    if intersect(d, "result", "ss1", "ss2").is_ok() {
        d.puts("Error: intersection algorithm must return fail status. But it is not.");
    }
    number_single_curve(d, "result")?;
    let mut count = 0;
    loop {
        let curve = format!("result_{}", count + 1);
        if !whatis(d, &curve)?.contains("3d curve") {
            break;
        }
        display(d, &[&curve])?;
        sampled_curve(d, &curve, ["ss1", "ss2"], 1.0e-7, true)?;
        count += 1;
    }
    if count == 0 {
        d.puts("OK: Number of curves is good!");
    } else {
        d.puts(&format!("Error: 0 is expected but {count} is found!"));
    }
    Ok(())
}

pub fn bug28222_1(d: &mut Script) -> ScriptStep {
    d.puts("========");
    d.puts("OCC28222");
    d.puts("========");
    d.puts("");
    restore(d, "bug28222_s1_cyl_read_d2.draw", "s1")?;
    restore(d, "bug28222_s2_cyl_read_d2.draw", "s2")?;
    intersect(d, "result", "s1", "s2")?;
    for curve in d.directory("result") {
        sampled_curve(d, &curve, ["s1", "s2"], 2.0e-7, false)?;
    }
    let found = d.directory("result").len();
    if found == 4 {
        d.puts("OK: Number of curves is good!");
    } else {
        d.puts(&format!("Error: 4 is expected but {found} is found!"));
    }
    smallview(d, "AXON")?;
    don(d, "result")?;
    fit(d)?;
    display(d, &["s1", "s2"])?;
    checkview_screenshot(d, "bug28222_1")
}

pub fn bug565(d: &mut Script) -> ScriptStep {
    d.puts("OCC565: Can not intersect two trimmed conical surfaces");
    restore(d, "OCC565a.draw", "s1")?;
    restore(d, "OCC565b.draw", "s2")?;
    d.puts("Preliminary check: intersection work with infinite cones:");
    if intersect(d, "inf", "s1", "s2").is_err() {
        d.puts("Faulty OCC565: function intersection works wrongly with infinite cones");
    } else {
        let mut found = false;
        for curve in d.directory("inf") {
            if !whatis(d, &curve)?.contains("3d curve") {
                dump(d, &[&curve])?;
                d.puts("Error: Intersection result (with infinite cones) is not 3D-curve");
            } else {
                found = true;
            }
        }
        if !found {
            d.puts("Error: Empty intersection result (with infinite cones)");
        }
    }
    trim_surface(d, "s1x", "s1", [0., 2. * PI, 0., 2.8])?;
    trim_surface(d, "s2x", "s2", [0., 2. * PI, 0., 2.8])?;
    if intersect(d, "result", "s1x", "s2x").is_err() {
        d.puts("Faulty OCC565 exception: function intersection works wrongly with trimmed cones");
        return Ok(());
    }
    for curve in d.directory("result") {
        sampled_curve(d, &curve, ["s1", "s2"], 1.0e-7, false)?;
    }
    let found = d.directory("result").len();
    if found == 1 {
        d.puts("OK: Number of curves is good!");
    } else {
        d.puts(&format!("Error: 1 is expected but {found} is found!"));
    }
    smallview(d, "AXON")?;
    don(d, "result")?;
    fit(d)?;
    display(d, &["s1x", "s2x"])?;
    checkview_screenshot(d, "bug565")
}
