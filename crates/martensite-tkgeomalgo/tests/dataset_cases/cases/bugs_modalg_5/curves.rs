//! `bugs_modalg_5` case bodies on curve
//! extrema, 2D intersections and thread-safe surface intersections.
use crate::cases::script::{banner, captured};
use crate::cases::script_words::{number_after, words};
use crate::support::commands::extrema::extrema;
use crate::support::commands::planar::intersect2d;
use crate::support::commands::qa::occ23952intersect;
use crate::support::commands::topology::{explode_type, mkcurve, pcurve};
use crate::support::commands::variables::dump;
use crate::support::commands::views::{checkview_screenshot, fit, fit2d, smallview, view, xwd};
use crate::support::numbers::script_real;
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug23952_2(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC23952");
    restore(d, "bug23952_s1.draw", "a")?;
    restore(d, "bug23952_s2.draw", "b")?;
    occ23952intersect(d, 6, ["a", "b"])
}

/// The `Intersection point 1 : x y` and `parameter on the fist: p parameter on the
/// second: q` numbers of a `2dintersect` text.
fn first_intersection(info: &str, names: [&str; 4]) -> ScriptStep<[f64; 4]> {
    let point = captured(info.split("Intersection point 1 : ").nth(1), names[0])?;
    let mut coordinates = point.split_whitespace();
    let x = captured(coordinates.next(), names[0])?;
    let y = captured(coordinates.next(), names[1])?;
    let first = captured(number_after(info, "parameter on the fist:"), names[2])?;
    let second = captured(number_after(info, "parameter on the second:"), names[3])?;
    Ok([x, y, first, second].map(script_real))
}

pub fn bug24657(d: &mut Script) -> ScriptStep {
    d.puts("============");
    d.puts("0024657: 2dintersect returns different number of intersect points");
    d.puts("===========");
    d.puts("");
    restore(d, "bug24657.brep", "a")?;
    explode_type(d, "a", ShapeEnum::Edge)?;
    pcurve(d, "p4", "a_4", "a")?;
    pcurve(d, "p5", "a_5", "a")?;
    let inter1 = intersect2d(d, &["p4", "p5"], Some(1.0e-7), false)?;
    let inter2 = intersect2d(d, &["p5", "p4"], Some(1.0e-7), false)?;
    let (int1, int2) = (
        inter1.matches("Intersection point").count(),
        inter2.matches("Intersection point").count(),
    );
    if int1 != int2 || int1 != 1 {
        d.puts("Error : The intersection points number is different!");
    }
    let [x1, y1, par11, par12] = first_intersection(&inter1, ["x1", "y1", "par11", "par12"])?;
    let [x2, y2, par21, par22] = first_intersection(&inter2, ["x2", "y2", "par21", "par22"])?;
    if (x1 - x2).abs() > 1.0e-7 || (y1 - y2).abs() > 1.0e-7 {
        d.puts("Error : The intersection points do not match!");
    }
    if (par11 - par22).abs() > 1.0e-7 || (par21 - par12).abs() > 1.0e-7 {
        d.puts("Error : The intersection points parameters do not match!");
    }
    view(d, "-2D-", [728, 20, 400, 400])?;
    fit2d(d)?;
    xwd(d, "bug24657")
}

/// `extrema c1 c2` must publish exactly two words, then the view.
fn two_extrema(d: &mut Script, files: [&str; 2], image: &str) -> ScriptStep {
    d.puts("==========");
    d.puts("OCC25004  ");
    d.puts("==========");
    d.puts("");
    restore(d, files[0], "c1")?;
    restore(d, files[1], "c2")?;
    let list = extrema(d, "c1", "c2")?;
    if words(&list).len() == 2 {
        d.puts("OK : command extrema works properly");
    } else {
        d.puts("Error : command extrema does NOT work properly");
    }
    smallview(d, "AXON")?;
    fit(d)?;
    checkview_screenshot(d, image)
}

pub fn bug25004_2(d: &mut Script) -> ScriptStep {
    two_extrema(d, ["bug25004_c1.draw", "bug25004_c2.draw"], "bug25004_2")
}

pub fn bug25004_3(d: &mut Script) -> ScriptStep {
    two_extrema(d, ["bug25004_bc1.draw", "bug25004_bc2.draw"], "bug25004_3")
}

/// `regexp "Parameters : 0 +([-0-9*.+eE]+)" [dump ext_i]`: the segment length.
fn segment_length(d: &mut Script, name: &str, variable: &str) -> ScriptStep<f64> {
    let info = dump(d, &[name])?;
    let length = captured(number_after(&info, "Parameters : 0"), variable)?;
    Ok(script_real(length))
}

pub fn bug25708(d: &mut Script) -> ScriptStep {
    banner(d, "========", "OCC25708");
    restore(d, "bug25708_interror.brep", "b")?;
    explode_type(d, "b", ShapeEnum::Edge)?;
    mkcurve(d, "c1", "b_1")?;
    mkcurve(d, "c2", "b_2")?;
    let result = extrema(d, "c1", "c2")?;
    if words(&result).len() != 2 {
        d.puts("Error: GeomAPI_ExtremaCurveCurve does not return all intersection points");
        return Ok(());
    }
    d.puts("OK: GeomAPI_ExtremaCurveCurve return all intersection points");
    for (index, name) in ["ext_1", "ext_2"].into_iter().enumerate() {
        let step = index + 1;
        if segment_length(d, name, &format!("extLength{step}"))? > 1e-14 {
            d.puts(&format!("{step}. Error: bad distance points obtained"));
        } else {
            d.puts(&format!(
                "{step}. OK: good distance between obtained points "
            ));
        }
    }
    Ok(())
}
