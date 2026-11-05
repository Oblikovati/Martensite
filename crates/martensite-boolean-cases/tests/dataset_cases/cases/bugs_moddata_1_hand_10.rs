//! `bugs_moddata_1` bodies written by hand, second file.
use crate::support::{
    bugs_commands::{
        geometry::{ellipse_along, mksurface},
        inspect::proj_on_surface,
    },
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, make_box},
    construction::{revol, wire},
    one_step::{bcommon, bcut, bfuse},
    properties::Prop,
    restore::restore,
    session::Session,
    surface_commands::mkedge,
    topology_commands::mkplane,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug17046(s: &mut dyn Session) {
    println!("============");
    println!("OCC17046");
    println!("============");
    println!();
    let ell = ellipse_along([0., 0., 300.], [1., 0., 0.], 150., 100.);
    mkedge(s, "ell_e", &ell);
    wire(s, "ell_w", &["ell_e"]);
    mkplane(s, "ell_f", "ell_w", true);
    make_box(s, "ell_t", [-10., -110., 140.], [20., 110., 460.]);
    bcut(s, "ell_d", "ell_f", "ell_t");
    revol(s, "ell_z", "ell_d", [0., 0., 0.], [0., 0., 1.], 360.);
    explode(s, "ell_z", Some(ShapeEnum::Face));
    let su1 = mksurface(s, "ell_z_1");
    proj_on_surface(&su1, [0., 100., 300.]);
    println!("OK OCC17046");
}

/// `bug160`: the operation between the two inputs, then the script's count of faces and its area.
fn faces_and_area(
    s: &mut dyn Session,
    operation: fn(&mut dyn Session, &str, &str, &str),
    first: &str,
    second: &str,
    area: f64,
) {
    restore(s, "buc60775a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60775b.brep", "b");
    checkshape_named(s, "b");
    operation(s, "result", first, second);
    checkshape_named(s, "result");
    if explode(s, "result", Some(ShapeEnum::Face)).len() < 6 {
        println!("Error : the resulting shape is unclosed!!!");
    }
    checkprops(s, "result", Prop::Area, Expected::Mass(area));
    checkview(s, &CheckView::of("result"));
}

pub fn bug160_1(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60775");
    println!("OCC160");
    println!("(COMMON: A and B)");
    println!("========================");
    faces_and_area(s, bcommon, "a", "b", 94067.7);
}

pub fn bug160_2(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60775");
    println!("OCC160");
    println!("(COMMON: B and A)");
    println!("========================");
    faces_and_area(s, bcommon, "b", "a", 94067.7);
}

pub fn bug160_3(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60775");
    println!("OCC160");
    println!("(FUSE: A and B)");
    println!("========================");
    faces_and_area(s, bfuse, "a", "b", 328116.);
}

pub fn bug160_4(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60775");
    println!("OCC160");
    println!("(FUSE: B and A)");
    println!("========================");
    faces_and_area(s, bfuse, "b", "a", 328116.);
}

pub fn bug160_5(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60775");
    println!("OCC160");
    println!("(CUT: A and B)");
    println!("========================");
    faces_and_area(s, bcut, "a", "b", 48129.6);
}

pub fn bug160_6(s: &mut dyn Session) {
    println!("TODO OCC12345 ALL: Error : the resulting shape is unclosed!!!");
    println!("========================");
    println!("BUC60775");
    println!("OCC160");
    println!("(CUT: B and A)");
    println!("========================");
    faces_and_area(s, bcut, "b", "a", 247636.);
}
