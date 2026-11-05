//! `lowalgos_intss` bodies written by hand: the scripts read back what `bopcurves` answers.
use crate::support::{
    bopcurves::bopcurves,
    checkview::screenshot,
    commands::explode,
    construction::copy,
    restore::restore,
    session::{Session, ViewStep},
    tolerance_check::checkreal,
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn reached_text(answer: &crate::support::bopcurves::BopCurves) -> &str {
    answer
        .tolerance_reached
        .as_deref()
        .unwrap_or_else(|| panic!("regexp Tolerance Reached: no such line in {answer:?}"))
}

pub fn bug26684_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC26684");
    println!("============");
    restore(s, "CTO909_IR2_TuyoPipe20.brep", "b1");
    restore(s, "CTO909_IR2_sphere1_20.brep", "b2");
    explode(s, "b1", Some(ShapeEnum::Face));
    copy(s, "b1_1", "f1");
    explode(s, "b2", Some(ShapeEnum::Face));
    copy(s, "b2_1", "f2");
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&["f1", "f2"]));
    s.view(ViewStep::Fit);
    let spatial = bopcurves(s, "f1", "f2", false);
    let reached = reached_text(&spatial);
    checkreal(
        "Reached tolerance",
        reached,
        "5.8654166482879483e-009",
        1.0e-7,
        0.,
    );
    let planar = bopcurves(s, "f1", "f2", true);
    let reached = reached_text(&planar);
    checkreal(
        "Reached tolerance",
        reached,
        "1.4915699300398263e-07",
        1.0e-7,
        0.,
    );
    screenshot(s, "photo");
}

pub fn bug25820_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC25820");
    println!("========");
    restore(s, "OCC25820-Revolution_5.brep", "b1");
    explode(s, "b1", Some(ShapeEnum::Face));
    copy(s, "b1_4", "b1");
    restore(s, "OCC25820-Translation_1.brep", "b2");
    explode(s, "b2", Some(ShapeEnum::Face));
    copy(s, "b2_2", "b2");
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Clear);
    s.view(ViewStep::Disp(&["b1"]));
    s.view(ViewStep::Disp(&["b2"]));
    s.view(ViewStep::Fit);
    let answer = bopcurves(s, "b1", "b2", false);
    for line in occ25820_problems(reached_text(&answer), answer.curves) {
        println!("{line}");
    }
    screenshot(s, "photo");
}

/// The script's two conditions on what `bopcurves` answered.
fn occ25820_problems(reached: &str, curves: usize) -> Vec<String> {
    let tolerance: f64 = reached
        .parse()
        .unwrap_or_else(|cause| panic!("Tolerance Reached {reached:?}: {cause}"));
    let mut problems = Vec::new();
    if tolerance > 1.0e-7 {
        problems.push(format!(
            "ERROR: OCC25820 is reproduced. Tolerance is to large ({reached})."
        ));
    }
    if curves != 1 {
        problems.push("ERROR: OCC25820 is reproduced. Too many curves were found.".to_owned());
    }
    problems
}

#[test]
fn occ25820_conditions_sit_exactly_on_their_bounds() {
    let cap = 1.0e-7_f64;
    let just_over = f64::from_bits(cap.to_bits() + 1);
    assert!(occ25820_problems(&format!("{cap:e}"), 1).is_empty());
    assert_eq!(occ25820_problems(&format!("{just_over:e}"), 1).len(), 1);
    assert_eq!(occ25820_problems(&format!("{cap:e}"), 0).len(), 1);
    assert_eq!(occ25820_problems(&format!("{cap:e}"), 2).len(), 1);
}
