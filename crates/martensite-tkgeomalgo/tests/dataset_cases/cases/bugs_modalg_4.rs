//! `bugs_modalg_4` case bodies.
use crate::cases::script::banner;
use crate::support::commands::intersect::intersect;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug23076(d: &mut Script) -> ScriptStep {
    banner(d, "========================", "OCC23076");
    restore(d, "bug23076_c1.draw", "c")?;
    restore(d, "bug23076_s2.draw", "s")?;
    intersect(d, "i", "c", "s")?;
    // `regexp -all {i_[-0-9.+eE]+} [directory]`: the numbered `i_` variables.
    let numbered = d
        .directory("i_")
        .iter()
        .filter(|name| name[2..].starts_with(|c: char| c.is_ascii_digit()))
        .count();
    if numbered != 26 {
        d.puts("Faulty OCC23076");
    } else {
        d.puts("OK OCC23076");
    }
    Ok(())
}
