//! `bugs_modalg_2` bodies written by hand: the scripts with a variable or a read-back value.
use crate::support::bugs_commands::geometry::plane_at;
use crate::support::{
    bugs_commands::inspect::statshape_has_vertex,
    checkprops::{Expected, checkprops},
    checkshape::{checkshape_named, checkshape_top},
    checkview::{CheckView, checkview},
    one_step::{bcut, bsection},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::mkface,
};

pub fn bug415(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC415 ");
    println!("PRO5557");
    println!("========================");
    println!();
    restore(s, "pro5557_revol.brep", "revol1");
    checkshape_named(s, "revol1");
    restore(s, "pro5557_bsurf.brep", "bsurf2");
    checkshape_named(s, "bsurf2");
    println!("\nrouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL");
    crate::support::bugs_commands::repair::halfspace_of(s, "space2", "bsurf2", [0., -0.3, 0.]);
    bcut(s, "result", "revol1", "space2");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(0.402138));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

fn section_at(s: &mut dyn Session, result: &str, z: f64) {
    let plane = plane_at([0., 0., z]);
    mkface(s, "f", &plane);
    bsection(s, result, "a", "f");
    if !statshape_has_vertex(s, result) {
        println!("OCC526 FAULTY Z={z:.6}");
    }
}

pub fn bug526(s: &mut dyn Session) {
    println!("========");
    println!("OCC526");
    println!("========");
    println!();
    restore(s, "OCC592.brep", "a");
    section_at(s, "r1", 67.599311020481764);
    section_at(s, "r2", 66.599311020481764);
    section_at(s, "result", 65.599311020481764);
    checkprops(s, "result", Prop::Length, Expected::Mass(27.8021));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}
