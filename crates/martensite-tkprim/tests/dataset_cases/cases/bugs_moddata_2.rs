//! `bugs_moddata_2` case bodies (the inputs of bug23165 are public dataset members).
use crate::support::{
    bounding::{Bounding, bounding},
    curves::{explode, mkedge_range, shape_set, vertex_set},
    distance::distmini,
    geometry::line,
    log,
    logged_checks::checkreal,
    misc::{brep_int_cs, scalexyz},
    prim::{psphere, psphere_angle, psphere_latitudes, ptorus},
    props::Prop,
    props_view::{Expected, checkprops, props_command},
    restore::{Restored, restore_as},
    session::{Session, ViewStep, checkview_display},
    text::numbers_after,
    topology::{checksection, dump_shape, nurbsconvert, scientific, trotate, ttranslate},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

/// `regexp {.* is a ([A-Za-z0-9]*)} [whatis brics_i] full info`: the `whatis`
/// procedure returns `brics_i is a [dtyp brics_i]`, so the capture is `point` when
/// the variable exists (the point `whatis` text) and empty otherwise.
fn whatis_brics(points: &[Point3], index: usize) -> Option<&'static str> {
    if index <= points.len() {
        log::result("point");
        Some("point")
    } else {
        Some("")
    }
}

/// `regexp {Point : (x), (y), (z)} [dump brics_i]`.
fn dumped_point(points: &[Point3], index: usize) -> [f64; 3] {
    let p = points
        .get(index - 1)
        .unwrap_or_else(|| panic!("can't read \"x\": brics_{index} is not a variable"));
    // the point dump: `Point : x, y, z` in scientific notation at precision 15.
    let dump = format!(
        "\n\n*********** Dump of brics_{index} *************\nPoint : {}, {}, {}\n",
        scientific(p.x(), 15),
        scientific(p.y(), 15),
        scientific(p.z(), 15)
    );
    log::result(&dump);
    let values = numbers_after(&dump, "Point : ", 3, ",").expect("Point : x, y, z");
    [values[0], values[1], values[2]]
}

/// The common body of bug22809_1..4: `BRepIntCS l r` of the line through the poles;
/// `second` is the index of the point the script reads as the second one, `apex` whether
/// the point order may be swapped (the line through the exact poles).
fn bug22809(s: &mut dyn Session, shape: &Shape, x: f64, y: f64, second: &[usize], apex: bool) {
    let bug_number = "OCC22809";
    let l = line([x, y, -100.], [0., 0., 1.]);
    s.set("l", &Restored::Curve(l.clone()));
    let points = brep_int_cs(s, &l, shape);
    let info1 = whatis_brics(&points, 1).unwrap_or_else(|| panic!("can't read \"info1\""));
    let mut info2 = None;
    for index in second {
        if let Some(info) = whatis_brics(&points, *index) {
            info2 = Some(info);
        }
    }
    let info2 = info2.unwrap_or_else(|| panic!("can't read \"info2\""));
    let status_point = i32::from(!(info1 == "point" && info2 == "point"));
    whatis_brics(&points, 3).unwrap_or_else(|| panic!("can't read \"info3\""));
    // `status_nb` is reset to 0 whatever `info3` is: multiple result points are to be
    // processed by other algorithms.
    let status_nb = 0;
    let last = *second.last().unwrap();
    let [x1, y1, z1] = dumped_point(&points, 1);
    dumped_point(&points, 2);
    let [x2, y2, z2] = dumped_point(&points, last);
    let (good_x, good_y) = if apex { (0., 0.) } else { (-1.0e-06, 1.0e-10) };
    let (good_z1, good_z2) = (-100., 100.);
    checkreal("x1", x1, good_x, 0., 0.001);
    checkreal("y1", y1, good_y, 0., 0.001);
    if apex {
        if (z1 - good_z1).abs() > 0.001 * good_z1.abs() {
            checkreal("z1", z1, good_z2, 0., 0.001);
        }
    } else {
        checkreal("z1", z1, good_z1, 0., 0.001);
    }
    checkreal("x2", x2, good_x, 0., 0.001);
    checkreal("y2", y2, good_y, 0., 0.001);
    if apex {
        if (z2 - good_z2).abs() > 0.001 * good_z2.abs() {
            checkreal("z2", z2, good_z1, 0., 0.001);
        }
    } else {
        checkreal("z2", z2, good_z2, 0., 0.001);
    }
    if !apex {
        log::puts("");
    }
    if status_point == 0 && status_nb == 0 {
        log::puts(&format!("OK {bug_number}"));
    } else {
        log::puts(&format!("Faulty {bug_number}"));
    }
}

