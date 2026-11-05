//! `tests/heal/unify_same_domain`: same-domain faces and edges merged, with edges kept on request.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    heal_commands::{
        UnifyOptions, isdeleted, modified, pcylinder_wedge, savehistory, setflags_locked,
        unifysamedom,
    },
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    session::Session,
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn occ_banner(title: &str) {
    println!("==========");
    println!("{title}");
    println!("==========");
    println!();
}

fn short_banner(title: &str) {
    println!("=======");
    println!("{title}");
    println!("=======");
    println!();
}

fn counts(vertices: usize, edges: usize, faces: usize) -> [NbCount; 3] {
    [
        NbCount::Vertices(vertices),
        NbCount::Edges(edges),
        NbCount::Faces(faces),
    ]
}

fn checked_counts(s: &mut dyn Session, name: &str, vertices: usize, edges: usize, faces: usize) {
    checknbshapes(s, name, Counting::Located, &counts(vertices, edges, faces));
}

/// `explode a f`, `explode a_1 v`, `explode a e`: the names `A1`, `A4` and `A6` read.
fn exploded_face_fixture(s: &mut dyn Session) {
    restore(s, "bug28228_face.brep", "a");
    explode(s, "a", Some(ShapeEnum::Face));
    explode(s, "a_1", Some(ShapeEnum::Vertex));
    explode(s, "a", Some(ShapeEnum::Edge));
}

pub fn A1(s: &mut dyn Session) {
    occ_banner("OCC28228");
    exploded_face_fixture(s);
    let plain = UnifyOptions::default();
    unifysamedom(s, "result1", "a", &["a_1_3", "a_1_5"], plain);
    checked_counts(s, "result1", 10, 10, 1);
    unifysamedom(s, "result2", "a", &["a_3", "a_9"], plain);
    checked_counts(s, "result2", 12, 14, 3);
    unifysamedom(s, "result3", "a", &["a_3", "a_9", "a_1_9", "a_1_10"], plain);
    checked_counts(s, "result3", 13, 15, 3);
    unifysamedom(s, "result4", "a", &["a_3", "a_1_3", "a_1_4"], plain);
    checked_counts(s, "result4", 11, 12, 2);
    unifysamedom(s, "result5", "a", &["a_3", "a_1_3"], plain);
    checked_counts(s, "result5", 10, 11, 2);
    let internal = UnifyOptions {
        allow_internal: true,
        ..plain
    };
    unifysamedom(s, "result", "a", &["a_3", "a_9"], internal);
    checked_counts(s, "result", 12, 12, 1);
    checkview(s, &CheckView::of("result"));
}

pub fn A2(s: &mut dyn Session) {
    occ_banner("OCC28228");
    restore(s, "bug28228_face_circle.brep", "a");
    explode(s, "a", Some(ShapeEnum::Edge));
    let plain = UnifyOptions::default();
    unifysamedom(s, "result1", "a", &["a_5"], plain);
    checked_counts(s, "result1", 6, 9, 3);
    unifysamedom(s, "result", "a", &[], plain);
    checked_counts(s, "result", 2, 2, 1);
    checkview(s, &CheckView::of("result"));
}

pub fn A3(s: &mut dyn Session) {
    occ_banner("OCC28228");
    restore(s, "bug28228_unify_with_keep.brep", "a");
    explode(s, "a", Some(ShapeEnum::Edge));
    unifysamedom(s, "result", "a", &["a_13"], UnifyOptions::default());
    checked_counts(s, "result", 16, 19, 4);
    checkview(s, &CheckView::of("result"));
}

fn modified_answer(s: &mut dyn Session, history: &H, name: &str, wanted: &str) {
    if modified(s, "res", history, name) != wanted {
        println!("ERROR: OCC28226 is reproduced. Command modified does not work correctly.");
    }
}

fn deleted_answer(s: &dyn Session, history: &H, name: &str, wanted: &str, command: &str) {
    if isdeleted(s, history, name) != wanted {
        println!("ERROR: OCC28226 is reproduced. Command {command} does not work correctly.");
    }
}

