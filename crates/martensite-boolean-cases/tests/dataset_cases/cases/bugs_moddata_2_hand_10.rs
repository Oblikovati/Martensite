//! `bugs_moddata_2` bodies written by hand, second file.
use crate::support::{
    bugs_commands::repair::halfspace_of,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_top,
    checkview::{CheckView, checkview},
    commands::{make_box, pcylinder, trotate, ttranslate},
    one_step::{bcommon, bfuse},
    properties::Prop,
    session::Session,
    surface_commands::{mkface, plane},
};

pub fn bug259(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC259 ");
    println!("========================");
    println!();
    pcylinder(s, "b1", 1., 2.);
    let r = (2f64).sqrt() / 2.;
    // What `dset r sqrt(2)/2` answers after its echoes.
    println!("1.4142135623730951\n0.70710678118654757");
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    bfuse(s, "result", "b2", "b1");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(18.8496));
    checkview(s, &CheckView::of("result"));
}

fn halfspaces(s: &mut dyn Session, rotated: bool) {
    let surface = plane([100., 0., 0.], [1., 0., 0.]);
    mkface(s, "f", &surface);
    halfspace_of(s, "so1", "f", [0., 0., 0.]);
    ttranslate(s, "f", [-200., 0., 0.]);
    if rotated {
        trotate(s, "f", [0., 0., 0.], [0., 0., 1.], 45.);
    }
    halfspace_of(s, "so2", "f", [0., 0., 0.]);
    bcommon(s, "result", "so1", "so2");
    if s.shape("result").is_none() {
        println!("Faulty OCC6450: operation failed");
    } else {
        println!("OK OCC6450");
    }
    checkview(s, &CheckView::of("result"));
}

pub fn bug6450_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC6450");
    println!("============");
    println!();
    halfspaces(s, false);
}

pub fn bug6450_2(s: &mut dyn Session) {
    println!("============");
    println!("OCC6450");
    println!("============");
    println!();
    halfspaces(s, true);
}
