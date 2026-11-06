//! `lowalgos_intss` scripts built on `bopcurves`, continued: the later `bug26576` to `bug32849`.
use crate::support::{
    bopcurves::{BopCurvesOptions, BoundCurves, bopcurves_bound},
    checkview::screenshot,
    commands::{explode, prism, ptorus, trotate},
    construction::{copy, mkplane, tcopy},
    lowalgos_commands::{
        Curve, Surface, banner, bound_surface, checkoverlapedges, donly_curves, dump_curve,
        intersect, maxtolerance, name_refs, pcurve, plane, settolerance,
    },
    restore::restore,
    session::{Drawing, Session, ViewStep},
    surface_commands::mkface,
    surface_intersection::edge_of,
    tolerance_check::{checkreal, tolerance_report},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomSurfaceOfRevolution, GeomTrimmedCurve},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3};

/// `circle cc ...`, `trim cc cc 0 3.14159265358979`, `revsurf ss1 cc ...`.
fn bug26576_revolution(s: &mut dyn Session) -> Surface {
    let center = Point3::new(1.5, 3.125, 0.);
    let placed = Axis2::with_x_direction(center, Direction3::Z, Direction3::X).unwrap();
    let circle: Curve = Handle::new(GeomCircle::new(placed, 0.25).unwrap()).cast();
    let trimmed = GeomTrimmedCurve::new(&circle, 0., 3.14159265358979, true, true).unwrap();
    let curve: Curve = Handle::new(trimmed).cast();
    let axis = Axis1::new(center, Direction3::new(-1., 0., 0.).unwrap());
    let revolution = GeomSurfaceOfRevolution::new(&curve, axis).unwrap();
    bound_surface(s, "ss1", Handle::new(revolution).cast())
}

/// The script's pair of `checkoverlapedges`, on both kinds of curve.
fn bug26576_overlaps(s: &mut dyn Session, curves: [&[Curve]; 2], tolerance: f64) {
    for first in 0..curves[0].len() {
        for second in first + 1..curves[0].len() {
            let eb = [
                edge_of(s, "eb1", &curves[0][first]),
                edge_of(s, "eb2", &curves[0][second]),
            ];
            let ei = [
                edge_of(s, "ei1", &curves[1][first]),
                edge_of(s, "ei2", &curves[1][second]),
            ];
            let coeb = checkoverlapedges(("eb1", &eb[0]), ("eb2", &eb[1]), tolerance);
            let coei = checkoverlapedges(("ei1", &ei[0]), ("ei2", &ei[1]), tolerance);
            let (i, j) = (first + 1, second + 1);
            println!("{i}<->{j}: {coeb}");
            println!("{i}<->{j}: {coei}");
            if !coeb.contains("Edges are not overlapped") {
                println!("Error: c_{i} and c_{j} are overlapped");
            }
            if !coei.contains("Edges are not overlapped") {
                println!("Error: intres_{i} and intres_{j} are overlapped");
            }
        }
    }
}

pub fn bug26576_4(s: &mut dyn Session) {
    banner(12, "OCC26576");
    let ss1 = bug26576_revolution(s);
    let at = [3.48352775473762, 1.7282347013387, 0.];
    let ss2 = plane(s, "ss2", at, [0., 0., -1.], [-1., 0., 0.]);
    let answer = intersect(s, "intres", &ss1, &ss2, precision::confusion());
    if answer.curves.len() != 2 {
        let found = answer.curves.len();
        println!("Error in geometric intersection: 2 curves are expected but {found} are found!");
    } else {
        println!("OK : Geometric intersection is good.");
    }
    mkface(s, "b1", &ss1);
    mkface(s, "b2", &ss2);
    s.view(ViewStep::Donly(&["b1"]));
    let log = bopcurves_bound(s, "b1", "b2", BopCurvesOptions::TWO_D);
    let (reached, nb_curv) = log.reached_and_count();
    checkreal(
        "Tolerance Reached",
        reached,
        "1.9321146113460029e-008",
        1.0e-7,
        0.0,
    );
    if nb_curv != 2 {
        println!("Error: 2 curves are expected but {nb_curv} are found!");
    }
    let tolerance = log.reached().max(1.0e-7);
    let face_curves: Vec<Curve> = (0..nb_curv).map(|index| log.curve_at(index)).collect();
    bug26576_overlaps(s, [&face_curves, &answer.curves], tolerance);
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Fit);
    screenshot(s, "photo");
}

