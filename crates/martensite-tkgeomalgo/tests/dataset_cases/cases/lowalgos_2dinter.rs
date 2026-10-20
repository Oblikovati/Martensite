//! `lowalgos_2dinter` case bodies.
use crate::cases::script::{banner, captured};
use crate::support::commands::planar::intersect2d;
use crate::support::commands::topology::{explode, explode_type, mk2dcurve, pcurve, pcurve_face};
use crate::support::numbers::script_real;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The two curves of the shape must touch (`2dintersect -state`).
fn touching(d: &mut Script, file: &str) -> ScriptStep {
    restore(d, file, "a")?;
    explode(d, "a")?;
    mk2dcurve(d, "c1", "a_1", 1)?;
    mk2dcurve(d, "c2", "a_2", 1)?;
    let out = intersect2d(d, &["c1", "c2"], Some(1.0e-9), true)?;
    if !out.contains("TOUCH") {
        d.puts("Error: intersection type is not TOUCH");
    }
    Ok(())
}

pub fn a1(d: &mut Script) -> ScriptStep {
    touching(d, "bug29368_1.brep")
}

pub fn a2(d: &mut Script) -> ScriptStep {
    touching(d, "bug29368_2.brep")
}

/// The `regexp`s of `bug25593` over one `2dintersect` text: the point and both
/// parameters (unset when absent).
fn first_point(info: &str) -> [Option<String>; 4] {
    let point = info.split("Intersection point 1 : ").nth(1).map(|rest| {
        rest.split_whitespace()
            .take(2)
            .map(String::from)
            .collect::<Vec<_>>()
    });
    let parameters = info
        .split("parameter on the fist: ")
        .nth(1)
        .and_then(|rest| {
            let (first, rest) = rest.split_once(" parameter on the second: ")?;
            Some((
                first.trim().to_string(),
                rest.split_whitespace().next()?.to_string(),
            ))
        });
    let (px, py) = match point {
        Some(p) if p.len() == 2 => (Some(p[0].clone()), Some(p[1].clone())),
        _ => (None, None),
    };
    let (f, s) = parameters.map_or((None, None), |(f, s)| (Some(f), Some(s)));
    [px, py, f, s]
}

/// `NbIP`: 1 for exactly one intersection point, with the script's error lines.
fn point_count(d: &mut Script, info: &str, step: u8) -> u8 {
    let mut count = 1;
    if !info.contains("Intersection point 1") {
        d.puts(&format!(
            "Error : Intersection {step} should have one point"
        ));
        count = 0;
    }
    if info.contains("Intersection point 2") {
        d.puts(&format!(
            "Error : Intersection {step} should have one point"
        ));
        count = 2;
    }
    d.puts(&format!("NbIP{step}={count}"));
    count
}

pub fn bug25593(d: &mut Script) -> ScriptStep {
    banner(d, "============", "OCC25593");
    restore(d, "bug25593_face1.brep", "b")?;
    pcurve_face(d, "b")?;
    d.puts("Intersection 1");
    let info1 = intersect2d(d, &["b_2", "b_3"], Some(1.0e-10), false)?;
    d.puts("Intersection 2");
    let info2 = intersect2d(d, &["b_3", "b_2"], Some(1.0e-10), false)?;
    let (first, second) = (point_count(d, &info1, 1), point_count(d, &info2, 2));
    if first != 1 || second != 1 {
        d.puts("ERROR: Wrong solutions number");
        return Ok(());
    }
    let names1 = ["p1x", "p1y", "par1f", "par1s"];
    let names2 = ["p2x", "p2y", "par2f", "par2s"];
    let found1 = first_point(&info1);
    let found2 = first_point(&info2);
    let mut values1 = Vec::new();
    let mut values2 = Vec::new();
    for i in 0..4 {
        values1.push(captured(found1[i].as_deref(), names1[i])?.to_string());
        values2.push(captured(found2[i].as_deref(), names2[i])?.to_string());
    }
    d.puts(&format!(
        "p1x={} p1y={} par1f={} {}",
        values1[0], values1[1], values1[2], values1[3]
    ));
    d.puts(&format!(
        "p2x={} p2y={} par2f={} {}",
        values2[0], values2[1], values2[2], values2[3]
    ));
    let [p1x, p1y, par1f, par1s] = [0, 1, 2, 3].map(|i| script_real(&values1[i]));
    let [p2x, p2y, par2f, par2s] = [0, 1, 2, 3].map(|i| script_real(&values2[i]));
    let sqd = (p1x - p2x) * (p1x - p2x) + (p1y - p2y) * (p1y - p2y);
    if sqd > 1.0e-14 {
        d.puts("ERROR: Intersection points are different");
    } else {
        d.puts("OK: Intersection points are equal");
    }
    if (par1f - par2s).abs() > 1.0e-9 || (par1s - par2f).abs() > 1.0e-9 {
        d.puts("ERROR: Parameters on intersecting curves are not coincided");
    } else {
        d.puts("OK: Parameters on intersecting curves are coincided");
    }
    Ok(())
}

/// `[\-0-9.]*` at the start of `text`: the leading sign, digits and points.
fn plain_number(text: &str) -> &str {
    let end = text
        .find(|c: char| !(c == '-' || c == '.' || c.is_ascii_digit()))
        .unwrap_or(text.len());
    &text[..end]
}

pub fn bug26099(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC26099");
    restore(d, "OCC26099-f.brep", "f")?;
    explode_type(d, "f", ShapeEnum::Edge)?;
    pcurve(d, "c4", "f_4", "f")?;
    pcurve(d, "c5", "f_5", "f")?;
    let info = intersect2d(d, &["c4", "c5"], None, false)?;
    // regexp {fist: ([\-0-9.]*) .*second: ([\-0-9.]*)}: the first `fist:` and, through the
    // greedy `.*`, the last `second:`.
    let matched = info.find("fist: ").and_then(|at| {
        let rest = &info[at + "fist: ".len()..];
        let par1 = plain_number(rest);
        let tail = rest[par1.len()..].strip_prefix(' ')?;
        let second = tail.rfind("second: ")?;
        Some((
            par1.to_string(),
            plain_number(&tail[second + "second: ".len()..]).to_string(),
        ))
    });
    let Some((par1, par2)) = matched else {
        d.puts("ERROR: OCC26099 is reproduced. No intersection.");
        return Err(crate::support::script::ScriptError(
            "can't read \"par1\": no such variable".into(),
        ));
    };
    let (refpar1, refpar2) = (0.98989794855663704, 0.);
    if (script_real(&par1) - refpar1).abs() > 0.0001
        || (script_real(&par2) - refpar2).abs() > 0.0001
    {
        d.puts(&format!(
            "ERROR: OCC26099 is reproduced. Parameters are {par1} and {par2}, expected 0.98989794855663704 and 0"
        ));
    }
    Ok(())
}
