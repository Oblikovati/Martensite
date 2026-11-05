//! `lowalgos_intss` scripts built on `intersect`: the curves of two surfaces, sampled against
//! both surfaces and joined by a general fuse.
use crate::support::{
    bop_session::BopSession,
    checkview::screenshot,
    commands::{explode, pcone, psphere},
    lowalgos_commands::{
        SampledDistances, SavedSurface, SurfaceCurves, add_curve_edge, binrestore, bound_surface,
        check_intersection_result, checked_curves, checklength, checkmaxtol_within,
        checkoverlapedges, cone, curve_range, cylinder, distmini, intersect, intersect_caught,
        mksurface, name_refs, restore_surface, torus,
    },
    nbshapes::{Counting, NbCount, checknbshapes},
    section_check::checksection_expecting,
    session::{Session, ViewStep},
    surface_intersection::{Surface, edge_of, xdistcs},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{geom::GeomRectangularTrimmedSurface, top_abs::ShapeEnum};
use std::f64::consts::PI;

const CONFUSION_LOOP: f64 = 1.0e-9;

fn sampled(samples: usize, limit: f64, shortest_range: f64) -> SampledDistances {
    SampledDistances {
        samples,
        limits: [limit, limit],
        shortest_range,
    }
}

/// `smallview`, `don res_*`, `fit`, `don s1 s2`, `disp res_*`, and the capture.
fn view_curves_then_surfaces(s: &mut dyn Session, answer: &SurfaceCurves, surfaces: &[&str; 2]) {
    let names = name_refs(&answer.names);
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&names));
    s.view(ViewStep::Fit);
    s.view(ViewStep::Donly(surfaces));
    s.view(ViewStep::Disp(&names));
    screenshot(s, "photo");
}

/// `smallview`, `don result*`, `fit`: only `result` itself matches.
fn view_result(s: &mut dyn Session) {
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
}

fn checked_gaps(s: &mut dyn Session, name: &str, edges: usize, vertices: usize) {
    checksection_expecting(s, name, 0);
    checkmaxtol_within(s, name, "2.0e-7");
    let expected = [NbCount::Vertices(vertices), NbCount::Edges(edges)];
    checknbshapes(s, name, Counting::Located, &expected);
}

pub fn bug21494_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC21494: Intersection between cone and sphere fails");
    println!("========");
    println!();
    pcone(s, "c", 15., 0., 10.);
    psphere(s, "s", 10.);
    explode(s, "c", Some(ShapeEnum::Face));
    explode(s, "s", Some(ShapeEnum::Face));
    let (sc, ss) = (mksurface(s, "sc", "c_1"), mksurface(s, "ss", "s_1"));
    let answer = intersect(s, "res", &sc, &ss, precision::confusion());
    if answer.points.is_empty() {
        println!("Error: Cone apex and Pole of sphere are excluded from the intersection result");
    }
    let ic = answer.curves.len();
    if ic != 1 {
        println!("Error: 1 curves is expected but {ic} ones are found");
    } else {
        check_intersection_result(&answer, [&sc, &ss], 100, [2.0e-7, 2.0e-7]);
        edge_of(s, "result", &answer.curves[0]);
        checked_gaps(s, "result", 1, 1);
    }
    view_result(s);
    screenshot(s, "photo");
}

pub fn bug23176(s: &mut dyn Session) {
    println!("================");
    println!("OCC23176");
    println!("================");
    println!();
    let s1 = restore_surface(s, "bug23176_surface1_draw.draw", "s1", SavedSurface::Any);
    let s2 = restore_surface(s, "bug23176_surface2_draw.draw", "s2", SavedSurface::Any);
    let answer = intersect(s, "result", &s1, &s2, precision::confusion());
    let mut bop = BopSession::default();
    let distances = sampled(10, 2.0e-7, 1.0e-20);
    checked_curves(s, &mut bop, &answer, [&s1, &s2], &distances);
    bop.bfillds();
    bop.bbuild(s, "rw");
    checked_gaps(s, "rw", 4, 4);
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&name_refs(&answer.names)));
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["s1", "s2"]));
    screenshot(s, "photo");
}

