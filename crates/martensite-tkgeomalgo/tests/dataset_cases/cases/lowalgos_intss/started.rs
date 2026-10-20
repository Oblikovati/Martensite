//! `lowalgos_intss` cases with a start
//! point, trimmed bounds, lengths, projections from an intersection point, or their own
//! captures.
use crate::cases::script::banner;
use crate::cases::script_words::{string_range, words};
use crate::support::commands::checks::checklength;
use crate::support::commands::extrema::proj;
use crate::support::commands::geometry::{cvalue, trim_one_direction};
use crate::support::commands::intersect::{IntersectOptions, intersect, intersect_with};
use crate::support::commands::variables::{dump, point, whatis};
use crate::support::commands::views::{
    checkview_display, checkview_screenshot, clear, clpoles, display, donly, fit, smallview, xwd,
    zoom,
};
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

/// `clear`, `display` of the two surfaces and the curve, `xwd` of the snapshot.
fn snapshot(d: &mut Script, surfaces: [&str; 2], curve: &str) -> ScriptStep {
    clear(d)?;
    display(d, &[surfaces[0]])?;
    display(d, &[surfaces[1]])?;
    display(d, &[curve])?;
    xwd(d, &format!("bug24772_{curve}"))
}

pub fn bug24772(d: &mut Script) -> ScriptStep {
    banner(d, "============", "OCC24772");
    restore(d, "bug24772_s1.draw", "s1")?;
    restore(d, "bug24772_s2.draw", "s2")?;
    restore(d, "bug24772_s3.draw", "s3")?;
    smallview(d, "+X+Y")?;
    fit(d)?;
    zoom(d, 6.)?;
    let info_1 = intersect(d, "r1", "s1", "s2")?;
    let info_2 = intersect(d, "r2", "s2", "s3")?;
    for (info, result, surfaces) in [(info_1, "r1", ["s1", "s2"]), (info_2, "r2", ["s2", "s3"])] {
        if words(&info).len() != 3 {
            d.puts("ERROR: OCC24722 is reproduced.");
            continue;
        }
        for k in 1..=3 {
            snapshot(d, surfaces, &format!("{result}_{k}"))?;
        }
    }
    Ok(())
}

/// The two curves of an intersection of variable curvature and their lengths.
fn curvature_lengths(
    d: &mut Script,
    files: [(&str, &str); 2],
    hide_poles: bool,
    lengths: [&str; 2],
    image: &str,
) -> ScriptStep {
    banner(d, "============", "OCC27079");
    restore(d, files[0].0, files[0].1)?;
    restore(d, files[1].0, files[1].1)?;
    if hide_poles {
        clpoles(d, files[1].1)?;
    }
    smallview(d, "AXON")?;
    donly(d, &[files[1].1])?;
    fit(d)?;
    intersect(d, "result", files[0].1, files[1].1)?;
    checklength(d, "result_1", lengths[0], None)?;
    checklength(d, "result_2", lengths[1], None)?;
    checkview_screenshot(d, image)
}

pub fn bug27079_1(d: &mut Script) -> ScriptStep {
    let files = [("bug27079_s1.draw", "s1"), ("bug27079_s2.draw", "s2")];
    curvature_lengths(
        d,
        files,
        true,
        ["6.8873540591440428", "6.8873330997321212"],
        "bug27079_1",
    )
}

pub fn bug27079_2(d: &mut Script) -> ScriptStep {
    let files = [("bug27079_s3.draw", "s3"), ("bug27079_s4.draw", "s4")];
    curvature_lengths(
        d,
        files,
        false,
        ["4.2844275555620923", "4.2844275531269931"],
        "bug27079_2",
    )
}

pub fn bug271(d: &mut Script) -> ScriptStep {
    banner(d, "============", " OCC271 ");
    restore(d, "OCC271a.rle", "su1")?;
    restore(d, "OCC271b.rle", "su2")?;
    let (u1, u2) = (2.315721254379057e-16, 0.01131870392278708);
    let (v1, v2) = (226.9760100729095, 203.7283534809051);
    let options = IntersectOptions {
        start: Some([u1, v1, u2, v2]),
        tolerance: None,
    };
    intersect_with(d, "result", ["su1", "su2"], options)?;
    let che = whatis(d, "result")?;
    if !che.contains("3d") || !che.contains("curve") {
        d.puts("Faulty OCC271: command INTERSECT works wrongly");
    } else {
        d.puts("OCC271 OK : command INTERSECT works properly");
    }
    checkview_display(d, "result", "bug271")
}

