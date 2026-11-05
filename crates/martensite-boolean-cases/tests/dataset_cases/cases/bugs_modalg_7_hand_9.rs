//! `bugs_modalg_7` bodies written by hand: history and read-back values.
#![allow(unused_imports)]
use crate::support::bugs_commands::progress::tracked;
use crate::support::{
    bop_session::BopOptions,
    bugs_commands::{
        cells::ScriptCells,
        geometry::{
            circle_framed, ellipse_framed, frame_along, frame_at, line, mkedge_range, pcylinder_on,
            plane_at, psphere_on, trim_curve,
        },
        inspect::{
            bopapicheck, bopargcheck, bopcheck_level, bopcheck_text, checkmaxtol_value,
            checksection_count, compare, whatis,
        },
        options_bop::{ScriptBop, SectionFlags, bsection_with, generated, modified},
        pcurves::{checklength, pcurve_of_edge, pcurves_of_face, translate_pcurve},
        repair::{fixshape, settolerance},
        view::{donly, fit, smallview},
    },
    checkprops::{Expected, checkprops, checkprops_equal},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview, screenshot},
    commands::{bound_shape, compound, explode, make_box, psphere, ttranslate},
    construction::{mkplane, revol, vertex, wire},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    one_step::{bcut, bsection},
    properties::{Prop, props},
    restore::restore,
    section_check::checksection_expecting,
    session::Session,
    surface_commands::{circle_along, mkedge, mkface, mkface_bounded, plane, polyline},
    tolerance_check::{checkmaxtol_ref, tolerance_report},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::{
    bop_algo::{
        BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoSection, BOPAlgoSplitter,
        BuilderHooks, Operation, perform_with_filler_progress,
    },
    brep_algo_api::{
        BRepAlgoAPIBuilderAlgo, BRepAlgoAPICheck, BRepAlgoAPICommon, BRepAlgoAPICut,
        BRepAlgoAPIFuse, BRepAlgoAPISection, BRepAlgoAPISplitter,
    },
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;

fn view_of(s: &mut dyn Session, name: &str) {
    checkview(s, &CheckView::of(name));
}

pub fn bug25462(s: &mut dyn Session) {
    println!(
        "============================================================================================="
    );
    println!("0025462: BRepAlgoAPIFuse not work correctly for a planar face");
    println!(
        "============================================================================================="
    );
    println!();
    polyline(
        s,
        "p",
        &[[0., 0., 0.], [0.01, 0., 0.], [0., 0.01, 0.], [0., 0., 0.]],
    );
    mkplane(s, "f1", "p");
    let e = ellipse_framed([0.01, 0.01, 0.], [0., 0., 1.], [1., 0., 0.], 0.01, 0.005);
    mkedge(s, "e", &e);
    wire(s, "w", &["e"]);
    mkplane(s, "f2", "w");
    let c = circle_framed([0., 0., 0.], [0., 0., 1.], [1., 0., 0.], 0.005);
    mkedge(s, "e", &c);
    wire(s, "w", &["e"]);
    mkplane(s, "f3", "w");
    polyline(
        s,
        "p",
        &[
            [-0.01, 0.005, 0.],
            [-0.01, 0., 0.],
            [0., 0., 0.],
            [0., 0.005, 0.],
            [-0.01, 0.005, 0.],
        ],
    );
    mkplane(s, "f4", "p");
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["f1"]);
    bop.add_tools(s, &["f2", "f3", "f4"]);
    bop.simplify_edges = true;
    bop.simplify_faces = true;
    bop.bapibop(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(0.000293167));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Wires(1), NbCount::Faces(1)],
    );
    bop.simplify_edges = false;
    bop.simplify_faces = false;
    bop.bapibop(s, "result1", Operation::Fuse);
    checkshape_named(s, "result1");
    checkprops(s, "result1", Prop::Area, Expected::Mass(0.000293167));
    checknbshapes(
        s,
        "result1",
        Counting::Located,
        &[NbCount::Wires(7), NbCount::Faces(7)],
    );
    view_of(s, "result");
}

/// The script's checks of each operation number and name against the one-step command's result.
fn same_counts_as(s: &dyn Session, names: &[&str], reference: &str) {
    for name in names {
        checknbshapes_like(s, name, reference);
    }
}

