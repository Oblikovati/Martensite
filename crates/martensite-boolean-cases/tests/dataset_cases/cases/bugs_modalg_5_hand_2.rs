//! `bugs_modalg_5`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::beyond::{alone_count, child_count},
    bugs_commands::geometry::{cylinder, ellipse_framed, sphere},
    bugs_commands::options_bop::ScriptBop,
    bugs_commands::repair::mkvolume,
    bugs_commands::view::fit,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{add, explode, make_box, orientation, prism, ttranslate},
    construction::{copy, mkplane, vertex, wire},
    named_shapes::renamevar,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::{circle, mkedge, mkface, polyline, trimv},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug22027(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("=========");
    println!("OCC22027");
    println!("=========");
    println!();
    let var_c = circle([0., 0., 0.], 25.);
    let var_e = ellipse_framed([0., 0., 0.], [0., 0., 1.], [1., 0., 0.], 25., 10.);
    mkedge(s, "c", &var_c);
    mkedge(s, "e", &var_e);
    wire(s, "w1", &["c"]);
    wire(s, "w2", &["e"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    two_step.bop(s, "f1", "f2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(1178.1));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(2),
            NbCount::Edges(4),
            NbCount::Wires(2),
            NbCount::Faces(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug23402(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC23402");
    println!("============");
    println!();
    polyline(
        s,
        "l",
        &[
            [0., 0., 0.],
            [0., 5., 0.],
            [0., 5., 5.],
            [0., 3., 5.],
            [0., 3., 3.],
            [0., 2., 3.],
            [0., 2., 5.],
            [0., 0., 5.],
            [0., 0., 0.],
        ],
    );
    mkplane(s, "sf", "l");
    prism(s, "s", "sf", [5., 0., 0.]);
    polyline(
        s,
        "w1",
        &[
            [-2., 2.5, 2.],
            [7., 2.5, 2.],
            [7., 7., 2.],
            [-2., 7., 2.],
            [-2., 2.5, 2.],
        ],
    );
    mkplane(s, "f1", "w1");
    two_step.bop(s, "s", "f1");
    two_step.result(s, "c1", Operation::Common);
    ttranslate(s, "c1", [0., 0., 5.]);
    polyline(
        s,
        "w2",
        &[
            [-2., 2.5, 3.],
            [7., 2.5, 3.],
            [7., 7., 3.],
            [-2., 7., 3.],
            [-2., 2.5, 3.],
        ],
    );
    mkplane(s, "f2", "w2");
    two_step.bop(s, "s", "f2");
    two_step.result(s, "c2", Operation::Common);
    ttranslate(s, "c2", [0., 0., 5.]);
    polyline(
        s,
        "w3",
        &[
            [-2., 2.5, 4.],
            [7., 2.5, 4.],
            [7., 7., 4.],
            [-2., 7., 4.],
            [-2., 2.5, 4.],
        ],
    );
    mkplane(s, "f3", "w3");
    two_step.bop(s, "s", "f3");
    two_step.result(s, "c3", Operation::Common);
    ttranslate(s, "c3", [0., 0., 5.]);
    checkprops(s, "c1", Prop::Area, Expected::Mass(12.5));
    checkprops(s, "c2", Prop::Area, Expected::Mass(12.5));
    checkprops(s, "c3", Prop::Area, Expected::Mass(10.));
    fit(s);
    checkview(s, &CheckView::of("result"));
}

pub fn bug23998(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC23998");
    println!("============");
    println!();
    restore(s, "bug23998_m1s.brep", "b");
    explode(s, "b", None);
    two_step.bop(s, "b_1", "b_2");
    two_step.section(s, "result");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(67.026));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(1),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(3),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "b", 2);
    alone_count(s, "result", 0);
}

pub fn bug24060(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("================");
    println!("OCC24060");
    println!("================");
    println!();
    polyline(
        s,
        "p",
        &[
            [0., 0., 0.],
            [0., 0., 1.],
            [1., 0., 1.],
            [1., 0., 0.],
            [0., 0., 0.],
        ],
    );
    mkplane(s, "b1", "p");
    vertex(s, "b2", [0., 0., 1.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b1", "b2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checknbshapes(s, "result", Counting::Located, &[NbCount::Vertices(4)]);
    checkview(s, &CheckView::of("result"));
}

pub fn bug24597(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=========");
    println!("OCC24597");
    println!("=========");
    println!();
    let var_c1 = cylinder(20.);
    let var_c11 = trimv(&var_c1, 0., 50.);
    mkface(s, "b1", &var_c11);
    vertex(s, "v1", [0., 20., 25.]);
    orientation(s, "v1", Orientation::Internal);
    add(s, "v1", "b1");
    make_box(s, "b2", [-30., -30., 5.], [60., 60., 40.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["b1", "b2"]);
    bop.bfillds();
    bop.bbuild(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(33136.3));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(19),
            NbCount::Wires(13),
            NbCount::Faces(11),
            NbCount::Shells(2),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(61),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug24618_1(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=========");
    println!("OCC24618");
    println!("=========");
    println!();
    let var_c1 = cylinder(20.);
    let var_c11 = trimv(&var_c1, 0., 50.);
    mkface(s, "c1", &var_c11);
    vertex(s, "v1", [0., 20., 25.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c1", "v1"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    explode(s, "r", None);
    renamevar(s, "r_1", "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(0),
            NbCount::Shapes(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug24618_2(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=========");
    println!("OCC24618");
    println!("=========");
    println!();
    let var_c1 = cylinder(20.);
    let var_c11 = trimv(&var_c1, 0., 50.);
    mkface(s, "c1", &var_c11);
    vertex(s, "v1", [0., 20., 25.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c1", "v1"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    explode(s, "r", None);
    renamevar(s, "r_2", "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(3),
            NbCount::Edges(3),
            NbCount::Wires(1),
            NbCount::Faces(1),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(0),
            NbCount::Shapes(8),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug24618_3(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=========");
    println!("OCC24618");
    println!("=========");
    println!();
    let var_c1 = cylinder(20.);
    let var_c11 = trimv(&var_c1, 0., 50.);
    mkface(s, "c1", &var_c11);
    vertex(s, "v1", [0., 20., 25.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["v1", "c1"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    explode(s, "r", None);
    renamevar(s, "r_1", "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(1),
            NbCount::Edges(0),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(0),
            NbCount::Shapes(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug24618_4(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=========");
    println!("OCC24618");
    println!("=========");
    println!();
    let var_c1 = cylinder(20.);
    let var_c11 = trimv(&var_c1, 0., 50.);
    mkface(s, "c1", &var_c11);
    vertex(s, "v1", [0., 20., 25.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["v1", "c1"]);
    bop.bfillds();
    bop.bbuild(s, "r");
    explode(s, "r", None);
    renamevar(s, "r_2", "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(3),
            NbCount::Edges(3),
            NbCount::Wires(1),
            NbCount::Faces(1),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(0),
            NbCount::Shapes(8),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25163(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25163");
    println!("============");
    println!();
    restore(s, "bug25163_qf_25_39.brep", "b");
    explode(s, "b", Some(ShapeEnum::Face));
    copy(s, "b_1", "b1");
    copy(s, "b_2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(3417.71));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(4),
            NbCount::Wires(1),
            NbCount::Faces(1),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(11),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "b", 2);
}

pub fn bug25232_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC25232");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [4., 9., 4.], [2., 2., 2.]);
    explode(s, "b2", Some(ShapeEnum::Wire));
    let var_s = sphere([5., 5., 5.], 5.);
    mkface(s, "f", &var_s);
    mkvolume(s, "result", &["b1", "f", "b2_1"]);
    checkprops(s, "result", Prop::Area, Expected::Mass(1228.32));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(20),
            NbCount::Edges(21),
            NbCount::Wires(7),
            NbCount::Faces(7),
            NbCount::Shells(3),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(61),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
