//! `bugs_moddata_2`: checkshape stability, projections and distances.
use super::face_surface;
use crate::support::{
    checkshape::checkshape,
    checkshape_structural::checkshape_structural,
    commands::{dump_curve, explode_type, mkcurve},
    distance::distmini,
    geometry::{ProjSolution, cvalue, dump_point, proj_surface},
    log,
    logged_checks::checkreal,
    printed::g17,
    restore::restore_as,
    session::{Session, ViewStep, checkview_display, set, set_exploded},
    text::{labeled_numbers, last_number, lindex},
    topology::{maxtolerance_text, mkface, vertex},
};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

/// `dump d_val` of a case harness number: the banner and the value (`std::cout` precision 15).
fn dump_number(name: &str, value: f64) -> String {
    let text = format!(
        "\n\n*********** Dump of {name} *************\n{}",
        format_general(value, 15)
    );
    log::result(&text);
    text
}

/// 25 restores of the face give the same structural checkshape text.
fn bug441(s: &mut dyn Session, file: &str, image: &str) {
    let result = restore_as(s, file, "result").shape();
    let err = checkshape_structural(&result, "result", "r", Default::default());
    for j in 1..=25 {
        let result = restore_as(s, file, "result").shape();
        let err1 = checkshape_structural(&result, "result", "r", Default::default());
        if err != err1 {
            log::puts(&format!(
                " Faulty OCC441: function CHECKSHAPE works wrongly on {j} time"
            ));
            break;
        }
        log::puts(&format!(
            " OCC441  OK: function CHECKSHAPE works properly {j} time"
        ));
    }
    checkview_display(s, Some("result"), image);
}

pub fn bug441_1(s: &mut dyn Session) {
    bug441(s, "OCC441a.brep", "bug441_1");
}

pub fn bug441_2(s: &mut dyn Session) {
    bug441(s, "OCC441b.brep", "bug441_2");
}

pub fn bug441_3(s: &mut dyn Session) {
    bug441(s, "OCC441c.brep", "bug441_3");
}

/// `bug935` and `bug943` (REQUIRED faulty_1 to faulty_): 25 restores give the same contextual
/// checkshape text.
fn stable_checkshape(s: &mut dyn Session, file: &str, label: &str, image: &str) {
    log::required("Faulty shapes in variables faulty_1 to faulty_");
    let result = restore_as(s, file, "result").shape();
    let err = checkshape(&result).text;
    for j in 1..=25 {
        let result = restore_as(s, file, "result").shape();
        let err1 = checkshape(&result).text;
        if err != err1 {
            log::puts(&format!(
                " Faulty {label}: function CHECKSHAPE works wrongly on {j} time"
            ));
            break;
        }
        log::puts(&format!(
            " {label}  OK: function CHECKSHAPE works properly {j} time"
        ));
    }
    checkview_display(s, Some("result"), image);
}

pub fn bug935_1(s: &mut dyn Session) {
    stable_checkshape(s, "OCC935_1.brep", "OCC935 (case 1)", "bug935_1");
}

pub fn bug935_2(s: &mut dyn Session) {
    stable_checkshape(s, "OCC935_2.brep", "OCC935 (case 2)", "bug935_2");
}

pub fn bug943_1(s: &mut dyn Session) {
    stable_checkshape(s, "OCC943_1.brep", "OCC943 (case 1)", "bug943_1");
}

/// The structural checkshape finds the self-intersecting wire.
pub fn bug736(s: &mut dyn Session) {
    let result = restore_as(s, "OCC736.brep", "result").shape();
    if checkshape_structural(&result, "result", "r", Default::default()).contains("OK") {
        log::puts("Faulty OCC736:  checkshape command works wrongly");
    }
    checkview_display(s, Some("result"), "bug736");
}

