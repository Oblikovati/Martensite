//! `bugs_moddata_1` bodies written by hand.
use crate::support::{
    checkview::{CheckView, checkview},
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};

pub fn bug22703(s: &mut dyn Session) {
    println!("===========");
    println!("OCC22703");
    println!("===========");
    println!();
    println!("Load first shape ...");
    restore(s, "OCC22703-faceAC.brep", "b1");
    println!("Load second shape ...");
    restore(s, "OCC22703-pln.brep", "b2");
    println!("Prepare boolean operation ...");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "b1", "b2");
    println!("OCC22703  OK");
    checkview(s, &CheckView::of("result"));
}