fn banner_22809() {
    log::puts("============");
    log::puts("OCC22809");
    log::puts("============");
    log::puts("");
}

fn nurbs_sphere(s: &mut dyn Session) -> Shape {
    let a = shape_set(s, "a", psphere(100.));
    shape_set(s, "r1", nurbsconvert(&a))
}

fn scaled_sphere(s: &mut dyn Session) -> Shape {
    let a = shape_set(s, "a", psphere(100.));
    shape_set(s, "r2", scalexyz(&a, [2., 1., 1.]))
}

/// BRepIntCS of the B-spline sphere and the line through its poles.
pub fn bug22809_1(s: &mut dyn Session) {
    banner_22809();
    let r1 = nurbs_sphere(s);
    bug22809(s, &r1, 0., 0., &[2, 41], true);
}

/// BRepIntCS of the scaled sphere (an ellipsoid) and the line through its
/// apexes.
pub fn bug22809_2(s: &mut dyn Session) {
    banner_22809();
    let r2 = scaled_sphere(s);
    bug22809(s, &r2, 0., 0., &[23], true);
}

/// The same with the line slightly off the poles of the B-spline sphere.
pub fn bug22809_3(s: &mut dyn Session) {
    banner_22809();
    let r1 = nurbs_sphere(s);
    bug22809(s, &r1, -0.000001, 0.0000000001, &[2], false);
}

/// The same with the line slightly off the apexes of the ellipsoid.
pub fn bug22809_4(s: &mut dyn Session) {
    banner_22809();
    let r2 = scaled_sphere(s);
    bug22809(s, &r2, -0.000001, 0.0000000001, &[2], false);
}

/// The six `-save` values checked against `good` (`checkreal name value good 0 0.001`).
fn check_saved(names: [&str; 6], saved: [f64; 6], good: [f64; 6]) {
    for ((name, value), good) in names.iter().zip(saved).zip(good) {
        checkreal(name, value, good, 0., 0.001);
    }
}

/// BndLib_Add3dCurve gives the right boxes of a B-spline edge and of an edge on
/// a segment of the restored curve.
pub fn bug23165(s: &mut dyn Session) {
    log::puts("================");
    log::puts("OCC23165");
    log::puts("================");
    log::puts("");
    let bug_number = "OCC23165";
    s.step(ViewStep::SmallView("AXON"));
    let mut exception_status = 0;
    let e1 = restore_as(s, "OCC23165-edge1.brep", "e1").shape();
    s.step(ViewStep::Donly(&["e1"]));
    let msg = bounding(s, Bounding::of(&e1)).text;
    s.step(ViewStep::Fit);
    if msg.split_whitespace().any(|word| word == "exception") {
        exception_status = 1;
    } else {
        let saved = bounding(
            s,
            Bounding {
                save: true,
                ..Bounding::of(&e1)
            },
        )
        .saved
        .expect("bounding -save");
        check_saved(
            ["e1_x1", "e1_y1", "e1_z1", "e1_x2", "e1_y2", "e1_z2"],
            saved,
            [
                -17.610622244944413,
                -0.010622244944396092,
                -3.0106222449443982,
                -17.589377755055537,
                5.700038816113608,
                -1.6251884728673096,
            ],
        );
    }
    let c = restore_as(s, "OCC23165-curve.rle", "c").curve();
    let result = shape_set(s, "result", mkedge_range(&c, 20., 36.));
    s.step(ViewStep::Donly(&["result"]));
    let saved = bounding(
        s,
        Bounding {
            save: true,
            ..Bounding::of(&result)
        },
    )
    .saved
    .expect("bounding -save");
    s.step(ViewStep::Fit);
    check_saved(
        ["x1", "y1", "z1", "x2", "y2", "z2"],
        saved,
        [
            -17.6105835090592,
            -4.7133570660117918,
            -4.3679100133425806,
            -17.589416490940806,
            5.7000000802283299,
            -1.6252272087525899,
        ],
    );
    if exception_status == 0 {
        log::puts(&format!("{bug_number}: OK"));
    } else {
        log::puts(&format!("{bug_number}: Faulty"));
    }
    checkview_display(s, Some("result"), "bug23165");
    checkprops(s, "result", &result, Expected::l(15.8888));
    let alone = checksection(&result);
    log::result(&format!(" nb alone Vertices : {alone}\n"));
}