/// The four projections of (0, 10, 50) on the face's surface, their dumped
/// parameters, origins and axes (0.1%).
pub fn bug2755(s: &mut dyn Session) {
    let a = restore_as(s, "OCC2755.brep", "a").shape();
    checkshape(&a);
    let faces = set_exploded(s, "a", explode_type(&a, ShapeEnum::Face));
    let gs = face_surface(s, &faces[0], "gs");
    let solutions = proj_surface(s, &gs, Point3::new(0., 10., 50.)).unwrap_or_default();
    let segments: Vec<_> = solutions
        .iter()
        .filter_map(|solution| match solution {
            ProjSolution::Segment(segment) => Some(segment.clone()),
            ProjSolution::Point(_) => None,
        })
        .collect();
    if solutions.len() != 4 {
        log::puts("Faulty OCC2755");
        return;
    }
    let good_parameter2 = [
        40.4616826479819,
        211.944716681974,
        64.3186164305428,
        81.0582330575794,
    ];
    let good_axis_x = [
        -0.0300865818651084,
        -0.941236271764613,
        0.534306660761684,
        0.998573938898137,
    ];
    let good_axis_y = [
        0.999547296325529,
        -0.337748842654791,
        -0.845290714645381,
        0.0533862206328518,
    ];
    for (i, segment) in segments.iter().enumerate() {
        let dump_list = dump_curve(&format!("ext_{}", i + 1), segment);
        log::result(&dump_list);
        let parameters = labeled_numbers(&dump_list, "Parameters", 2, "").unwrap();
        let origin = labeled_numbers(&dump_list, "Origin", 3, ",").unwrap();
        let axis = labeled_numbers(&dump_list, "Axis", 3, ",").unwrap();
        let checks = [
            ("Parameter1", parameters[0], 0.0),
            ("Parameter2", parameters[1], good_parameter2[i]),
            ("OriginX", origin[0], 0.0),
            ("OriginY", origin[1], 10.0),
            ("OriginZ", origin[2], 50.0),
            ("AxisX", axis[0], good_axis_x[i]),
            ("AxisY", axis[1], good_axis_y[i]),
            ("AxisZ", axis[2], 0.0),
        ];
        for (name, value, good) in checks {
            checkreal(&format!("{name} / case 3 / {i}"), value, good, 0.0, 0.001);
        }
    }
}

/// Distmini between the trimmed line and the circle must not raise.
pub fn bug324_1(s: &mut dyn Session) {
    let el = restore_as(s, "OCC359a.brep", "el").shape();
    let ec = restore_as(s, "OCC359b.brep", "ec").shape();
    distmini(s, "dd", &el, &ec, None);
}

/// Distmini must not raise.
pub fn bug747(s: &mut dyn Session) {
    let s1 = restore_as(s, "OCC747_1.brep", "s1").shape();
    checkshape(&s1);
    let s2 = restore_as(s, "OCC747_2.brep", "s2").shape();
    checkshape(&s2);
    let done = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        distmini(
            &mut crate::support::session::NoView,
            "result",
            &s1,
            &s2,
            None,
        )
    }));
    match done {
        Ok(_) => log::puts("OCC747 OK"),
        Err(_) => log::puts("Faulty OCC747"),
    }
}

/// The offset surface's points at the V bounds and 1e-11 beyond them agree
/// within 1e-7.
pub fn bug498(s: &mut dyn Session) {
    let su = restore_as(s, "OCC498.draw", "su").surface();
    let (u1, u2) = (4.71238898038469, 7.85398163397449);
    let surface = su.get().unwrap();
    let point = |v: f64| surface.d0(0.0, v).unwrap();
    let v1 = set(s, "v1", vertex(point(u1)));
    let v2 = set(s, "v2", vertex(point(u2)));
    let vv1 = set(s, "vv1", vertex(point(u1 - 1e-11)));
    let vv2 = set(s, "vv2", vertex(point(u2 + 1e-11)));
    let d1 = distmini(s, "d", &v1, &vv1, None).unwrap().value;
    let err1 = last_number(&dump_number("d_val", d1));
    let d2 = distmini(s, "d", &v2, &vv2, None).unwrap().value;
    let err2 = last_number(&dump_number("d_val", d2));
    let (err1, err2) = (err1.unwrap(), err2.unwrap());
    if err1 > 1.0e-7 || err2 > 1.0e-7 {
        log::puts("Faulty OCC498: Wrong 3d point from offset surface by parameters ");
    }
}

/// Checkshape does not detect the edge lying off the surface.
pub fn bug592(s: &mut dyn Session) {
    let faulty = "Faulty OCC592 : Checkshape BRepCheck_Analyzer does not detect faulty shape";
    log::todo(faulty);
    let ff114 = restore_as(s, "OCC592.brep", "ff114").shape();
    let edges = set_exploded(s, "ff114", explode_type(&ff114, ShapeEnum::Edge));
    let cc = set(s, "cc", mkcurve(&edges[7]));
    let vv = set(s, "vv", vertex(cvalue(&cc, 0.95)));
    let gs = face_surface(s, &ff114, "gs");
    let ff = set(s, "ff", mkface(&gs));
    s.step(ViewStep::Donly(&["ff114", "vv"]));
    let trcc = crate::support::commands::trim_to_basis(&cc);
    set(s, "trcc", trcc);
    let dd = distmini(s, "dd", &vv, &ff, None).unwrap().value;
    let dis1 = last_number(&dump_number("dd_val", dd)).unwrap();
    log::puts(&format!(
        "distance between the point on curve and the surface is {}",
        g17(dis1)
    ));
    maxtolerance_text("ff114_8", &edges[7]);
    if !checkshape(&ff114).has_faulty() {
        log::puts(faulty);
    }
    checkview_display(s, None, "bug592");
}

