//! `bugs_moddata_2` case bodies.
use crate::cases::lowalgos_intss::sampled_curve;
use crate::cases::script::banner;
use crate::cases::script_words::words;
use crate::support::commands::checks::checksection;
use crate::support::commands::extrema::{extrema, proj};
use crate::support::commands::geometry::untrim;
use crate::support::commands::intersect::intersect;
use crate::support::commands::variables::{copy, whatis};
use crate::support::commands::views::{
    checkview_display, checkview_screenshot, don, fit, smallview,
};
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug231(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC231");
    restore(d, "OCC231.draw", "result")?;
    let found = proj(d, "result", [1751.38, 614.152, -180.331])?;
    if words(&found).len() < 4 {
        d.puts("Faulty OCC231: Projection was made wrongly");
    } else {
        d.puts("OCC231 OK: Projection was made properly");
    }
    checkview_display(d, "result", "bug231")
}

pub fn bug235(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC235");
    restore(d, "OCC235a.rle", "su1")?;
    restore(d, "OCC235b.rle", "su2")?;
    intersect(d, "result", "su1", "su2")?;
    // `regexp -all {result} [directory]`: the variables whose name holds `result`.
    if d.directory("")
        .iter()
        .filter(|name| name.contains("result"))
        .count()
        != 2
    {
        d.puts("Faulty OCC235 (first checking): Intersection was made WRONGLY");
    } else {
        d.puts("OCC235 OK (first checking) : Intersection command works properly");
        if checksection(d, "result_1")? != 0 {
            d.puts(" Faulty OCC235 (second checking): Result result_1 is UNclosed !!! ");
        } else {
            d.puts(" OCC235 OK (second checking): Result result_1 is closed !!! ");
        }
        if checksection(d, "result_2")? != 0 {
            d.puts(" Faulty OCC235 (second checking): Result result_2 is UNclosed !!! ");
        } else {
            d.puts("  OCC235 OK (second checking): Result result_2 is closed !!! ");
        }
    }
    smallview(d, "AXON")?;
    don(d, "result")?;
    fit(d)?;
    checkview_screenshot(d, "bug235")
}

pub fn bug430(d: &mut Script) -> ScriptStep {
    banner(d, "================", " OCC430 ");
    restore(d, "OCC430.rle", "result")?;
    if proj(d, "result", [49.86391363636364, 440.7894090909091, 127.05]).is_err() {
        d.puts("Faulty OCC430: Projection was made wrongly.");
    } else {
        d.puts("OK OCC348: function PROJECT works properly");
    }
    checkview_display(d, "result", "bug430")
}

pub fn bug444(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC444");
    restore(d, "OCC444a.draw", "a")?;
    restore(d, "OCC444a.draw", "b")?;
    extrema(d, "a", "b")?;
    Ok(())
}

pub fn bug541(d: &mut Script) -> ScriptStep {
    banner(d, "================", " OCC541 ");
    restore(d, "OCC541a.draw", "c1x")?;
    restore(d, "OCC541b.draw", "c2x")?;
    let result = extrema(d, "c1x", "c2x")?;
    if words(&result).is_empty() {
        d.puts("Faulty OCC541 (amount of solution): command extrema does NOT work properly");
    } else {
        d.puts("OCC541 OK (amount of solution): : command extrema works properly");
    }
    // The script variable `result` holds the command text; no case variable has that name.
    checkview_display(d, "result", "bug541")
}

pub fn bug569(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC569");
    restore(d, "OCC569a.draw", "s1")?;
    restore(d, "OCC569b.draw", "s2")?;
    smallview(d, "AXON")?;
    crate::support::commands::views::donly(d, &["s1", "s2"])?;
    fit(d)?;
    if intersect(d, "result", "s1", "s2").is_err() {
        d.puts("Faulty OCC569 exception: function intersection works wrongly with trimmed plane and cone surfaces ");
        return Err(crate::support::script::ScriptError(
            "can't read \"ic\": no such variable".into(),
        ));
    }
    checkview_screenshot(d, "bug569")?;
    if whatis(d, "result")?.contains("3d curve") {
        copy(d, "result", "result_1")?;
    }
    untrim(d, "s1", "s1")?;
    untrim(d, "s2", "s2")?;
    let mut count = 0;
    loop {
        let curve = format!("result_{}", count + 1);
        if !whatis(d, &curve)?.contains("3d curve") {
            break;
        }
        sampled_curve(d, &curve, ["s1", "s2"], 1e-7, true)?;
        count += 1;
    }
    if count == 1 {
        d.puts("OCC569 OK: function intersection works with trimmed plane and cone surfaces");
    } else {
        d.puts("Faulty OCC569: function intersection works wrongly with trimmed plane and cone surfaces");
    }
    Ok(())
}

pub fn bug766(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC766");
    restore(d, "OCC766.draw", "c")?;
    let result = proj(d, "c", [30., 3.944304526105059e-31, 20.0])?;
    if !words(&result).is_empty() {
        d.puts("OCC770 OK");
    } else {
        d.puts("Faulty OCC770");
    }
    checkview_display(d, "result", "bug766")
}
