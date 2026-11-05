//! `lowalgos_intss` scripts built on `bopcurves`, the last: `bug30354`, `bug32701`, `bug32849`.
use crate::support::{
    bopcurves::{BopCurvesOptions, BoundCurves, bopcurves_bound},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview, screenshot},
    commands::{compound, explode},
    lowalgos_commands::{SavedSurface, banner, checklength, donly_curves, restore_surface},
    one_step::{bcommon, bcut},
    properties::Prop,
    restore::restore,
    session::{Session, ViewStep},
    surface_commands::mkface,
    tolerance_check::checkreal,
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn bug30354_length(log: &BoundCurves) {
    match log.lines.first() {
        Some(line) => checklength("c_1", line.curve(), "0.0067028841369758101", 1.0e-4),
        None => println!("Error: The command cannot be built"),
    }
}

pub fn bug30354(s: &mut dyn Session) {
    banner(8, "0030354: BOP Cut doesn't modify the attached face");
    restore(s, "bug30354_Face_14.brep", "b1");
    restore(s, "bug30354_vol_neg.brep", "b2");
    explode(s, "b2", Some(ShapeEnum::Face));
    let log1 = bopcurves_bound(s, "b1", "b2_5", BopCurvesOptions::TWO_D);
    bug30354_length(&log1);
    let start = [
        [
            0.00464653890386874241,
            0.04500000000000001915,
            1.44140398677482628464,
            0.05759380964683875198,
        ],
        [
            0.00000000000000000000,
            0.04500000000000000527,
            1.44165603752761084522,
            0.05089567231303324113,
        ],
    ];
    let options = BopCurvesOptions {
        pcurves: true,
        start_points: &start,
    };
    let log2 = bopcurves_bound(s, "b1", "b2_5", options);
    bug30354_length(&log2);
    let (toler1, nb_curv1) = log1.reached_and_count();
    let (toler2, nb_curv2) = log2.reached_and_count();
    checkreal("ToleranceNoStartPoint", toler1, "1.0e-7", 0.0, 0.1);
    if nb_curv1 != 1 {
        println!("Error: Please check NbCurves for intersector w/o start points");
    }
    checkreal("ToleranceWithStartPoint", toler2, "1.0e-7", 0.0, 0.1);
    if nb_curv2 != 1 {
        println!("Error: Please check NbCurves for intersector with start points");
    }
    s.view(ViewStep::SmallView);
    donly_curves(s, &log2, &[]);
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["b1", "b2"]));
    screenshot(s, "photo");
}

fn bug32701_pair(s: &mut dyn Session, face: &str, cap: f64) {
    let log = bopcurves_bound(s, "s", face, BopCurvesOptions::TWO_D);
    let (_, nb_curv) = log.reached_and_count();
    if nb_curv != 1 {
        println!("Error: Number of curves is wrong");
    }
    if log.reached() > cap {
        println!("Error: Big tolerance value");
    }
}

pub fn bug32701(s: &mut dyn Session) {
    banner(
        8,
        "0032701: Modeling Algorithms - 2d curve has bending near the degenerated edge of the face",
    );
    restore(s, "bug32701s.brep", "s");
    restore(s, "bug32701t.brep", "t");
    explode(s, "t", Some(ShapeEnum::Face));
    bug32701_pair(s, "t_3", 3.0e-5);
    bug32701_pair(s, "t_4", 8.0e-5);
    bcommon(s, "cc", "s", "t");
    checkshape_named(s, "cc");
    checkprops(s, "cc", Prop::Area, Expected::Mass(19899.6));
    bcut(s, "cct", "s", "t");
    checkshape_named(s, "cct");
    checkprops(s, "cct", Prop::Area, Expected::Mass(32252.5));
    compound(s, &["cc", "cct"], "qq");
    checkview(s, &CheckView::of("qq"));
}

/// The script's conditions on what `bopcurves` answered: the printed tolerance is compared.
fn bug32849_problems(reached: &str, curves: usize) -> Vec<&'static str> {
    let tolerance: f64 = reached
        .parse()
        .unwrap_or_else(|cause| panic!("Tolerance Reached {reached:?}: {cause}"));
    let mut problems = Vec::new();
    if curves != 2 {
        problems.push("Error: Number of curves is wrong");
    }
    if tolerance > 1.0e-8 {
        problems.push("Error: Big tolerance value");
    }
    problems
}

pub fn bug32849(s: &mut dyn Session) {
    banner(
        8,
        "0032849: Modeling Algorithms - Intersection algorithm returns incomplete result",
    );
    let s1 = restore_surface(s, "bug32849_s1", "s1", SavedSurface::BSpline);
    let s2 = restore_surface(s, "bug32849_s2", "s2", SavedSurface::BSpline);
    mkface(s, "f1", &s1);
    mkface(s, "f2", &s2);
    let log = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::TWO_D);
    let (toler, nb_curv) = log.reached_and_count();
    for line in bug32849_problems(toler, nb_curv) {
        println!("{line}");
    }
}

#[test]
fn bug32849_conditions_sit_on_their_bounds() {
    assert!(bug32849_problems("1.0e-8", 2).is_empty());
    assert_eq!(
        bug32849_problems("0.0", 1),
        ["Error: Number of curves is wrong"]
    );
    assert_eq!(
        bug32849_problems("5.0e-8", 2),
        ["Error: Big tolerance value"]
    );
    let above = format!("{:e}", f64::from_bits(1.0e-8_f64.to_bits() + 1));
    assert_eq!(bug32849_problems(&above, 2), ["Error: Big tolerance value"]);
}