/// The `lindex [dump ext_1] 9` and `10` words of a projection solution's dump.
fn dump_words(solution: &ProjSolution) -> (String, String) {
    let text = match solution {
        ProjSolution::Segment(segment) => dump_curve("ext_1", segment),
        ProjSolution::Point(point) => dump_point("ext_1", *point),
    };
    log::result(&text);
    (lindex(&text, 9).to_string(), lindex(&text, 10).to_string())
}

/// `expr abs ([expr $p2 - $p1])`: `expr` joins its words before parsing, so an empty
/// `$p2` (the dump of a point has no word 10) leaves `- $p1`, a unary minus; any other
/// non-numeric word is a script error.
fn word_difference(p1: &str, p2: &str) -> f64 {
    let number = |word: &str| -> f64 {
        word.parse()
            .unwrap_or_else(|_| panic!("can't use non-numeric string {word:?} as operand of \"-\""))
    };
    let p1 = number(p1);
    if p2.is_empty() {
        return (-p1).abs();
    }
    (number(p2) - p1).abs()
}

/// `bug593` (TODO: the projection error exceeds the face tolerance).
pub fn bug593(s: &mut dyn Session) {
    let error = "Error : Projection of a 3D point on surface using GeomAPI_ProjectPointOnSurf works incorrect";
    log::todo(error);
    let ff48 = restore_as(s, "OCC593.brep", "ff48").shape();
    let tolerance = maxtolerance_text("ff48", &ff48);
    let max_face_tolerance =
        crate::support::text::numbers_after(&tolerance, "Max  ", 1, "").unwrap()[0];
    let p = Point3::new(-19.561252535222, 17.89876466186, 71.408126285268);
    let vv = set(s, "vv", vertex(p));
    let dd = distmini(s, "dd", &vv, &ff48, None).unwrap().value;
    last_number(&dump_number("dd_val", dd));
    let gs = face_surface(s, &ff48, "gs");
    for (case, point) in [(1, p), (2, gs.get().unwrap().d0(0.56, 0.4).unwrap())] {
        if case == 2 {
            log::puts("***** Another example: *****");
            set(s, "p", point);
        }
        let solutions = proj_surface(s, &gs, point).unwrap();
        let (pp1, pp2) = dump_words(&solutions[0]);
        let err = word_difference(&pp1, &pp2);
        log::puts(&format!("MaxFaceTolerance = {}", g17(max_face_tolerance)));
        log::puts(&format!("MaxDistance = {}", g17(err)));
        if err < max_face_tolerance {
            log::puts(&format!(
                "OCC593 (case {case}) : Projection of a 3D point on surface using GeomAPI_ProjectPointOnSurf works properly"
            ));
        } else {
            log::puts(error);
        }
    }
}

/// The projection's end point is within 1.108e-05 of the point.
pub fn bug943_2(s: &mut dyn Session) {
    let su = restore_as(s, "OCC943_2.draw", "su").surface();
    let p = Point3::new(5.759527537723098, 31.81368637070151, -0.2989616405627145);
    let v = set(s, "v", vertex(p));
    let solutions = proj_surface(s, &su, p).unwrap();
    let ProjSolution::Segment(ext_1) = &solutions[0] else {
        panic!("dump ext_1 of a point has no `Parameters` (regexp fails)");
    };
    let dump = dump_curve("ext_1", ext_1);
    log::result(&dump);
    let res_v = labeled_numbers(&dump, "Parameters", 2, "").unwrap()[1];
    let end = set(s, "V", vertex(cvalue(ext_1, res_v)));
    let d = distmini(s, "d", &end, &v, None).unwrap().value;
    let res = last_number(&dump_number("d_val", d)).unwrap();
    if res > 1.108e-05 {
        log::puts("Faulty OCC493 (case 2)");
    }
}
