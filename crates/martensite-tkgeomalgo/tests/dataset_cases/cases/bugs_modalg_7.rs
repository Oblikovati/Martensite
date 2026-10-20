//! `bugs_modalg_7` case bodies.
use crate::cases::script::banner;
use crate::cases::script_words::number_after;
use crate::support::commands::checks::xdistcs;
use crate::support::commands::construction::{interpol, tuyau};
use crate::support::commands::extrema::{extrema, proj};
use crate::support::commands::geometry::{bounds, plane};
use crate::support::commands::intersect::intersect;
use crate::support::commands::qa::occ29807;
use crate::support::commands::topology::{explode, explode_type, mkcurve, mksurface};
use crate::support::commands::variables::{isdraw, logged_dump, point};
use crate::support::commands::views::{
    checkview_display, checkview_screenshot, display, donly, erase, fit, smallview,
};
use crate::support::numbers::script_real;
use crate::support::restore::{restore, save_and_restore};
use crate::support::script::{Script, ScriptError, ScriptStep};
use martensite_tkg3d::top_abs::ShapeEnum;

/// `regexp {Accuracy of approximation = ([0-9+-.eE]*)} $log full accuracy` and the
/// 1e-4 check.
fn check_pipe_accuracy(d: &mut Script, log: &str) -> ScriptStep {
    let accuracy = number_after(log, "Accuracy of approximation =")
        .ok_or_else(|| ScriptError("can't read \"accuracy\": no such variable".into()))?;
    if script_real(accuracy) > 0.0001 {
        d.puts("Error: bad accuracy of approximation");
    }
    Ok(())
}

pub fn bug21413(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC21413 ALL: bad accuracy of approximation");
    banner(d, "========", "OCC21413");
    restore(d, "bug21413_dc1_err.draw", "a")?;
    let log = tuyau(d, "result", "a", "1")?;
    check_pipe_accuracy(d, &log)?;
    checkview_display(d, "result", "bug21413")
}

/// The four counts the script reads from the dump of a pipe surface.
struct SurfaceCounts {
    degree: String,
    poles: String,
    u_knots: String,
    v_knots: String,
}

/// `regexp {Degrees :+(a) +(b)}`-style captures of the second (or both) numbers.
fn counts(d: &mut Script, info: &str, suffix: u8) -> ScriptStep<SurfaceCounts> {
    let pair = |key: &str, variable: &str| -> ScriptStep<(String, String)> {
        let unset = || {
            ScriptError(format!(
                "can't read \"{variable}_{suffix}\": no such variable"
            ))
        };
        let start = info.find(&format!("{key} :")).ok_or_else(unset)? + key.len() + 1;
        let rest = info[start..].trim_start_matches(':');
        let mut numbers = rest.split(' ').filter(|w| !w.is_empty());
        let first = numbers.next().ok_or_else(unset)?.trim().to_string();
        let second = numbers
            .next()
            .ok_or_else(unset)?
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();
        Ok((first, second))
    };
    let (_, degree) = pair("Degrees", "Degrees")?;
    let (_, poles) = pair("NbPoles", "NbPoles")?;
    let (u_knots, v_knots) = pair("NbKnots", "UKnots")?;
    for (name, value) in [
        ("Degrees", &degree),
        ("NbPoles", &poles),
        ("UKnots", &u_knots),
        ("VKnots", &v_knots),
    ] {
        d.puts(&format!("{name}_{suffix} = {value}"));
    }
    Ok(SurfaceCounts {
        degree,
        poles,
        u_knots,
        v_knots,
    })
}

/// One pipe of `bug23942`: `tuyau`, the accuracy check and the counts of its dump.
fn pipe_counts(d: &mut Script, result: &str, curve: &str, suffix: u8) -> ScriptStep<SurfaceCounts> {
    let log = tuyau(d, result, curve, "5")?;
    check_pipe_accuracy(d, &log)?;
    let info = logged_dump(d, result)?;
    counts(d, &info, suffix)
}

pub fn bug23942(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC21413 ALL: bad accuracy of approximation");
    banner(d, "========", "OCC23942");
    interpol(d, "c", "bug23942_points.txt")?;
    let first = pipe_counts(d, "r_1", "c", 1)?;
    save_and_restore(d, "c", "cc")?;
    let second = pipe_counts(d, "r_2", "cc", 2)?;
    let pairs = [
        ("Degrees", &first.degree, &second.degree),
        ("NbPoles", &first.poles, &second.poles),
        ("UKnots", &first.u_knots, &second.u_knots),
        ("VKnots", &first.v_knots, &second.v_knots),
    ];
    for (name, a, b) in pairs {
        if script_real(a) != script_real(b) {
            d.puts(&format!("Error : {name}_1 is not equal {name}_2"));
        } else {
            d.puts(&format!("OK : {name}_1 is equal {name}_2"));
        }
    }
    checkview_display(d, "r_1", "bug23942_1")?;
    checkview_display(d, "r_2", "bug23942_2")
}

