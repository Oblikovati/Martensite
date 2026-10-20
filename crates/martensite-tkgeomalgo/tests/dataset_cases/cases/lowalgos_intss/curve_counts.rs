//! `lowalgos_intss` cases that check the
//! curves `intersect` publishes: their number, kind and closure.
use crate::cases::script::banner;
use crate::cases::script_words::{last_word, second_last_word, words};
use crate::support::commands::checks::checksection;
use crate::support::commands::geometry::trim_one_direction;
use crate::support::commands::intersect::intersect;
use crate::support::commands::topology::mksurface;
use crate::support::commands::variables::{dump, whatis};
use crate::support::commands::views::{
    checkview_display, checkview_screenshot, display, donly, fit, smallview,
};
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

/// The `repeat 10` loop of `bug106`: the leading `prefix_j` variables that are 3D curves.
fn counted_curves(d: &mut Script, prefix: &str) -> ScriptStep<usize> {
    let mut count = 0;
    for j in 1..=10 {
        let che = whatis(d, &format!("{prefix}_{j}"))?;
        let che1 = whatis(d, &format!("{prefix}_{j}"))?;
        if last_word(&che) != "curve" && second_last_word(&che1) != "3d" {
            break;
        }
        count += 1;
    }
    Ok(count)
}

fn occ106_check(d: &mut Script, step: &str, prefix: &str, surfaces: [&str; 2]) -> ScriptStep {
    if intersect(d, prefix, surfaces[0], surfaces[1]).is_err() {
        d.puts(&format!(
            "{step}) Faulty OCC106: Intersection was made WRONGLY"
        ));
        return Ok(());
    }
    d.puts("");
    let nom = counted_curves(d, prefix)?;
    if nom == 0 {
        d.puts(&format!(
            "{step}) Faulty OCC106: Intersection was made WRONGLY"
        ));
    } else {
        d.puts(&format!(
            "{step}) OCC106 OK : Intersection command works properly: {nom} curves"
        ));
        if step == "1" {
            d.puts("");
        }
    }
    Ok(())
}

pub fn bug106(d: &mut Script) -> ScriptStep {
    banner(d, "================", "BUC61032");
    d.puts("OCC106");
    restore(d, "OCC106-1.draw", "sh1")?;
    restore(d, "OCC106-2.draw", "sh2")?;
    trim_one_direction(d, false, "s1", "sh1", (-1000., 1000.))?;
    trim_one_direction(d, false, "s2", "sh2", (-1000., 1000.))?;
    occ106_check(d, "1", "res", ["s1", "s2"])?;
    occ106_check(d, "2", "rrs", ["sh1", "sh2"])
}

pub fn bug139(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC139");
    restore(d, "OCC139-1.draw", "sh1")?;
    restore(d, "OCC139-2.draw", "sh2")?;
    if intersect(d, "result", "sh1", "sh2").is_ok() {
        d.puts("OCC139 OK : intersection found");
        if checksection(d, "result")? != 0 {
            d.puts("Faulty OCC139: Intersection was made WRONGLY. The curve must be closed .");
        } else {
            d.puts("OCC139 OK : Intersection command works properly. The curve is closed");
        }
    } else {
        d.puts("Faulty OCC139 : NO intersection found");
    }
    checkview_display(d, "result", "bug139")
}

pub fn bug20524(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC20524");
    restore(d, "OCC20524-surf1.draw", "surf1")?;
    restore(d, "OCC20524-surf2.draw", "surf2")?;
    intersect(d, "result", "surf1", "surf2")?;
    if !whatis(d, "result")?.contains("3d curve") {
        d.puts("Faulty OCC20524: command INTERSECT works wrongly");
    } else {
        d.puts("OCC20524 OK : command INTERSECT works properly");
    }
    Ok(())
}

