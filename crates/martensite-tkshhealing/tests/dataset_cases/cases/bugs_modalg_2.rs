//! bugs modalg 2 regression cases owned by
//! TKShHealing whose inputs are in the public dataset. Each body follows its script line
//! by line; the bugs group end only prints `TEST COMPLETED`.
use super::bugs_common::{banner, nbshapes_of};
use crate::support::{
    build::nexplode,
    commands::{explode_type, mkcurve, mksurface},
    restore::Restored,
    session::{ViewStep, set_exploded},
    topology::{mkplane, wire},
};
use crate::support::{
    checkshape::{CheckOptions, checkshape},
    checkshape_structural::checkshape_structural,
    log,
    logged_checks::checknbshapes_some,
    props::{Prop, checkprops},
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::{dt_split_angle, fixshape, removeloc, scaleshape},
};
#[allow(clippy::approx_constant)]
const FULL_TURN_INPUT: f64 = 6.28318530717959;

use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{geom::GeomCurve, top_abs::ShapeEnum};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

/// modalg 2 — bug21.
/// DT_SplitAngle at 90 degrees of two faces sharing an edge.
pub fn bug21(s: &mut dyn Session) {
    banner(&["================", "OCC21", "================", ""]);
    let sh = restore_as(s, "OCC21.brep", "sh").shape();
    checkshape(&sh);
    log::puts("two faces in the compound share one edge");
    let printed = dt_split_angle(&sh, Some(90.));
    let result = set(s, "result", printed.shape);
    log::result(&printed.text);
    log::puts("in the resulting shape sharing is not preserved");
    checkprops(&result, Prop::Area, 12154.8);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug21");
}

/// modalg 2 — bug22968.
/// Removing the locations of a cylindrical face keeps it valid.
pub fn bug22968(s: &mut dyn Session) {
    banner(&["================", "OCC22968", "================", ""]);
    let a = restore_as(s, "OCC22968-cylface.brep", "a").shape();
    checkshape_structural(&a, "a", "f", CheckOptions::default());
    let result = set(s, "result", removeloc(&a));
    let info = checkshape_structural(&result, "result", "f", CheckOptions::default());
    if info.contains("The Shape result looks OK") {
        log::puts("OK OCC22968");
    } else {
        log::puts("Faulty OCC22968");
    }
    checkprops(&result, Prop::Area, 3141.59);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug22968");
}

/// modalg 2 — bug398.
/// `scaleshape result a 10`.
pub fn bug398(s: &mut dyn Session) {
    banner(&["========", "OCC398", "========", ""]);
    let a = restore_as(s, "OCC398.brep", "a").shape();
    let result = scaleshape(&a, 10.)
        .map(|shape| set(s, "result", shape))
        .expect("checknbshapes: result is not a shape (NO MODIFICATIONS)");
    let mut solids = [None; 9];
    solids[5] = Some(1);
    checknbshapes_some(&result, solids);
    checkprops(&result, Prop::Area, 180000.);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug398");
}

/// modalg 2 — bug535.
/// fixshape keeps the number of sub-shapes.
pub fn bug535(s: &mut dyn Session) {
    banner(&["=======", "OCC535", "=======", ""]);
    let a = restore_as(s, "OCC535.brep", "a").shape();
    let r1 = nbshapes_of(&a, "SHAPE");
    let result = set(s, "result", fixshape(&a, &[]));
    let r2 = nbshapes_of(&result, "SHAPE");
    log::puts("");
    log::puts(&format!("before Shapes = {r1}     after Shapes = {r2}"));
    log::puts("");
    if r1 == r2 {
        log::puts("OCC535 OK");
    } else {
        log::puts("OCC535 Faulty");
    }
    checkprops(&result, Prop::Area, 3554.31);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug535");
}

/// modalg 2 — bug548.
/// fixshape at 0.001 leaves no problem in the structural check (OCC548, OCC628).
pub fn bug548(s: &mut dyn Session) {
    banner(&["========", "OCC548", "OCC628", "========", ""]);
    let a = restore_as(s, "CTO908_topo103-t.brep", "a").shape();
    let result = set(s, "result", fixshape(&a, &["0.001"]));
    let che = checkshape_structural(&result, "result", "c", CheckOptions::default());
    if che.contains("problems") {
        log::puts("Faulty OCC628");
    }
    checkprops(&result, Prop::Area, 119630.);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug548");
}