pub fn bug26684_2(s: &mut dyn Session) {
    banner(12, "OCC26684");
    restore(s, "IR2_TuyoPipeBSpline_20_draw2.brep", "b1");
    restore(s, "IR2_sphere2_20.brep", "b2");
    explode(s, "b1", Some(ShapeEnum::Face));
    copy(s, "b1_1", "f1");
    explode(s, "b2", Some(ShapeEnum::Face));
    copy(s, "b2_1", "f2");
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&["f1", "f2"]));
    s.view(ViewStep::Fit);
    let spatial = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::PLAIN);
    let (tolerance, _) = spatial.reached_and_count();
    checkreal(
        "Reached tolerance",
        tolerance,
        "1.2530391548405894e-008",
        1.0e-7,
        0.,
    );
    let planar = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::TWO_D);
    let (tolerance, _) = planar.reached_and_count();
    checkreal(
        "Reached tolerance",
        tolerance,
        "0.00040497924613267194",
        0.,
        1.0e-2,
    );
    screenshot(s, "photo");
}

/// The script's `regexp` for one pole of a `dump`, as three printed coordinates.
fn pole_words(dump: &str, pole: usize) -> [String; 3] {
    let prefix = format!("{pole} : ");
    let line = dump
        .lines()
        .find_map(|line| line.trim_start().strip_prefix(&prefix))
        .unwrap_or_else(|| panic!("dump: expected pole {pole} in {dump:?}"));
    let words: Vec<String> = line.split(',').map(|word| word.trim().to_owned()).collect();
    words
        .try_into()
        .unwrap_or_else(|words| panic!("pole {pole}: expected 3 coordinates, got {words:?}"))
}

/// `Degree 6, 107 Poles, 22  Knots`: the three numbers of a `dump`.
fn dump_counts(dump: &str) -> [usize; 3] {
    let line = dump
        .lines()
        .find(|line| line.trim_start().starts_with("Degree "))
        .unwrap_or_else(|| panic!("dump: expected Degree in {dump:?}"));
    let fields: Vec<&str> = line.split([',', ' ']).filter(|f| !f.is_empty()).collect();
    [fields[1], fields[2], fields[4]].map(|field| {
        field
            .parse()
            .unwrap_or_else(|cause| panic!("dump {line:?}: {cause}"))
    })
}

fn report_pole(pole: usize, labels: [&str; 3], words: &[String; 3]) {
    println!("Pole={pole}");
    for (label, word) in labels.into_iter().zip(words) {
        println!("{label}={word}");
    }
    println!();
}

fn check_pole(labels: [&str; 3], words: &[String; 3], expected: [&str; 3], tol_abs: f64) {
    for ((label, word), wanted) in labels.into_iter().zip(words).zip(expected) {
        checkreal(label, word, wanted, tol_abs, 0.01);
    }
    println!();
}

pub fn bug27227(s: &mut dyn Session) {
    banner(12, "OCC27227");
    ptorus(s, "a0", 100., 20.);
    tcopy(s, "a0", "a1");
    // Beyond the script: an earlier test of this case saw a new topology root.
    assert!(
        !s.shape("a1").unwrap().is_same(s.shape("a0").unwrap()),
        "tcopy a0 a1: expected a new root"
    );
    trotate(s, "a1", [0.; 3], [1., 0., 0.], 90.);
    explode(s, "a0", Some(ShapeEnum::Face));
    explode(s, "a1", Some(ShapeEnum::Face));
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&["a0_1", "a1_1"]));
    let log = bopcurves_bound(s, "a0_1", "a1_1", BopCurvesOptions::TWO_D);
    let fifth = log.curve_at(4);
    let dump = dump_curve("c_5", &fifth);
    let [degree, poles, knots] = dump_counts(&dump);
    println!("Degree={degree}\nPoles={poles}\nKnotsPoles={knots}\n");
    let first_labels = ["X_first", "Y_first", "Z_first"];
    let end_labels = ["X_end", "Y_end", "Z_end"];
    let (first, end) = (pole_words(&dump, 1), pole_words(&dump, poles));
    report_pole(1, first_labels, &first);
    report_pole(poles, end_labels, &end);
    let tol_abs: f64 = tolerance_report(s, "a0").parse().unwrap();
    check_pole(first_labels, &first, ["80.", "0.", "0."], tol_abs);
    check_pole(end_labels, &end, ["120.", "0.", "0."], tol_abs);
}