pub fn bug22766(d: &mut Script) -> ScriptStep {
    banner(d, "================", "CR22766");
    restore(d, "bug22766_f1", "f1")?;
    restore(d, "bug22766_f2", "f2")?;
    mksurface(d, "s1", "f1")?;
    mksurface(d, "s2", "f2")?;
    intersect(d, "result", "s1", "s2")?;
    Ok(())
}

/// The tori intersect in b-splines only, at most 6 `result*` curves.
fn tori(d: &mut Script, names: [&str; 2], image: &str) -> ScriptStep {
    d.puts(
        "TODO OCC23163 ALL: Error: No resulting circle at the intersection of tori, only b-splines",
    );
    banner(d, "================", "OCC23163");
    restore(d, "bug23163_surface1_draw_b.draw", names[0])?;
    restore(d, "bug23163_surface2_draw_b.draw", names[1])?;
    intersect(d, "result", names[0], names[1])?;
    if d.directory("result").len() > 6 {
        d.puts("OK: Number of curves is good");
    } else {
        d.puts("Error: No resulting circle at the intersection of tori, only b-splines");
    }
    smallview(d, "AXON")?;
    donly(d, &["result*"])?;
    fit(d)?;
    display(d, &names)?;
    checkview_screenshot(d, image)
}

pub fn bug23163_2(d: &mut Script) -> ScriptStep {
    tori(d, ["s1", "s2"], "bug23163_2")
}

pub fn bug23163_3(d: &mut Script) -> ScriptStep {
    tori(d, ["ss1", "ss2"], "bug23163_3")
}

pub fn bug23166(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC23166");
    restore(d, "bug23166_surface1_draw.draw", "s1")?;
    restore(d, "bug23166_surface2_draw.draw", "s2")?;
    intersect(d, "result", "s1", "s2")?;
    for (curve, which) in [("result_1", "first"), ("result_2", "second")] {
        if dump(d, &[curve])?.contains("Parameters") {
            d.puts(&format!("Error: untrimmed {which} hyperbola"));
        } else {
            d.puts(&format!("OK: untrimmed {which} hyperbola"));
        }
    }
    smallview(d, "AXON")?;
    donly(d, &["result*"])?;
    fit(d)?;
    display(d, &["s1", "s2"])?;
    checkview_screenshot(d, "bug23166")
}

pub fn bug237(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC237");
    restore(d, "OCC236b.rle", "su1")?;
    restore(d, "OCC237b.rle", "su2")?;
    if intersect(d, "result", "su1", "su2").is_err() {
        d.puts("Faulty OCC237: Intersection was made WRONGLY");
    } else {
        d.puts("OCC237 OK  : Intersection command works properly");
    }
    checkview_display(d, "result", "bug237")
}

pub fn bug23732(d: &mut Script) -> ScriptStep {
    banner(d, "================", "CR23722");
    restore(d, "bug23732_fx1.brep", "b1")?;
    restore(d, "bug23732_fx2.brep", "b2")?;
    mksurface(d, "s1", "b1")?;
    mksurface(d, "s2", "b2")?;
    intersect(d, "result", "s1", "s2")?;
    checkview_display(d, "result", "bug23732")
}

pub fn bug25898(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC25898");
    restore(d, "bug25898_x001_1.draw", "s1")?;
    restore(d, "bug25898_x001_2.draw", "s2")?;
    let curve_numb = intersect(d, "i", "s1", "s2")?;
    if words(&curve_numb).len() != 2 {
        d.puts("Error : Wrong result obtained by intersection algorithm.");
    } else {
        d.puts("OK : Good result obtained by intersection algorithm.");
    }
    smallview(d, "AXON")?;
    fit(d)?;
    checkview_screenshot(d, "bug25898")
}