/// The isolines of the sphere of radius 10 cut at -90 degrees.
pub fn bug41(s: &mut dyn Session) {
    log::puts("================");
    log::puts("OCC41");
    log::puts("================");
    log::puts("");
    log::puts("Isos lines may be drawn incorrectly");
    shape_set(s, "result", psphere_angle(10., -90.));
    checkview_display(s, Some("result"), "bug41");
}

/// Extrema_ExtPElS from the torus centre-circle point gives an edge solution (no
/// gp_VectorWithNullMagnitude).
pub fn bug7(s: &mut dyn Session) {
    log::puts("================");
    log::puts("OCC7");
    log::puts("================");
    log::puts("");
    let tor = shape_set(s, "tor", ptorus(3., 1.));
    let faces = explode(s, "tor", &tor, Some(ShapeEnum::Face));
    let vv = vertex_set(s, "vv", [0., 3., 0.]);
    let result = distmini(s, "result", &vv, &faces[0], None).expect("distmini");
    let info = dump_shape("result", &result.solutions[0]);
    log::result(&info);
    if !info.contains("TShape # 1 : EDGE")
        || !info.contains("TShape # 2 : VERTEX")
        || !info.contains("TShape # 3 : VERTEX")
    {
        log::puts("Error : Result is incorrect");
    }
    checkview_display(s, Some("result"), "bug7");
}

/// The `regexp -all -inline { +N +N +N}` matches of a report, then the numbers in them.
fn number_triples(text: &str) -> Vec<String> {
    let is_number = |word: &str| {
        !word.is_empty()
            && word
                .chars()
                .all(|c| c.is_ascii_digit() || "-.+eE".contains(c))
    };
    let mut numbers = Vec::new();
    for line in text.split('\n') {
        // Each `%15g %15g %15g` row is one match: three blank-separated numbers after
        // leading blanks; the other lines have at most one number after their label.
        let words: Vec<&str> = line.split(' ').filter(|w| !w.is_empty()).collect();
        if line.starts_with(' ') && words.len() == 3 && words.iter().all(|w| is_number(w)) {
            numbers.extend(words.iter().map(|w| w.to_string()));
        }
    }
    numbers
}

/// The matrix of inertia of the translated and rotated sphere band has zero
/// off-diagonal terms.
pub fn bug705(s: &mut dyn Session) {
    log::puts("==========");
    log::puts("OCC705    ");
    log::puts("==========");
    log::puts("");
    let result = shape_set(s, "result", psphere_latitudes(10., 10., 10.));
    let result = shape_set(s, "result", ttranslate(&result, [10., 10., 10.], false));
    let result = shape_set(
        s,
        "result",
        trotate(&result, [0., 0., 0.], [0., 1., 1.], 34., false),
    );
    let list = props_command(s, Prop::Volume, &result, None, false).text;
    let matrix = number_triples(&list.replace('\0', ""));
    let value = |index: usize| matrix[index].parse::<f64>().unwrap();
    if value(1) != 0. || value(3) != 0. || value(5) != 0. || value(7) != 0. {
        log::puts("Faulty OCC705");
    } else {
        log::puts("OCC705 OK");
    }
    checkprops(s, "result", &result, Expected::l(123.755));
    let alone = checksection(&result);
    log::result(&format!(" nb alone Vertices : {alone}\n"));
    checkview_display(s, Some("result"), "bug705");
}
