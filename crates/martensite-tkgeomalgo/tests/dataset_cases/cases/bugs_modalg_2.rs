//! `bugs_modalg_2` case bodies.
use crate::cases::script::banner;
use crate::support::commands::extrema::extrema;
use crate::support::commands::geometry::trim_curve;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug23114(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC23114");
    restore(d, "OCC23114-c1.draw", "c1")?;
    restore(d, "OCC23114-c2.draw", "c2")?;
    trim_curve(
        d,
        "c1t",
        "c1",
        (6.8444417666253693e-010, 3.3201579728233819e-007),
    )?;
    trim_curve(d, "c2t", "c2", (0.57529999999422077, 0.99999999996162448))?;
    if extrema(d, "c1t", "c2t").is_ok() {
        d.puts("OK OCC23114");
    } else {
        d.puts("Faulty OCC23114");
    }
    Ok(())
}