pub fn bug27252_1(s: &mut dyn Session) {
    banner(12, "OCC27252");
    restore(s, "bug27221.brep", "a");
    explode(s, "a", None);
    mkplane(s, "f", "a_2");
    prism(s, "p", "f", [-25., 0., 0.]);
    explode(s, "a_1", Some(ShapeEnum::Face));
    explode(s, "p", Some(ShapeEnum::Face));
    s.view(ViewStep::Donly(&["a_1_7", "p_3"]));
    let log = bopcurves_bound(s, "a_1_7", "p_3", BopCurvesOptions::TWO_D);
    let (_, nb_curv) = log.reached_and_count();
    if nb_curv != 1 {
        println!("Error: Curve Number is bad!");
    }
    if log.reached() > 1.0e-7 {
        println!("Error: Tolerance is too big!");
    }
    s.view(ViewStep::SmallView);
    donly_curves(s, &log, &[]);
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["a_1_7", "p_3"]));
    screenshot(s, "photo");
}

/// `smallview`, `don b1_3 b2_1 c_*`, `fit`, a capture; `v2d`, `don c2d*`, `2dfit`, a capture.
fn bug27431_views(s: &mut dyn Session, log: &BoundCurves, stage: &str) {
    s.view(ViewStep::SmallView);
    donly_curves(s, log, &["b1_3", "b2_1"]);
    s.view(ViewStep::Fit);
    screenshot(s, &format!("photo_3d_{stage}"));
    s.view(ViewStep::PlaneView);
    s.view(ViewStep::Donly(&name_refs(&log.plane_names)));
    s.view(ViewStep::Fit);
    screenshot(s, &format!("photo_2d_{stage}"));
}

pub fn bug27431(s: &mut dyn Session) {
    banner(12, "OCC27431");
    println!("1"); // What `pload ALL` returns.
    restore(s, "OCC13116_sh1.brep", "b1");
    restore(s, "OCC13116_sh2.brep", "b2");
    explode(s, "b1", Some(ShapeEnum::Face));
    explode(s, "b2", Some(ShapeEnum::Face));
    let log = bopcurves_bound(s, "b1_3", "b2_1", BopCurvesOptions::TWO_D);
    let (_, nb_curv) = log.reached_and_count();
    if log.reached() > 0.0005 {
        println!("Error: bad tolerance of result");
    }
    bug27431_views(s, &log, "1");
    if nb_curv != 4 {
        println!("Error: Number of curves is bad!");
    }
    settolerance(s, "b1_3", 2.34623727264857e-007);
    settolerance(s, "b2_1", 2.0e-7);
    let log = bopcurves_bound(s, "b1_3", "b2_1", BopCurvesOptions::TWO_D);
    if log.reached() > 0.0005 {
        println!("Error: bad tolerance of result");
    }
    bug27431_views(s, &log, "2");
}

/// `trim c c2d1_1 0 1`, `2dcvalue` at both ends and `dset dot`: the sign the script reads.
fn end_direction_dot(curve: &Handle<dyn Geom2dCurve>) -> f64 {
    let trimmed = Geom2dTrimmedCurve::new(curve, 0., 1., true, true).unwrap();
    let start = trimmed.d0(0.).unwrap();
    let (end, derivative) = trimmed.d1(1.).unwrap();
    let (u01, v01) = (end.x() - start.x(), end.y() - start.y());
    u01 * derivative.x() + v01 * derivative.y()
}

