//! `bugs_modalg_7`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    beyond_script::all_counts,
    bugs_commands::geometry::{line, mkedge_range},
    bugs_commands::inspect::bopcheck_ok,
    bugs_commands::options_bop::ScriptBop,
    bugs_commands::repair::{mkvolume, settolerance},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    commands::{compound, explode, make_box, ttranslate},
    construction::{copy, vertex},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bfuse,
    properties::Prop,
    restore::restore,
    session::Session,
    surface_commands::{mkedge, mkface, mkface_bounded, plane},
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug29322_2(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC29322");
    println!("========");
    println!();
    let var_line1 = line([0., 0., 0.], [0., 0., 1.]);
    mkedge_range(s, "e1", &var_line1, 0., 10.);
    let var_line2 = line([0., 0.5, 0.], [0., 0., 1.]);
    mkedge_range(s, "e2", &var_line2, 0., 10.);
    vertex(s, "v1", [0., 0.25, 0.]);
    settolerance(s, "v1", 0.25);
    vertex(s, "v2", [0., 0.25, 10.]);
    settolerance(s, "v2", 0.25);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["e1", "e2", "v1", "v2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(10.));
}

pub fn bug29322_3(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC29322");
    println!("========");
    println!();
    let var_line1 = line([0., 0., 0.], [0., 0., 1.]);
    mkedge_range(s, "e1", &var_line1, 0., 10.);
    let var_line2 = line([0., 0.5, 0.], [0., 0., 1.]);
    mkedge_range(s, "e2", &var_line2, 0., 10.);
    vertex(s, "v1", [0., 0.25, 0.]);
    settolerance(s, "v1", 0.25);
    vertex(s, "v2", [0., 0.25, 10.]);
    settolerance(s, "v2", 0.25);
    vertex(s, "v3", [0., 0.25, 5.]);
    settolerance(s, "v3", 0.25);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["e1", "e2", "v1", "v2", "v3"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(10.));
}

pub fn bug29322_4(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC29322");
    println!("========");
    println!();
    let var_line1 = line([0., 0., 0.], [0., 0., 1.]);
    mkedge_range(s, "e1", &var_line1, 0., 10.);
    let var_line2 = line([0., 0.5, 0.], [0., 0., 1.]);
    mkedge_range(s, "e2", &var_line2, 0., 10.);
    mkedge_range(s, "e3", &var_line2, -5., 15.);
    vertex(s, "v1", [0., 0.25, 0.]);
    settolerance(s, "v1", 0.25);
    vertex(s, "v2", [0., 0.25, 10.]);
    settolerance(s, "v2", 0.25);
    vertex(s, "v3", [0., 0.25, 5.]);
    settolerance(s, "v3", 0.25);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["e1", "e2", "e3", "v1", "v2", "v3"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(20.));
}

pub fn bug29322_5(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC29322");
    println!("========");
    println!();
    let var_line1 = line([0., 0., 0.], [0., 0., 1.]);
    mkedge_range(s, "e1", &var_line1, 0., 10.);
    let var_line2 = line([0., 0.5, 0.], [0., 0., 1.]);
    mkedge_range(s, "e2", &var_line2, 0., 10.);
    mkedge_range(s, "e3", &var_line2, -5., 15.);
    vertex(s, "v1", [0., 0.25, 0.]);
    settolerance(s, "v1", 0.25);
    vertex(s, "v2", [0., 0.25, 10.]);
    settolerance(s, "v2", 0.25);
    vertex(s, "v3", [0., 0.25, 5.]);
    settolerance(s, "v3", 0.25);
    let var_p = plane([0., 0., 0.], [0., 1., 0.]);
    mkface_bounded(s, "f", &var_p, (-5., 15., -5., 5.));
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["f", "e1", "e2", "e3", "v1", "v2", "v3"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(9),
            NbCount::Edges(8),
            NbCount::Wires(2),
            NbCount::Faces(1),
        ],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(90.));
    checkprops(s, "result", Prop::Area, Expected::Mass(200.));
    // Beyond the script: checks an earlier test of this case made.
    all_counts(s, "result", [9, 8, 2, 1, 0, 0, 0, 1, 21]);
}

pub fn bug29333_1(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC29333");
    println!("========");
    println!();
    let var_p = plane([0., 0., 0.], [0., 0., 1.]);
    mkface_bounded(s, "f1", &var_p, (-10., 10., -10., 10.));
    copy(s, "f1", "f2");
    ttranslate(s, "f2", [20., 0., 0.]);
    bfuse(s, "s", "f1", "f2");
    explode(s, "s", Some(ShapeEnum::Face));
    let var_l = line([0., 0., 0.], [1., 0., 0.]);
    mkedge(s, "e", &var_l);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s_1"]);
    bop.add_tools(s, &["e"]);
    bop.bfillds();
    bop.bsplit(s, "s1_sp");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s1_sp"]);
    bop.add_tools(s, &["s_2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(800.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(10),
            NbCount::Wires(3),
            NbCount::Faces(3),
        ],
    );
    compound(s, &["result", "s1_sp"], "c");
    checknbshapes(
        s,
        "c",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(10),
            NbCount::Wires(3),
            NbCount::Faces(3),
        ],
    );
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s_2"]);
    bop.add_tools(s, &["s1_sp"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(800.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(10),
            NbCount::Wires(3),
            NbCount::Faces(3),
        ],
    );
    compound(s, &["result", "s1_sp"], "c");
    checknbshapes(
        s,
        "c",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(10),
            NbCount::Wires(3),
            NbCount::Faces(3),
        ],
    );
}

pub fn bug29333_2(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC29333");
    println!("========");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [10., 0., 0.], [10., 10., 10.]);
    mkvolume(s, "s", &["b1", "b2"]);
    explode(s, "s", Some(ShapeEnum::Solid));
    let var_p = plane([0., 0., 5.], [0., 0., 1.]);
    mkface(s, "f", &var_p);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s_1"]);
    bop.add_tools(s, &["f"]);
    bop.bfillds();
    bop.bsplit(s, "s1_sp");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s1_sp"]);
    bop.add_tools(s, &["s_2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1400.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2000.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(28),
            NbCount::Wires(16),
            NbCount::Faces(16),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    compound(s, &["result", "s1_sp"], "c");
    checknbshapes(
        s,
        "c",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(28),
            NbCount::Wires(16),
            NbCount::Faces(16),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s_2"]);
    bop.add_tools(s, &["s1_sp"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1400.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(2000.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(28),
            NbCount::Wires(16),
            NbCount::Faces(16),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    compound(s, &["result", "s1_sp"], "c");
    checknbshapes(
        s,
        "c",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(28),
            NbCount::Wires(16),
            NbCount::Faces(16),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
}

pub fn bug29400(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC29400");
    println!("========");
    println!();
    restore(s, "bug29400_edges.brep", "le");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_compound(s, "le");
    bop.bfillds();
    bop.bbuild(s, "result");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(1.78839));
    bopcheck_ok(
        s,
        "result",
        &bop.options,
        "Error: the result is self-interfered",
    );
}