pub fn bug24429(s: &mut dyn Session) {
    println!("========");
    println!("OCC24429: Cylinder-torus intersection throws an ex-ception and produces no result");
    println!("========");
    println!();
    println!("TODO OCC24429 ALL: Error: 0 vertices are expected but 2 are found.");
    let s1 = restore_surface(s, "bug24429_s1.draw", "s1", SavedSurface::Any);
    let s2 = restore_surface(s, "bug24429_s2.draw", "s2", SavedSurface::Any);
    let answer = intersect(s, "res", &s1, &s2, precision::confusion());
    let mut bop = BopSession::default();
    let distances = sampled(100, 2.0e-6, CONFUSION_LOOP);
    checked_curves(s, &mut bop, &answer, [&s1, &s2], &distances);
    bop.bfillds();
    bop.bbuild(s, "rs");
    checknbshapes(s, "rs", Counting::Located, &[NbCount::Edges(8)]);
    checksection_expecting(s, "rs", 0);
    view_curves_then_surfaces(s, &answer, &["s1", "s2"]);
}

/// The two scripts on the saved spline and the cylinder: only their tolerance and limit differ.
fn bug29972_spline(s: &mut dyn Session, tolerance: f64, limit: f64) {
    println!("========");
    println!("OCC29972: Intersection curve has a weird gap in the middle of it");
    println!("========");
    println!();
    let s1 = restore_surface(s, "bug29972_s1.draw", "s1", SavedSurface::BSpline);
    let placed = (
        [-120., 54.2955104312028, -116.],
        [1., 0., 0.],
        [0., 0., -1.],
    );
    let s2 = cylinder(s, "s2", placed, 28.);
    let answer = intersect(s, "res", &s1, &s2, tolerance);
    let mut bop = BopSession::default();
    let distances = sampled(100, limit, CONFUSION_LOOP);
    checked_curves(s, &mut bop, &answer, [&s1, &s2], &distances);
    bop.bfillds();
    bop.bbuild(s, "rs");
    checknbshapes(s, "rs", Counting::Located, &[NbCount::Edges(2)]);
    checksection_expecting(s, "rs", 0);
    view_curves_then_surfaces(s, &answer, &["s1", "s2"]);
}

pub fn bug29972_1(s: &mut dyn Session) {
    bug29972_spline(s, precision::confusion(), 3.0e-6);
}

pub fn bug29972_2(s: &mut dyn Session) {
    bug29972_spline(s, 1.0e-4, 2.0e-5);
}

fn bug29972_3_gaps(s: &mut dyn Session) {
    let nbv = checksection_expecting(s, "rs", 4);
    if nbv < 2 {
        println!(
            "Error : The section is closed. The result is possible to better than on MASTER version. Please check carefully."
        );
        return;
    }
    for i in 1..nbv {
        for j in i + 1..=nbv {
            let distance = distmini(s, "dd", &format!("alone_{i}"), &format!("alone_{j}"));
            if distance > 3.0e-6 {
                println!("Error: The distance between alone_{i} and alone_{j} is too big");
            }
        }
    }
}

pub fn bug29972_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC29972: Intersection curve has a weird gap in the middle of it");
    println!("========");
    println!();
    let down = [0., 0., -1.];
    let s1 = torus(s, "s1", ([-127., 84., 100.], down, [1., 0., 0.]), 23., 10.);
    let placed = ([-132., 61.5500556793564, 131.], down, [1., 0., 0.]);
    let s2 = cylinder(s, "s2", placed, 10.);
    let answer = intersect(s, "res", &s1, &s2, 1.0e-4);
    let mut bop = BopSession::default();
    let distances = sampled(100, 6.0e-7, CONFUSION_LOOP);
    checked_curves(s, &mut bop, &answer, [&s1, &s2], &distances);
    bop.bfillds();
    bop.bbuild(s, "rs");
    checknbshapes(s, "rs", Counting::Located, &[NbCount::Edges(6)]);
    bug29972_3_gaps(s);
    view_curves_then_surfaces(s, &answer, &["s1", "s2"]);
}

pub fn bug29994(s: &mut dyn Session) {
    println!("========");
    println!("0029994: Misprint in IntWalk_PWalking::Perform(...) method");
    println!("========");
    println!();
    let placed = (
        [185.793144150183, -13., 70.9931441501827],
        [0., -1., 0.],
        [0., 0., -1.],
    );
    let s1 = torus(s, "s1", placed, 27.0094480547676, 0.25);
    let s2 = restore_surface(s, "bug29994_s2.draw", "s2", SavedSurface::BSpline);
    let answer = intersect(s, "res", &s1, &s2, 1.0e-4);
    let mut bop = BopSession::default();
    let distances = sampled(100, 2.0e-7, CONFUSION_LOOP);
    checked_curves(s, &mut bop, &answer, [&s1, &s2], &distances);
    if answer.curves.len() == 1 {
        println!("OK: good number of curves!");
        checklength("res_1", &answer.curves[0], "0.70541045554962345", 1.0e-3);
    } else {
        println!("Error: Incorrect number of curves in intersection result!");
        bop.bfillds();
        bop.bbuild(s, "result");
        checksection_expecting(s, "result", 2);
        checkmaxtol_within(s, "result", "2.0e-7");
        let expected = [NbCount::Vertices(2), NbCount::Edges(1)];
        checknbshapes(s, "result", Counting::Located, &expected);
    }
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&name_refs(&answer.names)));
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["s1", "s2"]));
    screenshot(s, "photo");
}

