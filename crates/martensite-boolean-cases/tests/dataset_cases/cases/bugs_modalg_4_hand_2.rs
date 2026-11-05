//! `bugs_modalg_4`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub fn bug620_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========================");
    println!("OCC620");
    println!("========================");
    println!();
    restore(s, "OCC620a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "OCC620b.brep", "b");
    checkshape_named(s, "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    println!("OCC620 OK: BOPCUT operation works OK");
    checkprops(s, "result", Prop::Area, Expected::Mass(314742.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug620_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========================");
    println!("OCC620");
    println!("========================");
    println!();
    restore(s, "OCC620a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "OCC620b.brep", "b");
    checkshape_named(s, "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    println!("OCC620 OK: BOPTUC operation works OK");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.15562e+06));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}
