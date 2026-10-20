//! `bugs_modalg_8` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checkshape::checkshape,
    commands::explode,
    distance::distmini,
    log,
    logged_checks::{checkreal, checktrinfo, trinfo_counts, trinfo_logged},
    props::{Prop, checkprops_eps, props_logged},
    restore::restore_as,
    session::{Session, checkview_display, set, set_exploded},
    topology::{
        SAMEPARAMETER_TOLERANCE, deform, sameparameter_in_place, trotate, tscale, ttranslate,
    },
};
use martensite_tkbrep::topods::Shape;

/// The face, node and triangle counts and the printed volume of the triangulation-only
/// shape (`trinfo`, `vprops`).
fn reference(s: &mut dyn Session) -> (Shape, (usize, usize, usize), f64) {
    let shape = restore_as(s, "bug31479_P-51-Mustang-2.brep", "s").shape();
    let (faces, triangles, nodes) = trinfo_counts(&trinfo_logged(&shape));
    let mass = props_logged(Prop::Volume, &shape, None).read_mass();
    (shape, (faces, nodes, triangles), mass)
}

/// 0031479: scaling, rotation and translation with `-copymesh` keep the triangulation
/// counts, and the volume scales by 8 (1e-5).
pub fn bug31479_1(s: &mut dyn Session) {
    let (shape, (faces, nodes, triangles), mass) = reference(s);
    let scale = 2.0;
    let mass = mass * scale * scale * scale;
    let scaled = set(s, "s", tscale(&shape, [0., 0., 0.], scale, true));
    checktrinfo(&scaled, faces, nodes, triangles);
    checkprops_eps(&scaled, Prop::Volume, mass, 1.0e-5);
    let rotated = set(
        s,
        "s",
        trotate(&scaled, [0., 0., 0.], [0., 0., 1.], 90., true),
    );
    checktrinfo(&rotated, faces, nodes, triangles);
    checkprops_eps(&rotated, Prop::Volume, mass, 1.0e-5);
    let moved = set(s, "s", ttranslate(&rotated, [0., 0., 10.], true));
    checktrinfo(&moved, faces, nodes, triangles);
    checkprops_eps(&moved, Prop::Volume, mass, 1.0e-5);
    checkview_display(s, Some("s"), "bug31479_1");
}

/// 0031479: `deform res s 2 3 4` keeps the counts and multiplies the volume by 24.
pub fn bug31479_2(s: &mut dyn Session) {
    let (shape, (faces, nodes, triangles), mass) = reference(s);
    let res = set(s, "res", deform(&shape, [2., 3., 4.]));
    checktrinfo(&res, faces, nodes, triangles);
    checkprops_eps(&res, Prop::Volume, mass * 2. * 3. * 4., 1.0e-5);
    checkview_display(s, Some("res"), "bug31479_2");
}

/// 0033173: distmini of the two sub-shapes is 7.049107166484117e-07 (1e-7, 1%).
pub fn bug33173(s: &mut dyn Session) {
    let min_dist_expected = 7.049107166484117e-07;
    let shape = restore_as(s, "bug33173.brep", "s").shape();
    let parts = set_exploded(s, "s", explode(&shape));
    match distmini(s, "dm", &parts[0], &parts[1], None) {
        Some(d) if !d.solutions.is_empty() => {
            checkreal("MinDist", min_dist_expected, d.value, 1.0e-7, 0.01);
        }
        _ => log::puts("Error: distmini failed"),
    }
}

/// 0033179: `fsameparameter s` must not crash; checkshape.
pub fn bug33179(s: &mut dyn Session) {
    let shape = restore_as(s, "bug33179.brep", "s").shape();
    // `fsameparameter s`: `Script::Atof("s")` is 0, so the default 1e-7, forced.
    sameparameter_in_place(&shape, SAMEPARAMETER_TOLERANCE, true);
    checkshape(&shape);
}

/// `BRepBndLib::AddClose` of the restored shape must not crash; it prints the box
/// corners (`%.17g` each). The input is an issue attachment
/// (`github-issues/OCCT-466/cutCompound.shape` of the test data).
pub fn bug_gh466(s: &mut dyn Session) {
    let result = restore_as(s, "bug_gh466.brep", "result").shape();
    let mut bounds = martensite_tkmath::bnd::BoundingBox3::default();
    martensite_tktopalgo::brep_bnd_lib::BRepBndLib::add_close(&result, &mut bounds).unwrap();
    let corners = bounds.get().unwrap();
    let text: Vec<String> = corners
        .iter()
        .map(|v| crate::support::printed::g17(*v))
        .collect();
    log::result(&text.join(" "));
}
