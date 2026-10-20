//! `lowalgos_extcc` case bodies ("Regression in GeomAPI_ExtremaCurveCurve").
use crate::cases::script::{banner, captured};
use crate::cases::script_words::{last_word, words};
use crate::support::commands::checks::{checklength, checkreal};
use crate::support::commands::extrema::extrema;
use crate::support::commands::geometry::length;
use crate::support::commands::topology::mkcurve;
use crate::support::numbers::script_real;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

/// `restore` of the two edges, `mkcurve c1 e1`, `mkcurve c2 e2`, `extrema c1 c2`.
fn edge_extrema(d: &mut Script, case: &str) -> ScriptStep<String> {
    banner(d, "========", "OCC29858");
    restore(d, &format!("bug29858_{case}_e1.brep"), "e1")?;
    restore(d, &format!("bug29858_{case}_e2.brep"), "e2")?;
    mkcurve(d, "c1", "e1")?;
    mkcurve(d, "c2", "e2")?;
    extrema(d, "c1", "c2")
}

pub fn bug29858_01(d: &mut Script) -> ScriptStep {
    edge_extrema(d, "01")?;
    checklength(d, "ext_1", "9.4224516867962155e-10", Some("0.01"))
}

pub fn bug29858_02(d: &mut Script) -> ScriptStep {
    let info = edge_extrema(d, "02")?;
    // regexp {Extrema 1 is point : +([-0-9.+eE]+) +([-0-9.+eE]+) +([-0-9.+eE]+)}
    let point = info.split("Extrema 1 is point : ").nth(1).unwrap_or("");
    let mut coordinates = point.split_whitespace();
    let x = captured(coordinates.next(), "x")?;
    let y = captured(coordinates.next(), "y")?;
    let z = captured(coordinates.next(), "z")?;
    let good = [0.001, 0.0073371170841145797, -0.083369169789921913];
    for ((axis, value), good) in ["x", "y", "z"].into_iter().zip([x, y, z]).zip(good) {
        checkreal(
            d,
            &format!("Intersection point {axis}:"),
            script_real(value),
            good,
            0.01,
            0.01,
        );
    }
    Ok(())
}

pub fn bug29858_03(d: &mut Script) -> ScriptStep {
    let info = edge_extrema(d, "03")?;
    if !info.contains("ext_1") {
        d.puts("Error: Extrema is not found");
        return Ok(());
    }
    let measured = length(d, "ext_1", None)?;
    if script_real(last_word(&measured)) > 1.0e-10 {
        d.puts("Error: Extrema distance is too big");
    }
    Ok(())
}

pub fn bug29858_04(d: &mut Script) -> ScriptStep {
    let info = edge_extrema(d, "04")?;
    if words(&info).len() != 2 {
        d.puts("Error: Invalid extrema number in extrema output");
    }
    checklength(d, "ext_1", "2.2204460492503131e-16", Some("0.01"))?;
    checklength(d, "ext_2", "9.693032344962995e-16", Some("0.01"))
}
