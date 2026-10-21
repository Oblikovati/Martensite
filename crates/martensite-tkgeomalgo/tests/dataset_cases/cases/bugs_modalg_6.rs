//! `bugs_modalg_6` case bodies.
use crate::cases::script::banner;
use crate::support::commands::extrema::extrema;
use crate::support::commands::topology::mkcurve;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug26075(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC26075");
    restore(d, "dist1-s1.brep", "s1")?;
    restore(d, "dist1-s2.brep", "s2")?;
    mkcurve(d, "c1", "s1")?;
    mkcurve(d, "c2", "s2")?;
    let info = extrema(d, "c1", "c2")?;
    // script `string range $info [string first "Infinite" $info] [expr {[string first
    // "distance" $info] - 3}]`, with -1 for a missing word.
    let first = |word: &str| info.find(word).map_or(-1, |at| at as i64);
    let (start, end) = (first("Infinite").max(0), first("distance") - 3);
    let range = if end >= start {
        &info[start as usize..=end as usize]
    } else {
        ""
    };
    if range != "Infinite number of extremas" {
        d.puts("ERROR: OCC26075 is reproduced. Flag IsParallel is not returned.");
    }
    Ok(())
}
