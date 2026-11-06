//! `bugs_modalg_6`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview, screenshot},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    properties::Prop,
    restore::restore,
    session::{Session, ViewStep},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub use super::bugs_modalg_6_hand::*;
pub use super::bugs_modalg_6_hand_2::*;
pub use super::bugs_modalg_6_hand_3::*;
pub use super::bugs_modalg_6_hand_9::*;
pub use super::bugs_modalg_6_hand_10::*;
pub use super::bugs_modalg_6_hand_11::*;

pub fn bug21427(s: &mut dyn Session) {
    println!("============");
    println!("OCC21427");
    println!("============");
    println!();
    restore(s, "bug21427a.brep", "shape");
    restore(s, "bug21427b.brep", "tool");
    bcut(s, "result", "shape", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.49188e+007));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug26923(s: &mut dyn Session) {
    println!("========");
    println!("OCC26923");
    println!("========");
    println!();
    s.view(ViewStep::SmallView);
    restore(s, "bug26923-resultCutted_9_Edge.brep", "a");
    restore(s, "bug26923-toolToCut_10.brep", "b");
    bcut(s, "result", "a", "b");
    checkshape_named(s, "result");
    s.view(ViewStep::Donly(&["result"]));
    s.view(ViewStep::Fit);
    screenshot(s, "photo");
}

pub fn bug28773(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========");
    println!("OCC28773");
    println!("========");
    println!();
    restore(s, "bug28773_1.brep", "b1");
    restore(s, "bug28773_2.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "res", Operation::Cut);
    checkshape_named(s, "res");
    checkshape_named(s, "res");
    checknbshapes(s, "res", Counting::Located, &[NbCount::Solids(1)]);
    checkprops(s, "res", Prop::Area, Expected::Mass(0.101938));
    checkprops(s, "res", Prop::Volume, Expected::Mass(0.000203073));
    checkview(s, &CheckView::of("res"));
}
