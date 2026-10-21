//! `bugs_modalg_6` case bodies whose
//! inputs are in the public dataset (bug27884 builds its circle in the script).
use crate::support::{
    checkshape::checkshape,
    commands::{explode, explode_type, mksurface},
    distance::{b2dclassify, bclassify_default, distmini},
    log,
    logged_checks::{checknbshapes_some, checkreal},
    props::{Prop, checkprops},
    qa::{occ26525, occ27884},
    restore::restore_as,
    session::{Session, ViewStep, checkview_display, set, set_exploded},
    text::numbers_after,
    topology::{mkface, mkpoint, tcopy, vertex, wire},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCurve, GeomRectangularTrimmedSurface, GeomSurface},
    top_abs::{ShapeEnum, State},
};
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::{
    convert::ParameterisationType,
    gp::{Axis2, Direction3, Point2, Point3},
};

/// Checkshape must not raise.
pub fn bug21246(s: &mut dyn Session) {
    log::required("Faulty shapes in variables faulty_1 to faulty_2");
    let result = restore_as(s, "bug21246.brep", "result").shape();
    let info = checkshape(&result).text;
    if !info.contains("Faulty shapes in variables faulty_1 to faulty_2") {
        log::puts("OK : Good result of checkshape");
    } else if info.contains("Problems are not detected") {
        log::puts("Error : Wrong result of checkshape");
    }
    checkview_display(s, Some("result"), "bug21246");
}

/// Distmini of the two edges must not lock up.
pub fn bug26064(s: &mut dyn Session) {
    let s1 = restore_as(s, "dist1-s1.brep", "s1").shape();
    let s2 = restore_as(s, "dist1-s2.brep", "s2").shape();
    distmini(s, "d", &s1, &s2, None);
}

/// The curve of an edge of b2 against the face b1 (untrimmed) or b1t (its
/// surface trimmed in V to [-30000, 30000]); two points with the expected coordinates.
fn bug26525(s: &mut dyn Session, edge_index: usize, trimmed: bool, expected: [[f64; 3]; 2]) {
    let b1 = restore_as(s, "bug26525_a.brep", "b1").shape();
    let b2 = restore_as(s, "bug26525_b.brep", "b2").shape();
    let sb1 = set(s, "sb1", mksurface(&b1));
    let sb1t =
        GeomRectangularTrimmedSurface::with_one_direction(&sb1, -30000., 30000., false, true);
    let sb1t: Handle<dyn GeomSurface> = Handle::new(sb1t.unwrap()).cast();
    let sb1t = set(s, "sb1t", sb1t);
    let b1t = set(s, "b1t", mkface(&sb1t));
    let edges = set_exploded(s, "b2", explode_type(&b2, ShapeEnum::Edge));
    let (r, face) = if trimmed { ("pt", &b1t) } else { ("p", &b1) };
    let log_text = occ26525(r, &edges[edge_index - 1], face);
    let number = numbers_after(&log_text, "found: ", 1, "").map(|v| v[0]);
    if number != Some(2.0) {
        log::puts("Error: Bad Number of intersection points");
        return;
    }
    for (i, expected) in expected.iter().enumerate() {
        let point = numbers_after(&log_text, &format!("point {r}_{} ", i + 1), 3, "").unwrap();
        for (axis, name) in ["x", "y", "z"].iter().enumerate() {
            let label = format!("{name}{}", i + 1);
            checkreal(&label, point[axis], expected[axis], 0.0001, 0.01);
        }
    }
}

const B2_3: [[f64; 3]; 2] = [[48.4205, -22.5336, 82.7431], [32.5621, -5.89907, 82.7431]];

pub fn bug26525_1(s: &mut dyn Session) {
    bug26525(s, 3, false, B2_3);
}

pub fn bug26525_2(s: &mut dyn Session) {
    bug26525(s, 3, true, B2_3);
}

pub fn bug26525_3(s: &mut dyn Session) {
    let expected = [[43.1172, -17.16, 82.6048], [39.0504, -12.8696, 82.6099]];
    bug26525(s, 1, false, expected);
}

pub fn bug26525_4(s: &mut dyn Session) {
    let expected = [[39.0504, -12.8696, 82.6099], [43.1172, -17.16, 82.6048]];
    bug26525(s, 1, true, expected);
}

/// The restored point is not OUT of the solid; smallview, fit, screenshot.
pub fn bug26609(s: &mut dyn Session) {
    let p = restore_as(s, "bug26609_p.draw", "p").point();
    let z = restore_as(s, "bug26609_z.brep", "z").shape();
    if bclassify_default(&z, p) == State::Out {
        log::puts("Error : Wrong result obtained by solid classifier algorithm");
    }
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug26609"));
}

/// The classifier takes the tolerances into account: the point is not IN.
pub fn bug27117(s: &mut dyn Session) {
    let solid = restore_as(s, "bug27117.brep", "s").shape();
    let p = set(
        s,
        "p",
        Point3::new(103.740000000001, 1.85e-014, 24.3498644581774),
    );
    if bclassify_default(&solid, p) == State::In {
        log::puts("Error : Wrong result obtained by solid classifier algorithm");
    }
}

