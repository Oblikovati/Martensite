//! `tests/lowalgos/intss/bug25950`, whose budget is its own (`lowalgos_intss_cpu200.rs`), and `bug30202_2`.
use crate::support::{
    bop_session::BopSession,
    bopcurves::{BopCurvesOptions, bopcurves_bound},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview, screenshot},
    commands::explode,
    lowalgos_commands::{
        SurfaceCurves, add_curve_edge, check_intersection_result, checkmaxtol_within,
        checkoverlapedges, intersect, mksurface,
    },
    nbshapes::{Counting, NbCount, checknbshapes},
    perf_commands::Chrono,
    properties::Prop,
    restore::restore,
    section_check::checksection_expecting,
    session::{Session, ViewStep},
    surface_intersection::{Surface, edge_of},
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;

/// `CheckOverlapIntCurves`, the procedure of the grid's `begin`, at its default tolerance.
fn check_overlap_int_curves(s: &mut dyn Session, answer: &SurfaceCurves) {
    for first in 0..answer.curves.len() {
        for second in first + 1..answer.curves.len() {
            let one = edge_of(s, "e1", &answer.curves[first]);
            let two = edge_of(s, "e2", &answer.curves[second]);
            let coe = checkoverlapedges(("e1", &one), ("e2", &two), 1.0e-7);
            if !coe.contains("Edges are not overlapped") {
                let (i, j) = (first + 1, second + 1);
                println!("Error: Curves {i} and {j} are overlapped");
            }
        }
    }
}

fn bug25950_checks(s: &mut dyn Session, answer: &SurfaceCurves, surfaces: [&Surface; 2]) {
    check_intersection_result(answer, surfaces, 100, [7.0e-5, 2.0e-4]);
    check_overlap_int_curves(s, answer);
    let mut bop = BopSession::default();
    for curve in &answer.curves {
        add_curve_edge(s, &mut bop, curve);
    }
    bop.bfillds();
    bop.bbuild(s, "result");
    checksection_expecting(s, "result", 0);
    checkmaxtol_within(s, "result", "2.0e-7");
    let expected = [NbCount::Vertices(4), NbCount::Edges(3)];
    checknbshapes(s, "result", Counting::Located, &expected);
    checkprops(s, "result", Prop::Length, Expected::Mass(37.721));
}

pub fn bug25950(s: &mut dyn Session) {
    println!("=========");
    println!("0025950: Bad performance of intersection algorithm.");
    println!("=========");
    println!();
    println!("CPU and elapsed time limit set to 200 seconds");
    println!("TODO 0025950 ALL: Error: 0 vertices are expected but 2 are found.");
    println!("TODO 0025950 ALL: Error : The length of result shape is 18.853");
    restore(s, "bug25950_b1.brep", "b1");
    restore(s, "bug25950_b2.brep", "b2");
    let (s1, s2) = (mksurface(s, "s1", "b1"), mksurface(s, "s2", "b2"));
    s.view(ViewStep::Donly(&["s1", "s2"]));
    let mut chrono = Chrono::started();
    let answer = intersect(s, "res", &s1, &s2, precision::confusion());
    chrono.stop();
    chrono.counter("Bug25950IntSS");
    chrono.show();
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Fit);
    screenshot(s, "photo_2d");
    let ic = answer.curves.len();
    if ic != 3 {
        println!("Error: 3 curves is expected but {ic} ones are found");
    } else {
        bug25950_checks(s, &answer, [&s1, &s2]);
    }
    checkview(s, &CheckView::of("result"));
}

pub fn bug30202_2(s: &mut dyn Session) {
    println!("========");
    println!("0030202: IntPatch_WLineTool::JoinWLines array out of bounds");
    println!("========");
    println!();
    restore(s, "bug30202_src.brep", "b1");
    restore(s, "bug30202_tool.brep", "b2");
    explode(s, "b1", Some(ShapeEnum::Face));
    explode(s, "b2", Some(ShapeEnum::Face));
    let log = bopcurves_bound(s, "b1_5", "b2_7", BopCurvesOptions::TWO_D);
    if !log.lines.is_empty() || !log.point_names.is_empty() {
        println!("Error: Expected intersection result is empty");
    }
}