pub fn bug30206(s: &mut dyn Session) {
    println!("============================================");
    println!("0030206: Improve API of commands bbop and bapibop");
    println!("============================================");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [5., 5., 5.], [10., 10., 10.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    let groups: [(&str, Operation, &[&str], &[&str]); 4] = [
        (
            "r0",
            Operation::Common,
            &["r0_1", "r0_2"],
            &["r0_3", "r0_4"],
        ),
        ("r1", Operation::Fuse, &["r1_1", "r1_2"], &["r1_3", "r1_4"]),
        ("r2", Operation::Cut, &["r2_1", "r2_2"], &["r2_3", "r2_4"]),
        (
            "r3",
            Operation::Cut21,
            &["r3_1", "r3_2", "r3_3"],
            &["r3_4", "r3_5", "r3_6"],
        ),
    ];
    for (reference, kind, prepared, api) in groups {
        bop.direct(s, reference, ("b1", "b2"), kind);
        for name in prepared {
            bop.bbop(s, name, kind);
        }
        for name in api {
            bop.bapibop(s, name, kind);
        }
        same_counts_as(s, prepared, reference);
        same_counts_as(s, api, reference);
    }
    bsection(s, "r4", "b1", "b2");
    for name in ["r4_1", "r4_2"] {
        bop.bbop(s, name, Operation::Section);
    }
    for name in ["r4_3", "r4_4"] {
        bop.bapisection(s, name);
    }
    same_counts_as(s, &["r4_1", "r4_2", "r4_3", "r4_4"], "r4");
    // `bbop r 5` and `bbop r f` answer that the type is not valid; nothing else runs.
    println!("Invalid operation type\n");
    println!("Invalid operation type\n");
}

pub fn bug29580_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC29580");
    println!("========");
    println!();
    let first = circle_along([0., -5., 0.], [1., 0., 0.], 10.);
    let second = circle_along([0., 5., 0.], [1., 0., 0.], 10.);
    mkedge(s, "e1", &first);
    mkedge(s, "e2", &second);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    bcut(s, "f12", "f1", "f2");
    revol(s, "b1", "f12", [0., 0., 0.], [0., 0., 1.], 180.);
    ttranslate(s, "b1", [0., 31.358955689999998, 0.]);
    let frame = crate::support::bugs_commands::geometry::frame_framed(
        [0., 0., 0.],
        [0., 0., 1.],
        [1.1102230246251565e-016, -1., 0.],
    );
    psphere_on(s, "b2", frame, 8.6602540399999999, Some(180.));
    ttranslate(s, "b2", [0., 31.358955689999998, 0.]);
    let mut bop = ScriptBop::default();
    bop.add_objects(s, &["b1"]);
    bop.add_tools(s, &["b2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    let counts = [
        NbCount::Vertices(4),
        NbCount::Edges(16),
        NbCount::Wires(10),
        NbCount::Faces(10),
        NbCount::Shells(3),
        NbCount::Solids(3),
    ];
    checknbshapes(s, "result", Counting::Typed, &counts);
    checkprops(s, "result", Prop::Area, Expected::Mass(3879.55));
    checkprops(s, "result", Prop::Volume, Expected::Mass(6295.15));
    view_of(s, "result");
}

pub fn bug29586(s: &mut dyn Session) {
    println!("========");
    println!("OCC29586");
    println!("========");
    println!();
    let first = circle_along([0., -5., 0.], [1., 0., 0.], 10.);
    let second = circle_along([0., 5., 0.], [1., 0., 0.], 10.);
    mkedge(s, "e1", &first);
    mkedge(s, "e2", &second);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    bcut(s, "f12", "f1", "f2");
    let history = crate::support::bugs_commands::repair::revol_with_history(
        s,
        "r",
        "f12",
        [0., 0., 1.],
        180.,
    );
    explode(s, "r", Some(ShapeEnum::Face));
    for face in ["r_1", "r_2", "r_3"] {
        explode(s, face, Some(ShapeEnum::Edge));
    }
    for (first, second) in [("r_1_1", "r_2_1"), ("r_3_2", "r_2_2")] {
        if compare(s, first, second).contains("same shapes") {
            println!("Error");
        }
    }
    explode(s, "f12", Some(ShapeEnum::Vertex));
    for (name, vertex, edges) in [("g1", "f12_1", 2), ("g2", "f12_2", 1), ("g3", "f12_3", 2)] {
        generated(s, name, &history, vertex);
        checknbshapes(s, name, Counting::Located, &[NbCount::Edges(edges)]);
    }
}

pub fn bug31470(s: &mut dyn Session) {
    println!("=================================================================");
    println!("OCC31470: BOP common produces empty result (box and holed sphere)");
    println!("=================================================================");
    println!();
    let frame = frame_at([51.899912462, 99.996691888, 62.33204004]);
    psphere_on(s, "s1", frame, 15., None);
    psphere_on(s, "s2", frame, 10., None);
    bcut(s, "ss", "s1", "s2");
    make_box(s, "bb", [0., 0., 0.], [100., 100., 100.]);
    let mut bop = ScriptBop::default();
    bop.direct(s, "result", ("bb", "ss"), Operation::Common);
    let history = bop
        .saved_history()
        .expect("savehistory: expected a history");
    explode(s, "ss", Some(ShapeEnum::Face));
    modified(s, "ff", &history, "ss_1");
    generated(s, "ee", &history, "ss_1");
    checkshape_named(s, "result");
    let counts = [
        NbCount::Vertices(6),
        NbCount::Edges(8),
        NbCount::Wires(6),
        NbCount::Faces(3),
        NbCount::Shells(1),
        NbCount::Solids(1),
    ];
    checknbshapes(s, "result", Counting::Located, &counts);
    if checkmaxtol_value(s, "result") > 2.0e-7 {
        println!("Error: bad tolerance of result");
    }
    checkprops(s, "result", Prop::Volume, Expected::Mass(4975.49));
    smallview(s);
    pcurve_of_edge(s, "ee", "ff");
    screenshot(s, "photo");
    // Beyond the script: checks an earlier test of this case made.
    crate::support::beyond_script::all_counts(s, "result", [6, 8, 6, 3, 1, 1, 0, 1, 26]);
}

pub fn bug31404(s: &mut dyn Session) {
    println!("=======================================================");
    println!(
        "0031404: Modeling Algorithms - BOP Fuse produces a self-interfering or a good shape depending on the order of arguments"
    );
    println!("=======================================================");
    println!();
    psphere(s, "sph1", 25.);
    let frame = frame_at([32., 68., -27.]);
    psphere_on(s, "sph2", frame, 75., None);
    let mut bop = ScriptBop::default();
    bop.direct(s, "f1", ("sph1", "sph2"), Operation::Fuse);
    let first = bop.saved_history().expect("savehistory h1");
    bop.direct(s, "f2", ("sph2", "sph1"), Operation::Fuse);
    let second = bop.saved_history().expect("savehistory h2");
    explode(s, "sph1", Some(ShapeEnum::Face));
    modified(s, "ff1", &first, "sph1_1");
    modified(s, "ff2", &second, "sph1_1");
    for name in ["f1", "f2"] {
        checkshape_named(s, name);
        if !bopcheck_text(s, name, &BopOptions::default()).contains("OK") {
            println!("Error: the {name} is a self-interfering shape");
        }
        if checkmaxtol_value(s, name) > 2.0e-7 {
            println!("Error: {name} has bad tolerance");
        }
    }
    let counts = [
        NbCount::Vertices(6),
        NbCount::Edges(9),
        NbCount::Wires(3),
        NbCount::Faces(2),
    ];
    checknbshapes(s, "f1", Counting::Located, &counts);
    checknbshapes(s, "f2", Counting::Located, &counts);
    checkprops(s, "f1", Prop::Area, Expected::Mass(74192.6));
    checkprops(s, "f2", Prop::Area, Expected::Mass(74192.6));
    smallview(s);
    let curves_one = pcurves_of_face(s, "ff1");
    let curves_two = pcurves_of_face(s, "ff2");
    let moved = translate_pcurve(&curves_two[5], std::f64::consts::PI, 0.);
    checklength("ff1_6", &curves_one[5], 7.167);
    checklength("ff2_6", &moved, 7.167);
    screenshot(s, "photo");
}