/// The restored box and its counts, area and checkshape.
fn bug27264(s: &mut dyn Session, file: &str, counts: [usize; 3], area: f64) {
    let a = restore_as(s, file, "a").shape();
    let [v, e, f] = counts;
    checknbshapes_some(
        &a,
        [
            Some(v),
            Some(e),
            None,
            Some(f),
            None,
            None,
            None,
            None,
            None,
        ],
    );
    checkprops(&a, Prop::Area, area);
    checkshape(&a);
}

pub fn bug27264_1(s: &mut dyn Session) {
    bug27264(s, "bug27264_1.brep", [8, 12, 6], 6.0);
}

pub fn bug27264_2(s: &mut dyn Session) {
    bug27264(s, "bug27264_2.brep", [2, 3, 1], 1256.64);
}

/// A wire of a curve edge and a degenerated edge; its counts; the screenshot.
pub fn bug27568(s: &mut dyn Session) {
    let c = restore_as(s, "bug27568_curves.brep", "c").shape();
    let parts = set_exploded(s, "c", explode(&c));
    match wire(&[&parts[0], &parts[1]]) {
        Ok(result) => {
            let result = set(s, "result", result);
            let some = [
                Some(2),
                Some(2),
                Some(1),
                None,
                None,
                None,
                None,
                None,
                None,
            ];
            checknbshapes_some(&result, some);
        }
        Err(message) => log::result(&message),
    }
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug27568"));
}

/// The classification must not raise ConstructionError.
pub fn bug27802(s: &mut dyn Session) {
    let a = restore_as(s, "bug27802_cls_error.brep", "a").shape();
    let parts = set_exploded(s, "a", explode(&a));
    let p = set(s, "p", mkpoint(&parts[1]));
    bclassify_default(&parts[0], p);
}

/// The 2D classifier with boxes must be faster than without by the given
/// percentages (over the circle and its B-spline conversion).
pub fn bug27884(_s: &mut dyn Session) {
    let circle = GeomCircle::new(
        Axis2::new(
            Point3::new(1., 0., 0.),
            Direction3::new(0., 0., 1.).unwrap(),
        )
        .unwrap(),
        150.,
    )
    .unwrap();
    let c: Handle<dyn GeomCurve> = Handle::new(circle).cast();
    let checks = [(400.0, 0.0, 4.0), (250.0, 1.0, 3.0)];
    for (points, check, limit) in checks {
        if occ27884(&c, points, check) <= limit {
            log::puts("Error: algorithm slowed down");
        }
    }
    let c1 = geom_convert::curve_to_bspline_curve(&c, ParameterisationType::TgtThetaOver2).unwrap();
    let c1: Handle<dyn GeomCurve> = c1.cast();
    for (points, check, limit) in [(350.0, 2.0, 15.0), (250.0, 1.0, 15.0)] {
        if occ27884(&c1, points, check) <= limit {
            log::puts("Error: algorithm slowed down");
        }
    }
}

/// D0, D1 and D2 of the trimmed offset surface give the same point and D1/D2
/// the same first derivatives (1e-14); the view with f1, v1 and v2.
pub fn bug27929(s: &mut dyn Session) {
    const TOLER: f64 = 1.0e-14;
    let f1 = restore_as(s, "bug27929_f1.brep", "f1").shape();
    let os1 = set(s, "os1", mksurface(&f1));
    let surface = os1.get().unwrap();
    let p0 = surface.d0(0.5, 0.5).unwrap();
    let d1 = surface.d1(0.5, 0.5).unwrap();
    let v1 = set(s, "v1", vertex(p0));
    let v2 = set(s, "v2", vertex(d1.point));
    if distmini(s, "di12", &v1, &v2, None).unwrap().value > TOLER {
        log::puts("Error: Methods D0 and D1 return different result.");
    }
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Display(&["f1", "v1", "v2"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug27929"));
    let d2 = surface.d2(0.5, 0.5).unwrap();
    let v3 = set(s, "v3", vertex(d2.point));
    if distmini(s, "di13", &v1, &v3, None).unwrap().value > TOLER {
        log::puts("Error: Methods D0 and D2 return different result.");
    }
    let pairs = [
        (d1.d1u.x(), d2.d1u.x()),
        (d1.d1u.y(), d2.d1u.y()),
        (d1.d1u.z(), d2.d1u.z()),
        (d1.d1v.x(), d2.d1v.x()),
        (d1.d1v.y(), d2.d1v.y()),
        (d1.d1v.z(), d2.d1v.z()),
    ];
    for (a, b) in pairs {
        if (a - b).abs() > TOLER {
            log::puts("Error: Methods D1 and D2 return different result.");
        }
    }
}

/// Distmini with a face that has no surface must not raise.
pub fn bug27992(s: &mut dyn Session) {
    let a = restore_as(s, "bug27821_nullsurf.brep", "a").shape();
    let v = set(s, "v", vertex(Point3::new(0., 0., 0.)));
    distmini(s, "d", &v, &a, None);
    let b = set(s, "b", tcopy(&a));
    distmini(s, "d", &a, &b, None);
}

/// Both inner points are IN the face.
pub fn bug31047(s: &mut dyn Session) {
    let f = restore_as(s, "bug31047.brep", "f").shape();
    let p1 = set(s, "p1", Point2::new(-79., -282.7));
    let p2 = set(s, "p2", Point2::new(-79., -282.6));
    if b2dclassify(&f, p1) != State::In {
        log::puts("Error: inner point p1 is classified as OUT");
    }
    if b2dclassify(&f, p2) != State::In {
        log::puts("Error: inner point p2 is classified as OUT");
    }
}