pub fn bug30082_2(s: &mut dyn Session) {
    println!("========");
    println!("0030082: Intersection algorithm returns curve with big tolerance value");
    println!("========");
    println!();
    let first = (
        [-106.471129473161, -64.3442185874231, -19.25],
        [1., 0., 0.],
        [0., 0., -1.],
    );
    let second = (
        [-124.971129473161, -67.0942185874231, -20.5],
        [0., 0., 1.],
        [1., 0., -0.],
    );
    let s1 = cylinder(s, "s1", first, 0.25);
    let s2 = cylinder(s, "s2", second, 3.);
    let answer = intersect(s, "res", &s1, &s2, 1.0e-4);
    let mut bop = BopSession::default();
    let distances = sampled(100, 1.0e-4, CONFUSION_LOOP);
    checked_curves(s, &mut bop, &answer, [&s1, &s2], &distances);
    let ic = answer.curves.len();
    if ic != 4 {
        println!("Error: 4 curves are expected but {ic} ones are found");
    }
    bop.bfillds();
    bop.bbuild(s, "result");
    view_result(s);
    checked_gaps(s, "result", 4, 3);
    screenshot(s, "photo");
}

/// `trim name surface 0 2*pi 0 2*pi/13`, bound.
fn trimmed(s: &mut dyn Session, name: &str, surface: &Surface) -> Surface {
    let trimmed = GeomRectangularTrimmedSurface::new(
        surface,
        (0., 2. * PI),
        (0., 2. * PI / 13.),
        (true, true),
    )
    .unwrap_or_else(|failure| panic!("trim {name}: {failure:?}"));
    bound_surface(s, name, Handle::new(trimmed).cast())
}

/// The script's loop over `res_N`, with the overlap of each curve and the earlier ones.
fn checked_with_overlaps(
    s: &mut dyn Session,
    bop: &mut BopSession,
    answer: &SurfaceCurves,
    surfaces: [&Surface; 2],
) {
    for (index, curve) in answer.curves.iter().enumerate() {
        let range = curve_range(curve);
        if range[1] - range[0] < CONFUSION_LOOP {
            println!("Error: Wrong curve's range!");
        }
        let name = &answer.names[index];
        xdistcs(name, curve, surfaces[0], range, 100, 2.0e-7);
        xdistcs(name, curve, surfaces[1], range, 100, 2.0e-7);
        for earlier in (0..index).rev() {
            overlap_with_earlier(s, answer, index, earlier);
        }
        add_curve_edge(s, bop, curve);
    }
}

fn overlap_with_earlier(s: &mut dyn Session, answer: &SurfaceCurves, index: usize, earlier: usize) {
    let first = edge_of(s, "e1", &answer.curves[index]);
    let second = edge_of(s, "e2", &answer.curves[earlier]);
    let coe = checkoverlapedges(("e1", &first), ("e2", &second), 5.0e-5);
    let (ic, ip) = (index + 1, earlier + 1);
    println!("res_{ic} <-> res_{ip}: {coe}");
    if !coe.contains("Edges are not overlapped") {
        println!("Error: res_{ic} and res_{ip} are overlapped");
    }
}

/// The part of `bug567_1` and `bug567_2` after the `catch`: only the expected numbers differ.
fn bug567_checks(
    s: &mut dyn Session,
    answer: &SurfaceCurves,
    surfaces: [&Surface; 2],
    good_curves: usize,
    edges_and_section: (usize, usize),
) {
    let mut bop = BopSession::default();
    checked_with_overlaps(s, &mut bop, answer, surfaces);
    let ic = answer.curves.len();
    if ic != good_curves {
        println!("Error: {good_curves} curves are expected but {ic} ones are found.");
    }
    if ic != 1 {
        bop.bfillds();
        bop.bbuild(s, "rs");
        let (edges, section) = edges_and_section;
        checknbshapes(s, "rs", Counting::Located, &[NbCount::Edges(edges)]);
        checksection_expecting(s, "rs", section);
    }
}