/// `bug271` and `bug431`: `intersect`, then `whatis result` must name a 3D curve.
fn named_curve_result(d: &mut Script, bug: &str, files: [&str; 2], image: &str) -> ScriptStep {
    restore(d, files[0], "su1")?;
    restore(d, files[1], "su2")?;
    intersect(d, "result", "su1", "su2")?;
    let che = whatis(d, "result")?;
    if !che.contains("curve") || !che.contains("3d") {
        d.puts(&format!("Faulty {bug} : Command INTERSECT works WRONGLY"));
    } else {
        d.puts(&format!("{bug} OK  :  Command INTERSECT works properly"));
    }
    checkview_display(d, "result", image)
}

pub fn bug431(d: &mut Script) -> ScriptStep {
    banner(d, "================", " OCC431 ");
    named_curve_result(d, "OCC431", ["OCC431a.draw", "OCC431b.draw"], "bug431")
}

pub fn bug504(d: &mut Script) -> ScriptStep {
    banner(d, "================", " OCC504 ");
    restore(d, "OCC504a.draw", "su1")?;
    restore(d, "OCC504b.draw", "su2")?;
    if intersect(d, "result", "su1", "su2").is_err() {
        d.puts("Faulty OCC504 : A solution takes all memory and crashes with AllocMemory");
    } else {
        d.puts("OCC504 OK: function intersection works ok");
    }
    let first = whatis(d, "result_1")?;
    if first.contains("curve") && whatis(d, "result_2")?.contains("curve") {
        d.puts("OCC504 OK : Result shape is CORRECT !!! ");
    } else {
        d.puts(" Faulty OCC504: Result shape is INcorrect !!! ");
    }
    checkview_display(d, "result", "bug504")
}

/// `bug324` and `bug513`: an `intersect` that must not fail, then the result view.
fn caught_intersection(
    d: &mut Script,
    files: [(&str, &str); 2],
    faulty: &str,
    ok: &str,
    image: &str,
) -> ScriptStep {
    restore(d, files[0].0, files[0].1)?;
    restore(d, files[1].0, files[1].1)?;
    if intersect(d, "result", files[0].1, files[1].1).is_err() {
        d.puts(faulty);
    } else {
        d.puts(ok);
    }
    checkview_display(d, "result", image)
}

pub fn bug324(d: &mut Script) -> ScriptStep {
    banner(d, "================", " OCC324 ");
    let files = [("OCC324a.draw", "s1"), ("OCC324b.draw", "s2")];
    caught_intersection(
        d,
        files,
        "Faulty OCC324: function INTERSECT works wrongly",
        "OCC324 OK: function INTERSECT works properly",
        "bug324",
    )
}

pub fn bug513(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC513");
    let files = [("OCC513a.draw", "sa23"), ("OCC513b.draw", "sb8")];
    caught_intersection(
        d,
        files,
        "Faulty OCC513: function INTERSECTION works wrongly",
        "OCC513 OK: function INTERSECTION works properly",
        "bug513",
    )
}

pub fn bug531(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC531");
    restore(d, "OCC531_1.draw", "a")?;
    restore(d, "OCC531_2.draw", "b")?;
    intersect(d, "result", "a", "b")?;
    checkview_display(d, "result", "bug531")
}

pub fn bug537(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC12345 ALL: Error : OCC537");
    banner(d, "================", "OCC537");
    restore(d, "OCC537_CONE.draw", "c")?;
    restore(d, "OCC537_PLANE.draw", "p")?;
    intersect(d, "result", "c", "p")?;
    if whatis(d, "result")?.contains("WIRE") {
        d.puts(" OCC537 OK");
    } else {
        d.puts("Error : OCC537");
    }
    checkview_display(d, "result", "bug537")
}

pub fn bug6474(d: &mut Script) -> ScriptStep {
    banner(d, "================", "OCC6474");
    restore(d, "OCC6474-s1.draw", "s1")?;
    restore(d, "OCC6474-s2.draw", "s2")?;
    intersect(d, "res", "s1", "s2")?;
    if d.directory("res").len() != 1 {
        d.puts("Faulty OCC6474: more than 1 intersection line is got");
    } else {
        d.puts("OK OCC6474");
    }
    Ok(())
}
