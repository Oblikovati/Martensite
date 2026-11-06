//! `bugs_moddata_2`: the `bopargcheck` scripts, each with one expected verdict line.
use crate::support::bugs_commands::view::donly;
use crate::support::{
    bugs_commands::{geometry::mksurface, inspect::bopargcheck},
    checkview::{CheckView, checkview},
    commands::{add, compound, explode, make_box},
    construction::{tcopy, wire},
    one_step::{bcommon, bcut},
    restore::restore,
    session::Session,
    surface_commands::mkface,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

const FAULTIES: &str = "Faulties, that can not be treated by BOP, are detected.\n";

fn ends(s: &mut dyn Session, verdict: &str) {
    if verdict == FAULTIES {
        println!("OK OCC6862");
    } else {
        println!("Faulty OCC6862");
    }
    checkview(s, &CheckView::of("result"));
}

fn boxes(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [10., 20., 10.]);
    make_box(s, "b2", [0., 0., 0.], [20., 10., 10.]);
}

pub fn bug6862_7(s: &mut dyn Session) {
    println!("============");
    println!("OCC6862");
    println!("============");
    println!();
    boxes(s);
    explode(s, "b1", Some(ShapeEnum::Face));
    explode(s, "b2", Some(ShapeEnum::Face));
    bcut(s, "r1", "b1_6", "b2_6");
    bcommon(s, "r2", "b1_6", "b2_6");
    explode(s, "r1", Some(ShapeEnum::Wire));
    explode(s, "r2", Some(ShapeEnum::Wire));
    let surface = mksurface(s, "b1_6");
    mkface(s, "ff", &surface);
    add(s, "r1_1", "ff");
    add(s, "r2_1", "ff");
    let verdict = bopargcheck(s, ("ff", Some("b2_6")), Some(Operation::Fuse), "RTVEIP");
    ends(s, &verdict);
}

pub fn bug6862_5(s: &mut dyn Session) {
    println!("============");
    println!("OCC6862");
    println!("============");
    println!();
    boxes(s);
    explode(s, "b2", Some(ShapeEnum::Face));
    let verdict = bopargcheck(s, ("b1", Some("b2_1")), Some(Operation::Cut), "RFTVEI");
    ends(s, &verdict);
}

fn box_edges(s: &mut dyn Session) {
    boxes(s);
    explode(s, "b1", Some(ShapeEnum::Edge));
    explode(s, "b2", Some(ShapeEnum::Edge));
    tcopy(s, "b2_10", "ee2");
}

pub fn bug6862_8(s: &mut dyn Session) {
    println!("============");
    println!("OCC6862");
    println!("============");
    println!();
    box_edges(s);
    compound(s, &["b2_10", "ee2"], "comp");
    donly(s, &["b1_10", "comp"]);
    let verdict = bopargcheck(
        s,
        ("b1_10", Some("comp")),
        Some(Operation::Common),
        "RFTEIP",
    );
    ends(s, &verdict);
}

pub fn bug6862_9(s: &mut dyn Session) {
    println!("============");
    println!("OCC6862");
    println!("============");
    println!();
    box_edges(s);
    wire(s, "ww", &["b2_10", "ee2"]);
    donly(s, &["b1_10", "ww"]);
    let verdict = bopargcheck(s, ("b1_10", Some("ww")), Some(Operation::Common), "RFTVIP");
    ends(s, &verdict);
}

fn restored_pair(s: &mut dyn Session, first: &str, second: &str) -> String {
    restore(s, first, "a_1");
    restore(s, second, "a_2");
    bopargcheck(s, ("a_2", None), None, "")
}

pub fn bug6862_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC6862");
    println!("============");
    println!();
    let verdict = restored_pair(s, "OCC497a.brep", "OCC497b.brep");
    ends(s, &verdict);
}

pub fn bug6862_2(s: &mut dyn Session) {
    println!("============");
    println!("OCC6862");
    println!("============");
    println!();
    let verdict = restored_pair(s, "OCC497e.brep", "OCC497f.brep");
    ends(s, &verdict);
}
