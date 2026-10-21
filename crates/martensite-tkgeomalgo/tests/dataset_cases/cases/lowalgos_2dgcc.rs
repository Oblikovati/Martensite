//! `lowalgos_2dgcc` case bodies.
use crate::cases::script::banner;
use crate::support::commands::checks::checklength;
use crate::support::commands::construction::cirtang_curves;
use crate::support::commands::geometry::to2d;
use crate::support::commands::topology::{explode, mkcurve};
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug23866(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC23866");
    restore(d, "bug23866_3curves.brep", "3curves")?;
    explode(d, "3curves")?;
    for i in 1..=3 {
        mkcurve(d, &format!("3d_curve_{i}"), &format!("3curves_{i}"))?;
    }
    for i in 1..=3 {
        to2d(d, &format!("2d_curve_{i}"), &format!("3d_curve_{i}"))?;
    }
    for name in d.directory("cir") {
        d.unset(&name);
    }
    cirtang_curves(d, "cir", ["2d_curve_1", "2d_curve_2", "2d_curve_3"])?;
    if d.directory("cir").len() != 8 {
        d.puts("Error: GccAna_Circ2d3Tan does not find a circle");
        return Ok(());
    }
    d.puts("OK: GccAna_Circ2d3Tan find a circle");
    let lengths = [
        "9429.8727947293246",
        "888.65915068757909",
        "8.5227983514817236",
        "6.2260879234386",
        "14054.240322333564",
        "596.25725451268329",
        "7.6941358383928531",
        "6.8966408969867814",
    ];
    for (index, length) in lengths.into_iter().enumerate() {
        checklength(d, &format!("cir_{}", index + 1), length, None)?;
    }
    Ok(())
}
