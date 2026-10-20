//! `bugs_moddata_3` case bodies (the inputs of bug25631, bug26560 and bug31995 are
//! public dataset members).
use crate::support::{
    bounding::{Bounding, bounding},
    checkshape_structural::checkshape_structural,
    curves::{circle, circle_normal, explode, plane_normal, shape_set, trim_surface},
    distance::distmini,
    log,
    logged_checks::checkreal,
    misc::{checkloc, compare, located_scale, qa_nullify_shape},
    prim::{PrismFlag, box_sizes, pcylinder, prism, revol, whatis},
    props_view::{Expected, checkprops},
    restore::{Restored, restore_as},
    session::{Session, ViewStep, checkview_display},
    text::{last_number, llength, numbers_after},
    topology::{dump_shape, maxtolerance_text, mkedge, mkface, ttranslate, wire},
};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The distance between a circle and the lateral face of a cylinder is 0 at
/// (0, 10, 5).
pub fn bug23863(s: &mut dyn Session) {
    log::puts("================");
    log::puts("OCC23863");
    log::puts("================");
    log::puts("");
    let b1 = shape_set(s, "b1", pcylinder(10., 10.));
    let faces = explode(s, "b1", &b1, Some(ShapeEnum::Face));
    let b1 = shape_set(s, "b1", faces[0].clone());
    let b2 = circle_normal([0., 10., 10.], [1., 0., 0.], 5.);
    s.set("b2", &Restored::Curve(b2.clone()));
    let b2 = shape_set(s, "b2", mkedge(&b2));
    let d = distmini(s, "d", &b1, &b2, None).expect("distmini d b1 b2");
    let d_val = format!(
        "\n\n*********** Dump of d_val *************\n{}",
        format_general(d.value, 15)
    );
    log::result(&d_val);
    let dist = last_number(&d_val).expect("the distance");
    let tolerance = maxtolerance_text("d", &d.solutions[0]);
    let at = tolerance
        .find("Vertex")
        .expect("maxtolerance prints the vertices");
    let toler = numbers_after(&tolerance[at..], "Max ", 1, "").expect("Max")[0];
    let good_dist = 0.;
    if (dist - good_dist).abs() > toler {
        log::puts(&format!("Faulty : the distance is {dist}. It is bad value"));
    }
    let dump = dump_shape("d", &d.solutions[0]);
    log::result(&dump);
    let p = numbers_after(&dump, "Point 3D : ", 3, ",").expect("Point 3D : x, y, z");
    for (axis, value, good) in [("x", p[0], 0.), ("y", p[1], 10.), ("z", p[2], 5.)] {
        if (value - good).abs() > toler {
            log::puts(&format!(
                "Faulty : the {axis} coordinate of the point is {value}. It is bad value"
            ));
        }
    }
    checkview_display(s, None, "bug23863");
}

fn closed_flag(s: &mut dyn Session, result: &martensite_tkbrep::topods::Shape, image: &str) {
    let info = whatis("result", result);
    if !info.contains("Closed") {
        log::puts("OK : value of IsClosed flag is correct");
    } else {
        log::puts("Error : value of IsClosed flag is not correct");
    }
    checkview_display(s, Some("result"), image);
}

fn banner_25202() {
    log::puts("========");
    log::puts("CR25202");
    log::puts("========");
    log::puts("");
}

/// The IsClosed flag of the prism of a trimmed plane face.
pub fn bug25202_3(s: &mut dyn Session) {
    banner_25202();
    let pp = plane_normal([0., 0., 0.], [0., 0., 1.]);
    s.set("pp", &Restored::Surface(pp.clone()));
    let pp = trim_surface(&pp, [0., 100.], [0., 100.]);
    s.set("pp", &Restored::Surface(pp.clone()));
    let ff = shape_set(s, "ff", mkface(&pp));
    let result = prism(&ff, [0., 0., 20.], PrismFlag::None).unwrap().shape;
    let result = shape_set(s, "result", result);
    closed_flag(s, &result, "bug25202_3");
}

/// The IsClosed flag of the revolution of a circular face by 90 degrees.
pub fn bug25202_4(s: &mut dyn Session) {
    banner_25202();
    let cc = circle([0., 100., 0.], 20.);
    s.set("cc", &Restored::Curve(cc.clone()));
    let ee = shape_set(s, "ee", mkedge(&cc));
    let ww = shape_set(s, "ww", wire(&[&ee]).unwrap());
    let ff = shape_set(
        s,
        "ff",
        crate::support::curves::mkplane(&ww, false).unwrap(),
    );
    let result = revol(&ff, [0., 0., 0.], [1., 0., 0.], 90., false).expect("revol");
    let result = shape_set(s, "result", result.shape);
    closed_flag(s, &result, "bug25202_4");
}

/// `bounding result -save Xmin Ymin Zmin Xmax Ymax Zmax` then `checkreal` of each with
/// the script's tolerances.
fn check_box(saved: [f64; 6], good: [f64; 6], tol_abs: f64, tol_rel: f64) {
    let names = ["Xmin", "Ymin", "Zmin", "Xmax", "Ymax", "Zmax"];
    for ((name, value), good) in names.iter().zip(saved).zip(good) {
        checkreal(name, value, good, tol_abs, tol_rel);
    }
}

