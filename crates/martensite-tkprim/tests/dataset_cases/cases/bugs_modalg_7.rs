//! `bugs_modalg_7` case bodies (the inputs of bug26516 and bug28402 are public
//! dataset members).
use crate::support::{
    bounding::{Bounding, bounding, isbbinterf},
    checks::nbshapes,
    checkshape::checkshape,
    commands::{dump_curve, mkcurve},
    curves::{
        ellipse, explode, imagedir, mkface_wire, plane_default, plane_normal, shape_set, sphere,
        trim_curve,
    },
    geometry::line,
    log,
    logged_checks::{checknbshapes, checkreal},
    misc::brestore,
    prim::{PrismFlag, box_at, box_sizes, dump_history, prism, ptorus, ptorus_pipe_angle, revol},
    printed::g17,
    props::Prop,
    props_view::{Expected, checkprops, props_command},
    restore::{Restored, restore_as},
    session::{Session, ViewStep, checkview_display},
    text::numbers_after,
    topology::{compound, mkedge, mkface_bounds, trotate, validrange, vertex, wire},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use std::time::Instant;

/// The torus segment with the latitudes 90..270 and the angle 45 is valid.
pub fn bug23612(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC23612");
    log::puts("========");
    log::puts("");
    let p = shape_set(s, "p", ptorus_pipe_angle(10., 2., [90., 270., 45.]));
    checkshape(&p);
    checkview_display(s, Some("p"), "bug23612");
}

/// The revolution of the restored shape by 45 degrees is valid.
pub fn bug26516(s: &mut dyn Session) {
    log::puts("========");
    log::puts("OCC26516");
    log::puts("========");
    log::puts("");
    let a = restore_as(s, "bug26516_tmp.brep", "a").shape();
    let result = revol(&a, [0., 0., 0.], [1., 0., 0.], 45., false).expect("revol");
    let result = shape_set(s, "result", result.shape);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug26516");
}

/// `regexp {Elapsed time: ...} [dchrono name show]`: the elapsed seconds.
fn elapsed_seconds(start: Instant) -> f64 {
    start.elapsed().as_secs_f64()
}

/// `vprops` of the restored deep
/// assembly takes more than 100 times as long as `vprops` of a box.
pub fn bug28402(s: &mut dyn Session) {
    log::todo("Error: calculation of properties of complex compounds is slow");
    log::puts("========");
    log::puts("OCC28402");
    log::puts("========");
    log::puts("");
    let a = restore_as(s, "bug28402_hugeassembly.brep", "a").shape();
    log::result(&nbshapes(&a));
    let start = Instant::now();
    props_command(s, Prop::Volume, &a, None, false);
    let a_time = elapsed_seconds(start);
    log::result(&format!("COUNTER CPU vprops compounds: {}\n", g17(a_time)));
    let b = shape_set(s, "b", box_sizes(100., 100., 100.));
    let start = Instant::now();
    props_command(s, Prop::Volume, &b, None, false);
    let b_time = elapsed_seconds(start);
    log::result(&format!("COUNTER CPU vprops box: {}\n", g17(b_time)));
    if a_time / b_time > 100. {
        log::puts("Error: calculation of properties of complex compounds is slow");
    }
}

fn banner_29311() {
    log::puts("========");
    log::puts("OCC29311");
    log::puts("========");
    log::puts("");
}

/// `bounding` of explicit corners builds the box with the volume 910000.
pub fn bug29311_3(s: &mut dyn Session) {
    banner_29311();
    let out = bounding(
        s,
        Bounding {
            min_max: Some([50., 100., 30., 180., 200., 100.]),
            result_shape: Some("result"),
            ..Bounding::default()
        },
    );
    let result = out.shape.expect("bounding -shape result");
    checkprops(s, "result", &result, Expected::v(910000.));
}

/// The AABB and OBB of a rotated torus, their volumes and the view.
pub fn bug29311_4(s: &mut dyn Session) {
    banner_29311();
    let result = shape_set(s, "result", ptorus(20., 5.));
    let result = shape_set(
        s,
        "result",
        trotate(&result, [5., 10., 15.], [1., 1., 1.], 28., false),
    );
    log::puts("AABB");
    let aabb = bounding(
        s,
        Bounding {
            result_shape: Some("ra"),
            print: true,
            save: true,
            ..Bounding::of(&result)
        },
    );
    let [x1, y1, z1, x2, y2, z2] = aabb.saved.expect("bounding -save");
    // `dump x1 y1 z1 x2 y2 z2`: each number at precision 15.
    let mut dump = String::new();
    for (name, value) in [
        ("x1", x1),
        ("y1", y1),
        ("z1", z1),
        ("x2", x2),
        ("y2", y2),
        ("z2", z2),
    ] {
        dump.push_str(&format!(
            "\n\n*********** Dump of {name} *************\n{}",
            martensite_tkernel::storage::format_general(value, 15)
        ));
    }
    log::result(&dump);
    let va_exp = (x2 - x1) * (y2 - y1) * (z2 - z1);
    let ra = aabb.shape.expect("bounding -shape ra");
    checkprops(s, "ra", &ra, Expected::v(va_exp));
    log::puts("OBB");
    let obb = bounding(
        s,
        Bounding {
            result_shape: Some("ro"),
            print: true,
            obb: true,
            ..Bounding::of(&result)
        },
    );
    let ro = obb.shape.expect("bounding -shape ro");
    checkprops(s, "ro", &ro, Expected::v(28694.7));
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug29311_4"));
}

fn interference(first: &Shape, second: &Shape, oriented: bool, expect_out: bool) {
    let kind = if oriented { "OBB" } else { "AABB" };
    let text = isbbinterf(first, second, oriented);
    let out = text.contains(&format!("NOT interfered by {kind}"));
    if out != expect_out {
        log::puts(&format!("Error : Wrong check of {kind}s interferences"));
    } else {
        log::puts(&format!("OK : check of {kind}s interferences"));
    }
}

/// Two disjoint boxes are NOT interfered by AABB, in both orders.
pub fn bug29311_5(s: &mut dyn Session) {
    banner_29311();
    let b1 = shape_set(s, "b1", box_at([100., 60., 140.], [20., 10., 80.]));
    let b2 = shape_set(s, "b2", box_at([210., 200., 80.], [120., 60., 90.]));
    interference(&b1, &b2, false, true);
    interference(&b2, &b1, false, true);
}

/// A box inside another interferes by AABB, in both orders.
pub fn bug29311_6(s: &mut dyn Session) {
    banner_29311();
    let b1 = shape_set(s, "b1", box_sizes(300., 300., 300.));
    let b2 = shape_set(s, "b2", box_at([100., 100., 100.], [50., 50., 50.]));
    interference(&b1, &b2, false, false);
    interference(&b2, &b1, false, false);
}

/// Partly overlapping boxes interfere by AABB, in both orders.
pub fn bug29311_7(s: &mut dyn Session) {
    banner_29311();
    let b1 = shape_set(s, "b1", box_sizes(300., 300., 300.));
    let b2 = shape_set(s, "b2", box_at([200., 200., 200.], [111., 222., 333.]));
    interference(&b1, &b2, false, false);
    interference(&b2, &b1, false, false);
}

/// Two copies of a box rotated about different axes are NOT interfered by OBB.
pub fn bug29311_8(s: &mut dyn Session) {
    banner_29311();
    let b1 = shape_set(s, "b1", box_sizes(100., 150., 200.));
    let b2 = shape_set(s, "b2", b1.clone());
    let b1 = shape_set(
        s,
        "b1",
        trotate(&b1, [-150., -150., -150.], [1., 2., 3.], -40., false),
    );
    let b2 = shape_set(
        s,
        "b2",
        trotate(&b2, [-150., -150., -150.], [1., 5., 2.], 60., false),
    );
    interference(&b1, &b2, true, true);
    interference(&b2, &b1, true, true);
}

/// A rotated box inside a rotated box interferes by OBB, in both orders.
pub fn bug29311_9(s: &mut dyn Session) {
    banner_29311();
    let b1 = shape_set(s, "b1", box_at([-300., -300., -300.], [600., 600., 600.]));
    let b2 = shape_set(s, "b2", box_sizes(100., 100., 100.));
    let b1 = shape_set(
        s,
        "b1",
        trotate(&b1, [0., 0., 0.], [1., 2., 3.], -28., false),
    );
    let b2 = shape_set(
        s,
        "b2",
        trotate(&b2, [0., 0., 0.], [5., 6., 7.], 34., false),
    );
    interference(&b1, &b2, true, false);
    interference(&b2, &b1, true, false);
}

/// Two copies of a box rotated about one axis interfere by OBB.
pub fn bug29311_10(s: &mut dyn Session) {
    banner_29311();
    let b1 = shape_set(s, "b1", box_sizes(100., 150., 200.));
    let b2 = shape_set(s, "b2", b1.clone());
    let b1 = shape_set(
        s,
        "b1",
        trotate(&b1, [-50., -50., -50.], [1., 1., 1.], -40., false),
    );
    let b2 = shape_set(
        s,
        "b2",
        trotate(&b2, [-50., -50., -50.], [1., 1., 1.], 60., false),
    );
    interference(&b1, &b2, true, false);
    interference(&b2, &b1, true, false);
}

/// Boxes interfered by AABB but NOT by OBB.
pub fn bug29311_11(s: &mut dyn Session) {
    banner_29311();
    let b1 = shape_set(s, "b1", box_sizes(300., 100., 100.));
    let b1 = shape_set(
        s,
        "b1",
        trotate(&b1, [0., 0., 0.], [0., 1., 0.], -30., false),
    );
    let b2 = shape_set(s, "b2", box_at([200., 0., 50.], [50., 50., 50.]));
    interference(&b1, &b2, false, false);
    interference(&b2, &b1, false, false);
    interference(&b1, &b2, true, true);
    interference(&b2, &b1, true, true);
}

/// The OBB of a trimmed line edge is computed (no exception).
pub fn bug29311_12(s: &mut dyn Session) {
    banner_29311();
    let ll = line([-5., 3., 8.], [-1., 0., 0.]);
    s.set("ll", &Restored::Curve(ll.clone()));
    let ll = trim_curve(&ll, -100., 100.);
    s.set("ll", &Restored::Curve(ll.clone()));
    let result = shape_set(s, "result", mkedge(&ll));
    bounding(
        s,
        Bounding {
            obb: true,
            ..Bounding::of(&result)
        },
    );
}

/// The OBB of a vertex is centred on it.
pub fn bug29311_15(s: &mut dyn Session) {
    banner_29311();
    let (px, py, pz) = (1., 2., 3.);
    let vv = shape_set(s, "vv", vertex(Point3::new(px, py, pz)));
    let log_text = bounding(
        s,
        Bounding {
            obb: true,
            print: true,
            ..Bounding::of(&vv)
        },
    )
    .text;
    let Some([xc, yc, zc]) =
        numbers_after(&log_text, "Center: ", 3, "").map(|v| [v[0], v[1], v[2]])
    else {
        log::puts("Error in Dump.");
        panic!("can't read \"xc\": no such variable");
    };
    checkreal("XC", xc, px, 1.0e-7, 0.0);
    checkreal("YC", yc, py, 1.0e-7, 0.0);
    checkreal("ZC", zc, pz, 1.0e-7, 0.0);
}

/// 0030154: the valid range of each edge of a huge box is within 3e-7 of its curve range.
pub fn bug30154_2(s: &mut dyn Session) {
    log::puts("========");
    log::puts(
        "0030154: Modeling Algorithms - Boolean Operation on planar geometry hangs inside BRepLib::FindValidRange()",
    );
    log::puts("========");
    log::puts("");
    let toler = 3.0e-7;
    let b = shape_set(
        s,
        "b",
        box_at(
            [-94190864., -46229000., -17178478.4],
            [519231048., 93653000., 120025348.8],
        ),
    );
    for e in explode(s, "b", &b, Some(ShapeEnum::Edge)) {
        let c = mkcurve(&e);
        s.set("c", &Restored::Curve(c.clone()));
        let dump = dump_curve("c", &c);
        log::result(&dump);
        let [t1, t2] = numbers_after(&dump, "Parameters : ", 2, "")
            .map(|v| [v[0], v[1]])
            .expect("Parameters : t1 t2");
        let (ts1, ts2) = validrange(&e).expect("a valid range");
        log::result(&format!("{} {}", g17(ts1), g17(ts2)));
        let delta1 = ts1 - t1;
        let delta2 = t2 - ts2;
        if delta1 < 1.0e-7 || delta1 > toler || delta2 < 1.0e-7 || delta2 > toler {
            log::puts("Error: incorrect computation of the valid range");
        }
    }
}

/// The face of bug30269: two edges lack their pcurves.
const BUG30269_SHAPE: &str = "
DBRep_DrawableShape

CASCADE Topology V1, (c) Matra-Datavision
Locations 0
Curve2ds 0
Curves 2
1 8111.3 2939 9014.2 0.23 5.2e-07 0.97
1 9996.2 2940 8568.4 -0.229 3.64e-07 -0.97
Polygon3D 0
PolygonOnTriangulations 0
Surfaces 2
1 9056.7 2940 8807.3 1.29e-07 -1 5e-07 -1.26e-29 -5.03e-07 -1 1 1.288e-07 -6.48e-14
1 10000 1519 8584.45 -0.97 -8.47e-08 0.23 0.23 0 0.97 -8.24e-08 1 1.95e-08
Triangulations 0

TShapes 10
Ve
1e-07
8111.30 2940 9014.2
0 0

0101101
*
Ve
1e-07
9996.21 2940 8568.4
0 0

0101101
*
Ed
 2.22e-16 0 0 0
0

0101000
+10 0 -9 0 *
Ve
1e-07
8115 2940 9030
0 0

0101101
*
Ed
 1e-07 0 0 0
1  1 0 0 16.5
0

0101000
+10 0 -7 0 *
Ve
1e-07
10000 2940 8584.5
0 0

0101101
*
Ed
 2.22e-16 0 0 0
0

0101000
+7 0 -5 0 *
Ed
 1e-07 0 0 0
1  2 0 -16.5 0
4 C0 1 0 2 0
0

0101000
+5 0 -9 0 *
Wi

0101000
-8 0 +6 0 +4 0 +3 0 *
Fa
0  1e-07 1 0

0101000
-2 0 *

-1 4
";

/// `set aFile [open $path wb]; puts $aFile $data; close $aFile; restore $path name;
/// file delete $path`.
fn restore_written(s: &mut dyn Session, data: &str, case: &str, name: &str) -> Shape {
    let path = imagedir().join(format!("{case}.brep"));
    std::fs::write(&path, format!("{data}\n")).unwrap();
    let shape = brestore(&path);
    s.set(name, &Restored::Shape(shape.clone()));
    log::result(name);
    std::fs::remove_file(&path).unwrap();
    shape
}

/// 0030269: the bounding boxes and properties of a face with missing pcurves are
/// computed without a Standard_NullObject.
pub fn bug30269(s: &mut dyn Session) {
    log::puts("========");
    log::puts(
        "0030269: Modeling Algorithms - unhandled Standard_NullObject within BRepBndLib::Add()",
    );
    log::puts("========");
    log::puts("");
    let f = restore_written(s, BUG30269_SHAPE, "bug30269", "f");
    bounding(s, Bounding::of(&f));
    bounding(
        s,
        Bounding {
            optimal: true,
            ..Bounding::of(&f)
        },
    );
    bounding(
        s,
        Bounding {
            obb: true,
            ..Bounding::of(&f)
        },
    );
    props_command(s, Prop::Area, &f, None, false);
    props_command(s, Prop::Volume, &f, None, false);
}

fn history_has(text: &str, deleted: usize, generated: usize) -> bool {
    text.contains(&format!("{deleted} Deleted")) && text.contains(&format!("{generated} Generated"))
}

fn banner_30346() {
    log::puts("========");
    log::puts(
        "0030346: Modeling Algorithms - BRepPrimAPI_MakeRevol throws BRepSweep_Translation::MakeEmptyVertex",
    );
    log::puts("========");
    log::puts("");
}

/// 0030346: the histories of a 60-degree and a full revolution of a planar face.
pub fn bug30346_1(s: &mut dyn Session) {
    banner_30346();
    let plane = plane_normal([0., 0., 0.], [0., 1., 0.]);
    s.set("f", &Restored::Surface(plane.clone()));
    let f = mkface_bounds(&plane, [0., 1., 0., 1.]);
    let f = shape_set(s, "f", f);
    let r1 = revol(&f, [0., 0., 0.], [0., 0., 1.], 60., false).expect("revol r1");
    shape_set(s, "r1", r1.shape);
    let s1 = dump_history("h1", &r1.history);
    if !history_has(&s1, 0, 6) {
        log::puts("Error: wrong history h1");
    }
    let r2 = revol(&f, [0., 0., 0.], [0., 0., 1.], 360., false).expect("revol r2");
    shape_set(s, "r2", r2.shape);
    let s2 = dump_history("h2", &r2.history);
    if !history_has(&s2, 6, 3) {
        log::puts("Error: wrong history h2");
    }
}

/// 0030346: the histories of the prisms of a full and a trimmed spherical band.
#[allow(clippy::approx_constant)] // The script's rounded latitudes.
pub fn bug30346_2(s: &mut dyn Session) {
    banner_30346();
    let surface = sphere(1.);
    s.set("s", &Restored::Surface(surface.clone()));
    let f1 = mkface_bounds(
        &surface,
        [
            0.,
            2. * std::f64::consts::PI,
            0.785398163397448,
            1.5707963267949,
        ],
    );
    let f1 = shape_set(s, "f1", f1);
    let r1 = prism(&f1, [0., 0., 2.], PrismFlag::None).unwrap();
    shape_set(s, "r1", r1.shape);
    let s1 = dump_history("h1", &r1.history);
    if !history_has(&s1, 0, 3) {
        log::puts("Error: wrong history h1");
    }
    let f2 = mkface_bounds(&surface, [0., 5., 0.785398163397448, 1.5707963267949]);
    let f2 = shape_set(s, "f2", f2);
    let r2 = prism(&f2, [0., 0., 2.], PrismFlag::None).unwrap();
    shape_set(s, "r2", r2.shape);
    let s2 = dump_history("h2", &r2.history);
    if !history_has(&s2, 0, 7) {
        log::puts("Error: wrong history h2");
    }
}

/// 0030433: the revolution of an elliptic face crossing the axis is not built.
pub fn bug30433(s: &mut dyn Session) {
    log::puts("========");
    log::puts("0030433: Checkshape: bad solid, created by revolution, is reported as valid.");
    log::puts("========");
    log::puts("");
    let e1 = ellipse([0., 0., 0.], 200., 100.);
    s.set("e1", &Restored::Curve(e1.clone()));
    let ell1 = shape_set(s, "ell1", mkedge(&e1));
    let w1 = shape_set(s, "w1", wire(&[&ell1]).unwrap());
    let p1 = plane_default();
    s.set("p1", &Restored::Surface(p1.clone()));
    let fe1 = shape_set(s, "fe1", mkface_wire(&p1, &w1));
    let rev1 = revol(&fe1, [0., 0., 0.], [1., 0., 0.], 180., true);
    if let Some(rev1) = &rev1 {
        shape_set(s, "rev1", rev1.shape.clone());
    }
    if rev1.is_none() {
        log::puts("OK: wrong shape is not created");
    } else {
        log::puts("Error: algorithm created wrong shape");
    }
}

/// 0030949: the JSON dump of the OBB of a vertex holds its centre.
pub fn bug30949(s: &mut dyn Session) {
    log::puts("=============");
    log::puts("0030949: Foundation Classes - Dump improvement for OCCT classes");
    log::puts("=============");
    let (px, py, pz) = (1., 2., 3.);
    let vv = shape_set(s, "vv", vertex(Point3::new(px, py, pz)));
    let log_text = bounding(
        s,
        Bounding {
            obb: true,
            dump_json: true,
            ..Bounding::of(&vv)
        },
    )
    .text;
    let Some([xc, yc, zc]) =
        numbers_after(&log_text, "\"gp_XYZ\": [", 3, ",").map(|v| [v[0], v[1], v[2]])
    else {
        log::puts("Error in DumpJson.");
        panic!("can't read \"xc\": no such variable");
    };
    checkreal("XC", xc, px, 1.0e-7, 0.0);
    checkreal("YC", yc, py, 1.0e-7, 0.0);
    checkreal("ZC", zc, pz, 1.0e-7, 0.0);
}

/// 0031031: the prism of a vertex generates one shape from it.
pub fn bug31031(s: &mut dyn Session) {
    log::puts("========");
    log::puts("0031031: Incorrect result is returned from BRepPrimAPI_MakePrism::Generated()");
    log::puts("========");
    log::puts("");
    let v1 = shape_set(s, "v1", vertex(Point3::new(0., 0., 0.)));
    let r1 = prism(&v1, [0., 0., 2.], PrismFlag::None).unwrap();
    shape_set(s, "r1", r1.shape);
    let s1 = dump_history("h1", &r1.history);
    if !history_has(&s1, 0, 1) {
        log::puts("Error: wrong history h1");
    }
}

/// The shell of optimal_bndbox_exception: a B-spline face of tolerance 4.
const OPTIMAL_BNDBOX_SHAPE: &str = "
DBRep_DrawableShape

CASCADE Topology V1, (c) Matra-Datavision
Locations 0
Curve2ds 4
1 -1.3333333333333335 0 0 1
1 0 -1.6666666666666667 1 0
1 1.6666666666666665 0 0 1
1 0 1.3333333333333333 1 0
Curves 0
Polygon3D 0
PolygonOnTriangulations 0
Surfaces 1
9 0 0 0 0 3 3 4 4 2 2 0 3 1.9999999999999998  0 1.6666666666666665 1.9999999999999998  0 0.3333333333333332 1.9999999999999996  0 -0.99999999999999989 2
0 2.9999999999999991 0.66666666666666718  0 1.6666666666666659 0.66666666666666741  0 0.3333333333333332 0.66666666666666663  0 -0.99999999999999956 0.66666666666666663
0 3.0000000000000009 -0.66666666666666718  0 1.6666666666666676 -0.66666666666666752  0 0.33333333333333331 -0.66666666666666652  0 -1.0000000000000002 -0.66666666666666685
0 3.0000000000000004 -2  0 1.6666666666666667 -2  0 0.33333333333333326 -1.9999999999999996  0 -1 -2

-1.3333333333333335 4
2.6666666666666665 4

-2.666666666666667 4
1.3333333333333333 4

Triangulations 0

TShapes 8
Ve
4.000004
0 -1 -1
0 0

0101101
*
Ed
 4.000004 1 1 1
2  1 1 0 -1.66666666666667 1.33333333333333
0

0101000
+8 0 -8 0 *
Ed
 4.000004 1 1 1
2  2 1 0 -1.33333333333333 1.66666666666667
0

0101000
+8 0 -8 0 *
Ed
 4.000004 1 1 1
2  3 1 0 -1.66666666666667 1.33333333333333
0

0101000
+8 0 -8 0 *
Ed
 4.000004 1 1 1
2  4 1 0 -1.33333333333333 1.66666666666667
0

0101000
+8 0 -8 0 *
Wi

0101100
-7 0 +6 0 +5 0 -4 0 *
Fa
0  1e-07 1 0

0101000
+3 0 *
Sh

1101000
+2 0 *

+1 0
";

/// 0032236: the optimal bounding box of a high-tolerance B-spline shell is computed
/// without an exception.
pub fn optimal_bndbox_exception(s: &mut dyn Session) {
    log::puts("========");
    log::puts(
        "0032236: Modeling Algorithms - E x c e p t i o n on calculation optimal bounding box",
    );
    log::puts("========");
    log::puts("");
    let path = imagedir().join("optimal_bndbox_exception.brep");
    std::fs::write(&path, format!("{OPTIMAL_BNDBOX_SHAPE}\n")).unwrap();
    let shape = brestore(&path);
    s.set("aShape", &Restored::Shape(shape.clone()));
    log::result("aShape");
    bounding(
        s,
        Bounding {
            optimal: true,
            ..Bounding::of(&shape)
        },
    );
    std::fs::remove_file(&path).unwrap();
}
