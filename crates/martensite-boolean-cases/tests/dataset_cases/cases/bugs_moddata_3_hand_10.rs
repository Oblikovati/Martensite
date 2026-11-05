//! `bugs_moddata_3` bodies written by hand, second file.
use crate::support::{
    bop_session::BopOptions,
    bugs_commands::inspect::bopcheck_text,
    checkview::{CheckView, checkview},
    restore::restore,
    session::Session,
};

pub fn bug25693_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC25693");
    println!("============");
    println!();
    restore(s, "bug25693_path3035.brep", "result");
    if bopcheck_text(s, "result", &BopOptions::default()).trim() != "This shape seems to be OK." {
        println!("Error : Wire of BSplines fails bopcheck");
    }
    checkview(s, &CheckView::of("result"));
}
