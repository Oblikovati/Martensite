//! `bugs_modalg_6` case bodies (the inputs of bug26016, bug26387, bug26406 and
//! bug28003 are public dataset members).
use crate::support::{
    checkshape::checkshape,
    commands::mksurface,
    curves::{add, explode, polyline, shape_set},
    log,
    misc::{checknbshapes, occ26313},
    prim::{PrismFlag, box_at, box_sizes, prism, psphere, revol, whatis},
    props_view::{Expected, checkarea, checkprops},
    restore::{Restored, restore_as},
    session::{Session, ViewStep, checkview_display},
    text::lindex,
    topology::{compound, mkface, nurbsconvert, setflags_locked, tcopy},
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// A new box is Free, adding it to a compound freezes it, and its `tcopy` is
/// Free again.
pub fn bug21727(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC21727");
    log::puts("========");
    log::puts("");
    let b = shape_set(s, "b", box_sizes(10., 10., 10.));
    let bug_info = whatis("b", &b);
    if lindex(&bug_info, 6) != "Free" {
        log::puts("ERROR: OCC21727 is reproduced. Shape b does not have flag FREE.");
    }
    shape_set(s, "c", compound(&[&b]));
    let bug_info = whatis("b", &b);
    if lindex(&bug_info, 6) == "Free" {
        log::puts("ERROR: OCC21727 is reproduced. Shape b has flag FREE.");
    }
    let q = shape_set(s, "q", tcopy(&b));
    log::result("q ");
    let bug_info = whatis("q", &q);
    if lindex(&bug_info, 6) != "Free" {
        log::puts("ERROR: OCC21727 is reproduced. Shape q does not have flag FREE.");
    }
}

/// `nurbsconvert` leaves the locked source sphere valid.
pub fn bug25957_1(s: &mut dyn Session) {
    log::puts("============");
    log::puts("OCC25957");
    log::puts("============");
    log::puts("");
    let sphere = shape_set(s, "s", psphere(10.));
    setflags_locked(&sphere);
    shape_set(s, "q", nurbsconvert(&sphere));
    checkshape(&sphere);
}

/// The revolution of the restored face by 90 degrees is valid.
pub fn bug26016(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC26016");
    log::puts("========");
    log::puts("");
    s.step(ViewStep::SmallView("AXON"));
    let a = restore_as(s, "OCC26016-revolve-crash.brep", "a").shape();
    let r = revol(&a, [0., 0., 0.], [0., 1., 0.], 90., false).expect("revol r");
    let r = shape_set(s, "r", r.shape);
    let bug_info = checkshape(&r);
    if bug_info.text != "This shape seems to be valid" {
        log::puts("ERROR: OCC26016 is reproduced");
    }
    s.step(ViewStep::Donly(&["r"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug26016"));
}

/// The modified shape of BRepBuilderAPI_GTransform has the box's sub-shapes.
pub fn bug26313(s: &mut dyn Session) {
    log::puts("============");
    log::puts("OCC26313");
    log::puts("============");
    log::puts("");
    let bb = shape_set(s, "bb", box_sizes(100., 100., 100.));
    let rr = occ26313(&bb).expect("OCC26313 rr bb");
    let rr = shape_set(s, "rr", rr);
    let expected = "
Number of shapes in shape
 VERTEX    : 8
 EDGE      : 12
 WIRE      : 6
 FACE      : 6
 SHELL     : 1
 SOLID     : 1
 COMPSOLID : 0
 COMPOUND  : 0
 SHAPE     : 34
";
    checknbshapes("rr", &rr, [None; 9], Some(expected), true, "Modified Shape");
}

/// The copied prism of the restored paraboloid face is valid.
pub fn bug26387(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC26387");
    log::puts("========");
    log::puts("");
    let p = restore_as(s, "OCC26387-Paraboloid.brep", "p").shape();
    let r = prism(&p, [0., 0., 5.], PrismFlag::Copy).unwrap().shape;
    let r = shape_set(s, "r", r);
    checkshape(&r);
}

/// The copied half revolution of the restored paraboloid face is valid.
pub fn bug26406(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC26406");
    log::puts("========");
    log::puts("");
    s.step(ViewStep::SmallView("AXON"));
    let p = restore_as(s, "OCC26406-Paraboloid.brep", "p").shape();
    let r = revol(&p, [0., 0., 0.], [0., 0., 1.], 180., true).expect("revol r");
    let r = shape_set(s, "r", r.shape);
    checkshape(&r);
    s.step(ViewStep::Xwd("bug26406"));
}

/// The face on the plane of a box face with the outer wires of two disjoint
/// box faces added has the area -200.
pub fn bug26607(s: &mut dyn Session) {
    log::puts("==========");
    log::puts("OCC26607");
    log::puts("==========");
    log::puts("");
    let b1 = shape_set(s, "b1", box_sizes(10., 10., 10.));
    let b2 = shape_set(s, "b2", box_at([20., 0., 0.], [10., 10., 10.]));
    let b1_faces = explode(s, "b1", &b1, Some(ShapeEnum::Face));
    let b2_faces = explode(s, "b2", &b2, Some(ShapeEnum::Face));
    let b1_5 = explode(s, "b1_5", &b1_faces[4], None);
    let b2_5 = explode(s, "b2_5", &b2_faces[4], None);
    let surface = mksurface(&b1_faces[4]);
    s.set("s", &Restored::Surface(surface.clone()));
    let fs = shape_set(s, "fs", mkface(&surface));
    let fs = shape_set(s, "fs", add(&b1_5[0], fs));
    let fs = shape_set(s, "fs", add(&b2_5[0], fs));
    checkarea(s, "fs", &fs, -200., 1e-06, 0.01);
}

/// The full revolution of a face with a straight B-spline edge on the axis.
pub fn bug28003(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC28003");
    log::puts("========");
    log::puts("");
    let face = restore_as(s, "bug28003_Surf_Bezier.brep", "s").shape();
    let result = revol(&face, [0., 0., 0.], [0., 0., 1.], 360., false).expect("revol");
    let result = shape_set(s, "result", result.shape);
    checkshape(&result);
    checkprops(s, "result", &result, Expected::s(129536.));
    checkview_display(s, Some("result"), "bug28003");
}

/// The revolution of a B-spline triangle wire.
pub fn bug28326(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC28326");
    log::puts("========");
    log::puts("");
    let w = shape_set(
        s,
        "w",
        polyline(&[[0., 0., 0.], [10., 0., 0.], [10., 5., 0.], [0., 0., 0.]]),
    );
    let wn = shape_set(s, "wn", nurbsconvert(&w));
    let result = revol(&wn, [0., 0., 0.], [1., 0., 0.], 90., false).expect("revol");
    let result = shape_set(s, "result", result.shape);
    checkshape(&result);
    checkprops(s, "result", &result, Expected::s(63.54));
    checkview_display(s, Some("result"), "bug28326");
}
