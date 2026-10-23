//! lowalgos intss regression cases owned by
//! TKShHealing: the surfaces are intersected, each curve's range checked, its distance
//! to both surfaces measured by `xdistcs`, every pair of curves checked not to overlap
//! (`checkoverlapedges`), the number of curves compared, then the 2D screenshot.
use crate::support::{
    geom_create::{cylinder, ellipse, extsurf},
    intersect::{bounds, intersect_surfaces, xdistcs},
    log,
    restore::{Restored, restore_as},
    session::{Session, ViewStep, set},
    swdraw::checkoverlapedges,
    topology::mkedge,
};
use martensite_tkernel::{standard::Handle, storage::format_general};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};

/// `dump name` of a scenario number (precision 15).
fn dump_number(name: &str, value: f64) -> String {
    format!(
        "\n\n*********** Dump of {name} *************\n{}",
        format_general(value, 15)
    )
}

/// The checks of one intersection curve `result_ic` against both surfaces and every
/// earlier curve.
fn check_curve(
    s: &mut dyn Session,
    curves: &[Handle<dyn GeomCurve>],
    ic: usize,
    surfaces: [(&str, &Handle<dyn GeomSurface>, f64); 2],
) {
    let name = format!("result_{ic}");
    let curve = &curves[ic - 1];
    s.step(ViewStep::Display(&[name.as_str()]));
    let (u1, u2) = bounds(curve);
    log::result(&(dump_number("U1", u1) + &dump_number("U2", u2)));
    if u2 - u1 < 1.0e-9 {
        log::puts("Error: Wrong curve's range!");
    }
    for (_, surface, tolerance) in surfaces {
        log::result(&xdistcs(&name, curve, surface, (u1, u2), 10, tolerance).0);
    }
    for ip in (1..ic).rev() {
        let e1 = set(s, "e1", mkedge(curve));
        let e2 = set(s, "e2", mkedge(&curves[ip - 1]));
        let coe = match checkoverlapedges(("e1", "e2"), &e1, &e2, Some(5.0e-5), None) {
            Ok(text) => text,
            Err(text) => panic!("checkoverlapedges failed: {text}"),
        };
        log::puts(&format!("result_{ic} <-> result_{ip}: {coe}"));
        if !coe.contains("Edges are not overlapped") {
            log::puts(&format!(
                "Error: result_{ic} and result_{ip} are overlapped"
            ));
        }
    }
}

/// `intersect result s1 s2`, the per-curve checks and the count, then the views.
fn intss(
    s: &mut dyn Session,
    surfaces: [(&str, &Handle<dyn GeomSurface>, f64); 2],
    image: &str,
) -> usize {
    let curves = intersect_surfaces(surfaces[0].1, surfaces[1].1)
        .unwrap_or_else(|error| panic!("intersect: {error}"));
    // One curve is named `result` and renamed result_1 by the script; several are result_i.
    for (index, curve) in curves.iter().enumerate() {
        s.set(
            &format!("result_{}", index + 1),
            &Restored::Curve(curve.clone()),
        );
    }
    for ic in 1..=curves.len() {
        check_curve(s, &curves, ic, surfaces);
    }
    let names: Vec<String> = (1..=curves.len()).map(|i| format!("result_{i}")).collect();
    let mut all: Vec<&str> = vec![surfaces[0].0, surfaces[1].0];
    all.extend(names.iter().map(String::as_str));
    let results: Vec<&str> = names.iter().map(String::as_str).collect();
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Display(&results));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Clear);
    s.step(ViewStep::Display(&all));
    s.step(ViewStep::Xwd(image));
    curves.len()
}

/// tests/lowalgos/intss/bug27937_2.
/// The extruded ellipse and the cylinder intersect in 4 curves (`cpulimit 100`).
pub fn bug27937_2(s: &mut dyn Session) {
    for line in ["================", "OCC27937", "================", ""] {
        log::puts(line);
    }
    martensite_process_policy::set_process_limit(Some(100))
        .unwrap_or_else(|error| panic!("cpulimit 100: {error}"));
    let c1 = ellipse(&[0., 0., 0., 0., 0., 1., 6.03031367203927, 3.11993062568844]);
    s.set("c1", &Restored::Curve(c1.clone()));
    let s1 = set(s, "s1", extsurf(&c1, [0., 0., 1.]));
    let s2 = set(
        s,
        "s2",
        cylinder(&[0., 0., 0., 1., 0., 0., 0.249128788767645]),
    );
    let found = intss(s, [("s1", &s1, 4.0e-5), ("s2", &s2, 1.0e-5)], "bug27937_2");
    if found == 4 {
        log::puts("OK: Number of curves is good!");
    } else {
        log::puts(&format!("Error: 4 is expected but {found} is found!"));
    }
}

/// tests/lowalgos/intss/bug333.
/// The two restored surfaces intersect in 14 curves without superimposed ones.
pub fn bug333(s: &mut dyn Session) {
    for line in [
        "================",
        "OCC333: Superimposed curves during intersection two surfaces ",
        "================",
        "",
    ] {
        log::puts(line);
    }
    let s1 = restore_as(s, "OCC333a.draw", "s1").surface();
    let s2 = restore_as(s, "OCC333b.draw", "s2").surface();
    let found = intss(s, [("s1", &s1, 2.0e-7), ("s2", &s2, 2.0e-7)], "bug333");
    if found == 14 {
        log::puts("OK: Number of curves is good!");
    } else {
        log::puts(&format!(
            "Error: 14 curves are expected but {found} ones are found!"
        ));
    }
}