/// The view of a `bug567` script, whatever its `catch` caught.
fn view_caught(s: &mut dyn Session, answer: Option<&SurfaceCurves>) {
    let empty = SurfaceCurves {
        curves: Vec::new(),
        names: Vec::new(),
        points: Vec::new(),
    };
    view_curves_then_surfaces(s, answer.unwrap_or(&empty), &["s1", "s2"]);
}

pub fn bug567_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC567: Can not intersect two Rectangular Trimmed Surfaces .");
    println!("========");
    println!();
    println!("TODO ?OCC30012 Linux: Error: 10 curves are expected but 9 ones are found.");
    println!("TODO ?OCC30012 Linux: Error: 10 curves are expected but 8 ones are found.");
    println!("TODO ?OCC29910 Windows: Error: 10 curves are expected but 11 ones are found.");
    println!(
        "TODO ?OCC29910 Windows: Error :  is WRONG because number of EDGE entities in shape \"rs\" is 15"
    );
    println!(
        "TODO ?OCC29910 Linux: Error :  is WRONG because number of EDGE entities in shape \"rs\" is 8"
    );
    println!("TODO ?OCC29910 Windows: Error: 0 vertices are expected but 2 are found");
    let s1 = restore_surface(s, "OCC567a.draw", "s1", SavedSurface::Any);
    let s2 = restore_surface(s, "OCC567b.draw", "s2", SavedSurface::Any);
    let answer = intersect_caught(s, "res", &s1, &s2, precision::confusion());
    if let Some(answer) = &answer {
        bug567_checks(s, answer, [&s1, &s2], 10, (14, 0));
    } else {
        println!("Faulty OCC567: function intersection works wrongly with infinite Surfaces");
    }
    view_caught(s, answer.as_ref());
}

pub fn bug567_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC567: Can not intersect two Rectangular Trimmed Surfaces .");
    println!("========");
    println!();
    let s1 = restore_surface(s, "OCC567a.draw", "s1", SavedSurface::Any);
    let s2 = restore_surface(s, "OCC567b.draw", "s2", SavedSurface::Any);
    let s1x = trimmed(s, "s1x", &s1);
    let s2x = trimmed(s, "s2x", &s2);
    let answer = intersect_caught(s, "res", &s1x, &s2x, precision::confusion());
    if let Some(answer) = &answer {
        bug567_checks(s, answer, [&s1, &s2], 1, (1, 2));
    } else {
        println!(
            "Faulty OCC565 exception: function intersection works wrongly with trimmed Surfaces"
        );
    }
    view_caught(s, answer.as_ref());
}

fn basis_of(s: &mut dyn Session, name: &str, surface: &Surface) -> Surface {
    let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
    let basis = trimmed.get().map_or_else(
        || surface.clone(),
        GeomRectangularTrimmedSurface::basis_surface,
    );
    bound_surface(s, name, basis)
}

/// The tail both `bug29807_i5001` and `bug29807_i5002` end with.
fn bug29807_built(s: &mut dyn Session, surfaces: [&Surface; 2]) {
    let answer = intersect(s, "res", surfaces[0], surfaces[1], 1.0e-4);
    let mut bop = BopSession::default();
    let distances = sampled(100, 2.0e-7, 1.0e-20);
    checked_curves(s, &mut bop, &answer, surfaces, &distances);
    bop.bfillds();
    bop.bbuild(s, "result");
    view_result(s);
    checked_gaps(s, "result", 2, 2);
    screenshot(s, "photo");
}

pub fn bug29807_i5001(s: &mut dyn Session) {
    println!("========");
    println!("0029807: Impossible to cut cone from prism");
    println!("========");
    println!();
    binrestore(s, "bug29807_f1.bin", "f1");
    binrestore(s, "bug29807_f2.bin", "f2");
    let (s1, s2) = (mksurface(s, "s1", "f1"), mksurface(s, "s2", "f2"));
    let (s1, s2) = (basis_of(s, "s1", &s1), basis_of(s, "s2", &s2));
    bug29807_built(s, [&s1, &s2]);
}

pub fn bug29807_i5002(s: &mut dyn Session) {
    println!("========");
    println!("0029807: Impossible to cut cone from prism");
    println!("========");
    println!();
    let first = (
        [11.4307383137554, 3.49999999999979, -89.7537975119388],
        [0., 0., 1.],
        [1., 0., 0.],
    );
    let second = (
        [-3.6479413426839, -11.578679656441, -89.9782110643133],
        [0., 0., 1.],
        [0., 1., 0.],
    );
    let s1 = cone(
        s,
        "s1",
        first,
        80.000725670142287835190342147806,
        9.45659107381736,
    );
    let s2 = cone(s, "s2", second, 5., 0.250951325477062);
    bug29807_built(s, [&s1, &s2]);
}