type H = martensite_tkernel::standard::Handle<martensite_tkbrep::brep_tools::History>;

pub fn A4(s: &mut dyn Session) {
    occ_banner("OCC28226");
    exploded_face_fixture(s);
    unifysamedom(
        s,
        "result",
        "a",
        &["a_3", "a_1_3", "a_1_4"],
        UnifyOptions::default(),
    );
    let history = savehistory();
    modified_answer(s, &history, "a_9", "The shape has not been modified.");
    modified_answer(s, &history, "a_10", "");
    modified_answer(s, &history, "a_11", "");
    modified_answer(s, &history, "a_3", "The shape has not been modified.");
    deleted_answer(s, &history, "a_3", "Not deleted.", "isdeleted");
    deleted_answer(s, &history, "a_9", "Deleted.", "isdeleted");
    deleted_answer(s, &history, "a_1_8", "Deleted.", "isdeleted");
    deleted_answer(s, &history, "a_1_9", "Deleted.", "isdeleted");
    deleted_answer(s, &history, "a_10", "Not deleted.", "isdeleted");
    deleted_answer(s, &history, "a_11", "Not deleted.", "isdeleted");
}

pub fn A5(s: &mut dyn Session) {
    println!("==========");
    println!("OCC28227");
    println!("==========");
    println!();
    restore(s, "bug28228_face.brep", "a");
    setflags_locked(s, "a");
    unifysamedom(s, "result", "a", &[], UnifyOptions::default());
}

pub fn A6(s: &mut dyn Session) {
    occ_banner("OCC28226");
    exploded_face_fixture(s);
    let faces_only = UnifyOptions {
        unify_edges: false,
        ..UnifyOptions::default()
    };
    unifysamedom(s, "result", "a", &["a_3", "a_1_3", "a_1_4"], faces_only);
    let history = savehistory();
    deleted_answer(s, &history, "a_1_8", "Not deleted.", "unifysamedomisdel");
    deleted_answer(s, &history, "a_1_9", "Deleted.", "unifysamedomisdel");
}

fn unified_fixture(s: &mut dyn Session, counted: [NbCount; 4], area: f64) {
    checkshape_named(s, "result");
    checknbshapes(s, "result", Counting::Located, &counted);
    checkprops(s, "result", Prop::Area, Expected::Mass(area));
    checkview(s, &CheckView::of("result"));
}

fn four_counts(vertices: usize, edges: usize, wires: usize, faces: usize) -> [NbCount; 4] {
    [
        NbCount::Vertices(vertices),
        NbCount::Edges(edges),
        NbCount::Wires(wires),
        NbCount::Faces(faces),
    ]
}

pub fn A9(s: &mut dyn Session) {
    short_banner("0028913");
    restore(s, "bug28913_26219_model.brep", "a");
    unifysamedom(s, "result", "a", &[], UnifyOptions::default());
    unified_fixture(s, four_counts(76, 124, 51, 44), 1473.4);
}

pub fn B1(s: &mut dyn Session) {
    short_banner("0028913");
    restore(s, "bug28913_26219_faces.brep", "a");
    unifysamedom(s, "result", "a", &[], UnifyOptions::default());
    unified_fixture(s, four_counts(2, 3, 1, 1), 94.2478);
}

pub fn B2(s: &mut dyn Session) {
    short_banner("0028913");
    restore(s, "bug28913_face.brep", "a");
    explode(s, "a", Some(ShapeEnum::Edge));
    unifysamedom(
        s,
        "result",
        "a",
        &["a_6", "a_9", "a_11"],
        UnifyOptions::default(),
    );
    unified_fixture(s, four_counts(4, 5, 2, 2), 651.238);
}

pub fn B3(s: &mut dyn Session) {
    short_banner("0029027");
    pcylinder_wedge(s, "a", 10., 30., 180.);
    unifysamedom(s, "result", "a", &[], UnifyOptions::default());
    unified_fixture(s, four_counts(4, 6, 4, 4), 1856.64);
}