pub fn bug28279(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC28279");
    restore(d, "bug28279_surface1_draw.draw", "s1")?;
    restore(d, "bug28279_surface2_draw.draw", "s2")?;
    intersect(d, "result", "s1", "s2")?;
    smallview(d, "AXON")?;
    // `point p121 -2.20000000000914, ...`: the real parser stops at each trailing comma.
    point(
        d,
        "p121",
        [-2.20000000000914, 4.42587047983305, 2.00000000001213],
    )?;
    point(
        d,
        "p11",
        [-2.20245354854755, 4.52573922818428, 1.82990686981508],
    )?;
    donly(d, &["p121", "p11"])?;
    fit(d)?;
    erase(d, &["p121", "p11"])?;
    display(d, &["result_1", "result_2", "s1", "s2"])?;
    checkview_screenshot(d, "bug28279")
}

pub fn bug28544_1(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC28544 ALL: Error in ii: T=");
    banner(d, "========", "OCC28544");
    restore(d, "bug24094_face.brep", "b1")?;
    mksurface(d, "s1", "b1")?;
    plane(d, "s2", &[183.6, 0., 0., 1., 0., 0., 0., 1., 0.])?;
    intersect(d, "ii", "s1", "s2")?;
    let range = bounds(d, "ii", ["u1", "u2"])?;
    xdistcs(d, "ii", "s1", range, 10, 2.0e-7)?;
    Ok(())
}

/// `regexp {Radius of curvature is +(r)} [OCC29807 s1 s2 u1 v1 u2 v2] full R`.
fn curvature_radius(d: &mut Script, uv: [f64; 4], variable: &str) -> ScriptStep<f64> {
    let text = occ29807(d, ["s1", "s2"], uv)?;
    let radius = number_after(&text, "Radius of curvature is")
        .ok_or_else(|| ScriptError(format!("can't read \"{variable}\": no such variable")))?;
    Ok(script_real(radius))
}

pub fn bug29807_sc01(d: &mut Script) -> ScriptStep {
    banner(d, "========", "0029807: Impossible to cut cone from prism");
    restore(d, "OCC13116_sh1.brep", "b1")?;
    restore(d, "OCC13116_sh2.brep", "b2")?;
    explode_type(d, "b1", ShapeEnum::Face)?;
    explode_type(d, "b2", ShapeEnum::Face)?;
    mksurface(d, "s1", "b1_3")?;
    mksurface(d, "s2", "b2_1")?;
    let points = [
        [
            1.5704836137868956,
            3.0501269066007808e-007,
            3.9658833912538207,
            1.5704836137865030,
        ],
        [
            1.6025602743734420,
            -6.1366790760075673e-007,
            0.82433854035089271,
            1.5390323792163476,
        ],
        [
            1.6375006167098363,
            -8.0934069046634249e-007,
            0.82433889938148752,
            1.5040920368799497,
        ],
        [
            1.7042049066248015,
            -1.1250135685259011e-006,
            0.82433908359918306,
            1.4373877469650012,
        ],
    ];
    for (index, uv) in points.into_iter().enumerate() {
        let step = index + 1;
        if curvature_radius(d, uv, &format!("R{step}"))? != -1.0 {
            d.puts(&format!("Error in R{step} computation"));
        }
    }
    let uv5 = [
        1.7772619860554566,
        -1.3863691492588259e-006,
        0.82433916835888565,
        1.3643306675343436,
    ];
    if (curvature_radius(d, uv5, "R5")? - 19.71766721319873).abs() > 1.0e-7 {
        d.puts("Error in R5 computation");
    }
    Ok(())
}

/// `proj c x y z` must name `ext_1 .. ext_{count}` and not `ext_{count + 1}`.
fn projection_count(d: &mut Script, file: &str, xyz: [f64; 3], count: usize) -> ScriptStep {
    restore(d, file, "c")?;
    let info = proj(d, "c", xyz)?;
    if info.contains(&format!("ext_{}", count + 1)) {
        d.puts("Error : Projection is not correct");
    }
    if (1..=count).all(|i| info.contains(&format!("ext_{i}"))) {
        d.puts("OK: Projection is correct");
    } else {
        d.puts("Error : Projection is not correct");
    }
    Ok(())
}

pub fn bug29857(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC29857");
    projection_count(
        d,
        "bug29857.brep",
        [-53.9663741221239, 118.723988602907, -57.6228206908223],
        1,
    )
}

pub fn bug29857_1(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC29857_1");
    projection_count(
        d,
        "bug29857_1.brep",
        [-139.04514104219, 128.256108703487, -64.9999999999983],
        2,
    )
}

pub fn bug30722(d: &mut Script) -> ScriptStep {
    d.puts("=================================================================");
    d.puts("OCC30722: Modeling Algorithms - BRepExtrema_DistShapeShape computes wrong distances");
    d.puts("=================================================================");
    d.puts("");
    restore(d, "bug30722_1.brep", "w")?;
    restore(d, "bug30722_2.brep", "e")?;
    explode(d, "w")?;
    mkcurve(d, "c1", "w_1")?;
    mkcurve(d, "c2", "e")?;
    extrema(d, "c1", "c2")?;
    if !isdraw(d, "ext_1")? {
        d.puts("Error: no solutions!");
    }
    Ok(())
}
