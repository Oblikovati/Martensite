//! `bugs_filling` case bodies.
use crate::cases::script::banner;
use crate::support::commands::checks::checkreal;
use crate::support::commands::construction::fillcurves;
use crate::support::commands::geometry::{dval, svalue};
use crate::support::commands::variables::whatis;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug16119(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC16119");
    for name in ["c1", "c2", "c31", "c32", "c41", "c42", "cv"] {
        restore(d, &format!("OCC16119-{name}.draw"), name)?;
    }
    fillcurves(d, "s11", ["c1", "c41", "cv", "c31"])?;
    fillcurves(d, "s12", ["cv", "c42", "c2", "c32"])?;
    let names1 = [
        "x1", "y1", "z1", "dux1", "duy1", "duz1", "dvx1", "dvy1", "dvz1",
    ];
    let names2 = [
        "x2", "y2", "z2", "dux2", "duy2", "duz2", "dvx2", "dvy2", "dvz2",
    ];
    svalue(d, "s11", (0.5, 0.5), names1)?;
    svalue(d, "s12", (0.5, 0.5), names2)?;
    let expected = [
        -3.5527136788005009e-015,
        -3.5527136788005009e-015,
        8.8817841970012523e-016,
    ];
    for ((axis, (a, b)), expected) in ["deltaX", "deltaY", "deltaZ"]
        .into_iter()
        .zip([("dvx1", "dvx2"), ("dvy1", "dvy2"), ("dvz1", "dvz2")])
        .zip(expected)
    {
        let delta = dval(d, &format!("{a}-{b}"), d.number(a) - d.number(b))?;
        checkreal(d, axis, delta, expected, 0., 0.001);
    }
    Ok(())
}
pub fn bug16833(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC16833");
    for name in ["c1", "c2", "c3", "c4"] {
        restore(d, &format!("OCC16833-{name}.draw"), name)?;
    }
    let mistake = if fillcurves(d, "result", ["c1", "c2", "c3", "c4"]).is_err() {
        d.puts("Faulty OCC16833: filling problem");
        true
    } else {
        d.puts("Filling OCC16833  OK");
        !whatis(d, "result")?.contains("surface")
    };
    if mistake {
        d.puts("Faulty OCC16833");
    } else {
        d.puts("OK OCC16833");
    }
    Ok(())
}
