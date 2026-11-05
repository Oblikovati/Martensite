//! `bugs_moddata_3` bodies written by hand.
use crate::support::{
    bop_session::BopOptions,
    bugs_commands::inspect::bopcheck_text,
    checkview::{CheckView, checkview},
    commands::prism,
    restore::restore,
    session::Session,
};

pub fn bug25693_2(s: &mut dyn Session) {
    println!("============");
    println!("OCC25693");
    println!("============");
    println!();
    restore(s, "bug25693_path3039.brep", "path3039");
    prism(s, "Extrude_path3039", "path3039", [0., 0., 50.]);
    if bopcheck_text(s, "path3039", &BopOptions::default()).trim() != "This shape seems to be OK." {
        println!("Error : bad shape");
    }
    if bopcheck_text(s, "Extrude_path3039", &BopOptions::default()).trim()
        != "This shape seems to be OK."
    {
        println!("Error : Wire of BSplines fails bopcheck");
    }
    checkview(s, &CheckView::of("path3039"));
    checkview(s, &CheckView::of("Extrude_path3039"));
}