/// `[checkshape f -short]`: the command's result from the topology check, which the
/// script keeps in a variable.
fn checkshape_short(f: &Shape) -> String {
    let analyzer = BRepCheckAnalyzer::new(f, true, false, false)
        .unwrap_or_else(|failure| panic!("checkshape exception : {failure:?}"));
    if analyzer.is_valid().unwrap() {
        "This shape seems to be valid".into()
    } else {
        "This shape has faulty shapes".into()
    }
}

/// `mkedge name curve p1 p2`.
fn mkedge_range(curve: &Handle<dyn GeomCurve>, p1: f64, p2: f64) -> Shape {
    let maker = BRepBuilderAPIMakeEdge::from_curve_params(curve, p1, p2).unwrap();
    maker.edge().unwrap().into_shape()
}

/// `mkplane f wire 0`, the command's error when not done.
fn plane_of(s: &mut dyn Session, w: &Shape) -> Shape {
    set(s, "f", mkplane(w).unwrap_or_else(|error| panic!("{error}")))
}

/// modalg 2 — bug23174.
/// A planar face is rebuilt from the healed cylinder face edges without changing the
/// plane (the cylinder face is closed by a seam and two arcs).
pub fn bug23174(s: &mut dyn Session) {
    banner(&["================", "OCC23174", "================", ""]);
    let mut status = 0;
    let mut maker = BRepPrimAPIMakeCylinder::new(50., 150.);
    let cyl = set(s, "cyl", maker.shape().unwrap().clone());
    let wires = set_exploded(s, "cyl", explode_type(&cyl, ShapeEnum::Wire));
    let f = plane_of(s, &wires[2]);
    let reslog = checkshape_short(&f);
    log::puts(&reslog);
    if reslog != "This shape seems to be valid" {
        status = 1;
    }
    let (edges, warning) = nexplode(&cyl, ShapeEnum::Edge);
    log::result(&warning);
    let edges = set_exploded(s, "cyl", edges);
    let c3d = mkcurve(&edges[0]);
    s.set("c3D", &Restored::Curve(c3d.clone()));
    let e1 = set(s, "e1", mkedge_range(&c3d, 0., 2.));
    let e2 = set(s, "e2", mkedge_range(&c3d, 2., 4.));
    let e3 = set(s, "e3", mkedge_range(&c3d, 4., FULL_TURN_INPUT));
    let cyl_3_r = set(s, "cyl_3_r", edges[2].reversed());
    let w = wire(&[&e1, &e2, &e3, &edges[2], &edges[1], &cyl_3_r]).unwrap();
    let w = set(s, "w", w);
    let (faces, warning) = nexplode(&cyl, ShapeEnum::Face);
    log::result(&warning);
    let faces = set_exploded(s, "cyl", faces);
    let cyl_surf = mksurface(&faces[2]);
    s.set("cyl_surf", &Restored::Surface(cyl_surf.clone()));
    let face_0 =
        BRepBuilderAPIMakeFace::from_surface_wire(&cyl_surf, &topods::wire(&w).unwrap(), true)
            .unwrap()
            .face()
            .unwrap()
            .into_shape();
    let face_0 = set(s, "cyl_face_0", face_0);
    let cyl_face = set(s, "cyl_face", fixshape(&face_0, &[]));
    let face_edges = set_exploded(s, "cyl_face", explode_type(&cyl_face, ShapeEnum::Edge));
    let w = wire(&[&face_edges[1], &face_edges[0], &face_edges[2]]).unwrap();
    let w = set(s, "w", w);
    let f = plane_of(s, &w);
    s.step(ViewStep::Donly(&["f"]));
    // The script keeps this result in `res_log` and checks `reslog` again.
    let _res_log = checkshape_short(&f);
    log::puts(&reslog);
    if reslog != "This shape seems to be valid" {
        status = 1;
    }
    let face_wires = set_exploded(s, "cyl_face", explode_type(&cyl_face, ShapeEnum::Wire));
    let f = plane_of(s, &face_wires[0]);
    let cyl_surf_2 = mksurface(&f);
    s.set("cyl_surf_2", &Restored::Surface(cyl_surf_2.clone()));
    let p1 = cyl_surf.get().unwrap().value(1., 1.).unwrap();
    let p2 = cyl_surf_2.get().unwrap().value(1., 1.).unwrap();
    if p1.x() - p2.x() > 0.0 || p1.y() - p2.y() > 0.0 || p1.z() - p2.z() > 0.0 {
        status = 1;
    }
    log::puts("");
    if status != 0 {
        log::puts("Faulty OCC23174");
    } else {
        log::puts("OK OCC23174");
    }
    let result = set(s, "result", f);
    checkprops(&result, Prop::Area, 47123.9);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug23174");
}