/// The bounding box of the restored face is not too big.
pub fn bug25631(s: &mut dyn Session) {
    log::puts("=========");
    log::puts("OCC25631");
    log::puts("=========");
    log::puts("");
    let result = restore_as(s, "bug25631_fbx.brep", "result").shape();
    let saved = bounding(
        s,
        Bounding {
            save: true,
            ..Bounding::of(&result)
        },
    )
    .saved
    .expect("bounding -save");
    check_box(
        saved,
        [
            107.49999989999992,
            -12.487307730633276,
            -12.135364833115547,
            140.20000009999987,
            -0.8336837432205401,
            -0.5631589698265231,
        ],
        1.0e-4,
        1.0e-4,
    );
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug25631"));
}

/// The bounding box of a planar spline edge is flat in Z.
pub fn bug26560(s: &mut dyn Session) {
    log::puts("=========");
    log::puts("OCC26560");
    log::puts("=========");
    log::puts("");
    let result = restore_as(s, "bug26560_planarspline.brep", "result").shape();
    let saved = bounding(
        s,
        Bounding {
            save: true,
            result_shape: Some("resbox"),
            ..Bounding::of(&result)
        },
    )
    .saved
    .expect("bounding -save");
    check_box(
        saved,
        [
            -277.03883383200952,
            -562.56241861670651,
            -9.9999999999999995e-08,
            20.000000100000001,
            -221.27317426638498,
            9.9999999999999995e-08,
        ],
        1.0e-4,
        0.0001,
    );
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Display(&["result", "resbox"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug26560"));
}

/// 0027457: `fscale` puts a scaled location on the box (checkshape and checkloc report
/// it, the volume is 8); `bscale` refuses it (the volume stays 1).
pub fn bug27457(s: &mut dyn Session) {
    log::puts("=========");
    log::puts("0027457: Modeling Data - Raise if scaled transformation is used for shape location");
    log::puts("=========");
    log::puts("");
    let b = shape_set(s, "b", box_sizes(1., 1., 1.));
    let b = shape_set(
        s,
        "b",
        located_scale(&b, [0., 0., 0.], 2., true).expect("fscale moves the shape"),
    );
    let chsh = checkshape_structural(&b, "b", "f", Default::default());
    let nb_sh = numbers_after(&chsh, "Shapes with problems : ", 1, "")
        .expect("Shapes with problems : N")[0];
    if nb_sh > 1. {
        log::puts(&format!("Number of faulty shapes : {nb_sh}"));
    } else {
        log::puts("Error: fscale does not produce wrong shape");
    }
    let chloc = checkloc(s, "b", &b);
    let nbloc = llength(&chloc);
    if nbloc > 0 {
        log::puts(&format!("Number of problematic locations : {nbloc}"));
    } else {
        log::puts("Error: fscale does not produce wrong location");
    }
    checkprops(s, "b", &b, Expected::v(8.));
    let b1 = shape_set(s, "b1", box_sizes(1., 1., 1.));
    if let Some(scaled) = located_scale(&b1, [0., 0., 0.], 2., false) {
        shape_set(s, "b1", scaled);
    }
    checkprops(s, "b1", &b1, Expected::v(1.));
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Donly(&["b", "b1"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug27457"));
}

/// 0029827: TopoDS_Shape::Nullify resets the location, so two nullified translated boxes
/// are the same and equal shapes.
pub fn bug29827(s: &mut dyn Session) {
    log::puts("==========");
    log::puts("0029827: Modeling Data - TopoDS_Shape::Nullify() does not reset location");
    log::puts("==========");
    log::puts("");
    let b1 = shape_set(s, "b1", crate::support::prim::box_sizes(1., 2., 1.));
    let b2 = shape_set(s, "b2", box_sizes(1., 1., 1.));
    let b1 = shape_set(s, "b1", ttranslate(&b1, [0., 1., 1.], false));
    let b2 = shape_set(s, "b2", ttranslate(&b2, [1., 1., 1.], false));
    let b1 = qa_nullify_shape(b1);
    let b2 = qa_nullify_shape(b2);
    let result = compare(&b1, &b2);
    for data in ["same shapes", "equal shapes"] {
        if !result.contains(data) {
            log::puts(&format!("Error: Shapes are not '{data}'"));
            break;
        }
    }
}

/// 0031995: the bounding box of the restored face is right.
pub fn bug31995(s: &mut dyn Session) {
    log::puts("=========");
    log::puts("0031995: Modeling Data - Bounding box wrong for face");
    log::puts("=========");
    log::puts("");
    let result = restore_as(s, "bug31995.brep", "result").shape();
    let saved = bounding(
        s,
        Bounding {
            save: true,
            ..Bounding::of(&result)
        },
    )
    .saved
    .expect("bounding -save");
    check_box(
        saved,
        [
            -0.4992512824357961,
            0.5646627848871493,
            0.5646627848871487,
            0.1535534905932737,
            1.4505295939775213,
            1.4505295939775207,
        ],
        1.0e-4,
        1.0e-4,
    );
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug31995"));
}
