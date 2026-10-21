//! `bugs_moddata_3` case bodies.
use crate::cases::script::{banner, captured};
use crate::support::commands::checks::{checklength, checkreal};
use crate::support::commands::extrema::{extrema, proj};
use crate::support::commands::topology::{explode, mkcurve, mksurface};
use crate::support::commands::variables::{dump, isdraw};
use crate::support::commands::views::{checkview_screenshot, fit, smallview};
use crate::support::numbers::script_real;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug23706(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC23706");
    restore(d, "bug23706_c.draw", "c")?;
    proj(
        d,
        "c",
        [3.9999965207720098, 5.0000000629157348, 5.0002142991819367],
    )?;
    if isdraw(d, "ext_1")? {
        dump(d, &["ext_1"])?;
        d.puts("OCC23706 OK");
    } else {
        d.puts("Faulty OCC23706");
    }
    Ok(())
}

pub fn bug25810(d: &mut Script) -> ScriptStep {
    banner(d, "============", "OCC25810");
    restore(d, "bug25810_e1.brep", "b1")?;
    restore(d, "bug25810_f1.brep", "b2")?;
    mkcurve(d, "c1", "b1")?;
    mksurface(d, "s2", "b2")?;
    extrema(d, "c1", "s2")?;
    Ok(())
}

pub fn bug27059(d: &mut Script) -> ScriptStep {
    banner(d, "================", "0027059");
    restore(d, "bug27059.brep", "aC")?;
    explode(d, "aC")?;
    mkcurve(d, "curve", "aC_2")?;
    for (case, x) in [(1, 12.699999), (2, 12.700001)] {
        let info = proj(d, "curve", [x, 16.8949999999593, 0.534684851975074])?;
        if !info.contains("ext_1") {
            d.puts(&format!("Error: No extrema found in case {case}"));
        }
        checklength(d, "ext_1", "1.0e-6", None)?;
    }
    let info = proj(d, "curve", [12.700000, 16.8949999999593, 0.534684851975074])?;
    if !info.contains("ext_1") {
        d.puts("Error: No extrema found in case 3");
    }
    // regexp {Point : ([-0-9.+eE]+), ([-0-9.+eE]+), ([-0-9.+eE]+)} [dump ext_1]
    let case3_info = dump(d, &["ext_1"])?;
    let point = case3_info.split("Point : ").nth(1).unwrap_or("");
    let mut coordinates = point.lines().next().unwrap_or("").split(", ");
    let x = captured(coordinates.next(), "x")?;
    let y = captured(coordinates.next(), "y")?;
    let z = captured(coordinates.next(), "z")?;
    let expected = [12.700000, 16.8949999999593, 0.534684851975074];
    for ((axis, value), expected) in ["X", "Y", "Z"].into_iter().zip([x, y, z]).zip(expected) {
        checkreal(
            d,
            &format!("case 3 coord {axis}"),
            script_real(value),
            expected,
            1.0e-6,
            0.001,
        );
    }
    Ok(())
}

pub fn bug32058(d: &mut Script) -> ScriptStep {
    banner(
        d,
        "=========",
        "0032058: Modeling Data - Extrema curve-surface gives wrong result for planar surface of revolunion and circle",
    );
    restore(d, "bug32058_c", "c")?;
    restore(d, "bug32058_s", "s")?;
    extrema(d, "c", "s")?;
    checklength(d, "ext_1", "2.3437142008433856e-13", None)?;
    smallview(d, "AXON")?;
    fit(d)?;
    checkview_screenshot(d, "bug32058")
}