pub fn bug27780(s: &mut dyn Session) {
    banner(12, "OCC27780");
    restore(s, "buc60532a.brep", "p");
    restore(s, "buc60532b.brep", "t1");
    explode(s, "t1", Some(ShapeEnum::Face));
    let start = [[
        6.2406621764215551,
        0.23999999463558200,
        0.00034444887595448459,
        -5.0019657458625186,
    ]];
    let options = BopCurvesOptions {
        pcurves: true,
        start_points: &start,
    };
    let log = bopcurves_bound(s, "p", "t1_2", options);
    let curves = pcurve(s, "p");
    let on_first = log.lines[0].first_curve2d().clone();
    let trimmed: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dTrimmedCurve::new(&on_first, 0., 1., true, true).unwrap()).cast();
    s.bind_drawing("c", Drawing::PlaneCurve(&trimmed));
    if end_direction_dot(&on_first) < 0. {
        println!("Error: p-curve has reversed direction at its end");
    } else {
        println!("OK: p-curve has correct direction at its end");
    }
    s.view(ViewStep::PlaneView);
    let names: Vec<String> = (1..=curves.len()).map(|i| format!("p_{i}")).collect();
    let mut shown = name_refs(&names);
    shown.push("c");
    s.view(ViewStep::Donly(&shown));
    s.view(ViewStep::Fit);
    screenshot(s, "photo");
}

pub fn bug28544_2(s: &mut dyn Session) {
    println!(
        "TODO OCC28544 ALL: Error: Big tolerance value is returned by Implicit-parametric algorithm"
    );
    banner(8, "OCC28544");
    restore(s, "heal-Face1.rle", "b1");
    restore(s, "Plan.rle", "b2");
    let log = bopcurves_bound(s, "b1", "b2", BopCurvesOptions::TWO_D);
    let reached = log.reached();
    let largest: f64 = maxtolerance(s, "b1").parse().unwrap();
    if reached > largest {
        println!("Error: Big tolerance value is returned by Implicit-parametric algorithm");
    }
}

pub fn bug29673(s: &mut dyn Session) {
    banner(8, "0029673: Ex-ception while Face-Face intersection");
    restore(s, "bug29673_f1.brep", "f1");
    restore(s, "bug29673_f2.brep", "f2");
    let log = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::TWO_D);
    let (_, nb_curv) = log.reached_and_count();
    if log.reached() > 1.0e-7 {
        println!("Error: too big tolerance");
    }
    if nb_curv != 1 {
        println!("Error: Please check NbCurves for intersector");
    }
}

pub fn bug30082_1(s: &mut dyn Session) {
    banner(
        8,
        "0030082: Intersection algorithm returns curve with big tolerance value",
    );
    restore(s, "bug27928_b1.brep", "b1");
    restore(s, "bug27928_b2.brep", "b2");
    explode(s, "b1", Some(ShapeEnum::Face));
    explode(s, "b2", Some(ShapeEnum::Face));
    let log = bopcurves_bound(s, "b1_1", "b2_2", BopCurvesOptions::TWO_D);
    if let Some(toler) = log.answer.tolerance_reached.as_deref() {
        checkreal("Tolerance", toler, "6.5e-7", 0.0, 0.1);
        let nb_curv = log.answer.curves;
        if nb_curv != 1 {
            println!("Error: 1 curve is expected but {nb_curv} curves are found.");
        }
    } else {
        println!("Error: Intersection result is empty");
    }
    s.view(ViewStep::SmallView);
    donly_curves(s, &log, &[]);
    s.view(ViewStep::Fit);
    screenshot(s, "photo");
}

#[test]
fn the_end_direction_is_read_from_the_derivative() {
    use martensite_tkernel::ncollection::Array1;
    use martensite_tkg2d::geom2d::Geom2dBezierCurve;
    use martensite_tkmath::gp::Point2;
    // B(t) = 4t - 3t^2 gives B(0) = 0, B(1) = 1 and B'(1) = -2 exactly.
    let poles = [
        Point2::new(0., 0.),
        Point2::new(2., 0.),
        Point2::new(1., 0.),
    ];
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dBezierCurve::new(&Array1::from_shared(&poles, 1)).unwrap()).cast();
    assert_eq!(end_direction_dot(&curve), -2.);
}
