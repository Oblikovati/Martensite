//! `bugs_modalg_5` bodies written by hand, third file.
use crate::support::{
    bop_session::BopOptions,
    bugs_commands::inspect::bopcheck_text,
    checkprops::{Expected, checkprops},
    commands::explode,
    one_step::bsection,
    properties::Prop,
    restore::restore,
    section_check::checksection_expecting,
    session::Session,
};

pub fn bug24519(s: &mut dyn Session) {
    println!("==========");
    println!("OCC24519");
    println!("==========");
    println!();
    restore(s, "bug24519_cylinder.brep", "cy");
    restore(s, "bug24519_plane.brep", "pl");
    bsection(s, "r", "cy", "pl");
    if explode(s, "r", None) != ["r_1"] {
        println!("Error: First intersection is not returned");
    }
    checksection_expecting(s, "r", 2);
    checkprops(s, "r", Prop::Length, Expected::Mass(3.83537));
}

pub fn bug24823(s: &mut dyn Session) {
    println!("============");
    println!("OCC24823");
    println!("============");
    println!();
    restore(s, "buc60555d.brep", "b");
    let log = bopcheck_text(s, "b", &BopOptions::default());
    // Beyond the script: checks an earlier test of this case made.
    assert!(
        !log.contains("Warning"),
        "bopcheck b: expected no warning, got {log:?}"
    );
}
