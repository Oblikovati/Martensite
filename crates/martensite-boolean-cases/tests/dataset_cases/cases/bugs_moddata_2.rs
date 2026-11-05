//! `bugs_moddata_2`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::{checkshape_named, checkshape_top},
    checkview::{CheckView, checkview},
    commands::make_box,
    construction::nurbsconvert,
    one_step::{bcommon, bsection},
    properties::Prop,
    restore::restore,
    session::Session,
    surface_commands::mkface_bounded,
    topology_commands::plane_along,
};

pub use super::bugs_moddata_2_hand_2::*;
pub use super::bugs_moddata_2_hand_9::*;
pub use super::bugs_moddata_2_hand_10::*;

pub fn bug380_1(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC380 ");
    println!("========================");
    println!();
    restore(s, "OCC380a.rle", "a");
    restore(s, "OCC380b.rle", "b");
    bcommon(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(21.154));
    checkview(s, &CheckView::of("result"));
}

pub fn bug380_2(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC380 ");
    println!("========================");
    println!();
    restore(s, "OCC380a.rle", "a");
    restore(s, "OCC380b.rle", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Area, Expected::Mass(21.154));
    checkview(s, &CheckView::of("result"));
}

pub fn bug408(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC408 ");
    println!("========================");
    println!();
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [1.5, 0.5, 0.5]);
    bcommon(s, "result", "b1", "b2");
    checkshape_top(s, "result");
}

pub fn bug75_1(s: &mut dyn Session) {
    println!("============");
    println!("BUC60861");
    println!("OCC75");
    println!("============");
    println!();
    println!("(case 1)");
    restore(s, "BUC60861_gap.brep", "ss");
    checkshape_named(s, "ss");
    let geom_pl1 = plane_along([25680.2288, 21428., 9995.], [1., 0., 0.]);
    mkface_bounded(s, "f1", &geom_pl1.surface, (-4000., 8000., -5000., 5000.));
    bsection(s, "result", "ss", "f1");
    checkprops(s, "result", Prop::Length, Expected::Mass(9265.));
    checkview(s, &CheckView::of("result"));
}