pub fn bug27950(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC27950 ALL: ERROR: OCC27950 is reproduced.");
    banner(d, "============", "OCC27950");
    restore(d, "bug27950_s1.draw", "s1")?;
    restore(d, "bug27950_s2.draw", "s2")?;
    let steps = [
        None,
        Some((200., 300.)),
        Some((200., 220.)),
        Some((205.38050051360744, 210.55652011282123)),
        Some((205., 211.)),
        Some((206., 210.)),
        Some((205.4, 210.575)),
        Some((205.353, 210.529)),
    ];
    for (index, range) in steps.into_iter().enumerate() {
        let second = match range {
            None => "s2",
            Some(range) => {
                trim_one_direction(d, false, "s2t", "s2", range)?;
                "s2t"
            }
        };
        let info = intersect(d, &format!("i{}", index + 1), "s1", second)?;
        let count = words(info.trim()).len();
        if count != 4 {
            let step = index + 1;
            d.puts(&format!(
                "ERROR: OCC27950 is reproduced. Step #{step}. Result of intersection is {count}."
            ));
        }
    }
    Ok(())
}

pub fn bug28012(d: &mut Script) -> ScriptStep {
    banner(d, "============", "OCC28012");
    restore(d, "bug28012_s1.draw", "s1")?;
    restore(d, "bug28012_s2.draw", "s2")?;
    let start = [
        0.00012162815337817315,
        0.0010404164685680738,
        0.99999999999999989,
        0.22068942273845651,
    ];
    let options = IntersectOptions {
        start: Some(start),
        tolerance: None,
    };
    intersect_with(d, "result", ["s1", "s2"], options)?;
    smallview(d, "AXON")?;
    fit(d)?;
    checkview_screenshot(d, "bug28012")
}

/// `cvalue curve u x y z`, `point p x y z`, `proj surface x y z`.
fn projected_intersection_point(d: &mut Script, curve: &str, u: f64) -> ScriptStep {
    let p = cvalue(d, curve, u, ["x", "y", "z"])?;
    point(d, "p", [p.x(), p.y(), p.z()])?;
    proj(d, "b", [p.x(), p.y(), p.z()])?;
    Ok(())
}

/// `regexp {Point +: +(x, +y, +z)} [dump name]`: the coordinates text, empty without one.
fn dumped_point(d: &mut Script, name: &str) -> ScriptStep<String> {
    let text = dump(d, &[name])?;
    let Some(start) = text.find("Point : ") else {
        return Ok(String::new());
    };
    let line = &text[start + "Point : ".len()..];
    Ok(line.lines().next().unwrap_or("").trim_end().to_string())
}

pub fn bug286(d: &mut Script) -> ScriptStep {
    d.puts("TODO OCC12345 ALL: Faulty OCC286: Attempt to project the point on surface gives wrong result.");
    banner(d, "============", " OCC286 ");
    restore(d, "OCC286a.draw", "a")?;
    restore(d, "OCC286b.draw", "b")?;
    intersect(d, "i", "a", "b")?;
    projected_intersection_point(d, "i", 0.31)?;
    let err = [
        dumped_point(d, "ext_1")?,
        dumped_point(d, "ext_2")?,
        dumped_point(d, "ext_3")?,
    ];
    if err[0] == err[1] || err[0] == err[2] || err[1] == err[2] {
        d.puts("Faulty OCC286: Attempt to project the point on surface gives wrong result.");
    } else {
        d.puts("OCC286 OK : Attempt to project the point on surface gives correct result");
    }
    checkview_display(d, "result", "bug286")
}

pub fn bug429(d: &mut Script) -> ScriptStep {
    banner(d, "============", " OCC429 ");
    restore(d, "OCC429a.draw", "a")?;
    restore(d, "OCC429b.draw", "b")?;
    intersect(d, "i", "a", "b")?;
    projected_intersection_point(d, "i_2", 0.43213918)?;
    let mut all_parameters = true;
    for k in 1..=6 {
        let text = dump(d, &[&format!("ext_{k}")])?;
        all_parameters &= string_range(&text, 55, 64) == "Parameters";
    }
    if !all_parameters {
        d.puts(" OCC286 OK : Projection for a point on a surface works properly");
    } else {
        d.puts("Faulty OCC429 : Attempt to project the point on surface gives incorrect result");
    }
    checkview_display(d, "result", "bug429")
}
