//! `bugs_modalg_7`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::geometry::{line, mkedge_range},
    bugs_commands::inspect::bopcheck_ok,
    bugs_commands::options_bop::ScriptBop,
    bugs_commands::repair::settolerance,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, make_box, pcone, ptorus, ttranslate},
    construction::vertex,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    section_check::checksection_expecting,
    session::Session,
    surface_commands::{mkface_bounded, plane},
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug29843_2(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("0029843: Modeling Algorithms - Boolean FUSE produces incorrect result");
    println!("========");
    println!();
    println!("Boolean operation fails on the objects located far from origin (1.e+8)");
    restore(s, "bug29843_loc.brep", "s");
    explode(s, "s", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s_1"]);
    bop.add_tools(s, &["s_2"]);
    bop.bfillds();
    bop.bbop(s, "r_0", Operation::Common);
    bop.bbop(s, "r_1", Operation::Fuse);
    bop.bbop(s, "r_2", Operation::Cut);
    bop.bbop(s, "r_3", Operation::Cut21);
    bop.bbop(s, "r_4", Operation::Section);
    bop.bbuild(s, "r_5");
    checkshape_named(s, "r_0");
    bopcheck_ok(
        s,
        "r_0",
        &bop.options,
        "Error: r_0 is self-intersecting shape",
    );
    checkshape_named(s, "r_1");
    bopcheck_ok(
        s,
        "r_1",
        &bop.options,
        "Error: r_1 is self-intersecting shape",
    );
    checkshape_named(s, "r_2");
    bopcheck_ok(
        s,
        "r_2",
        &bop.options,
        "Error: r_2 is self-intersecting shape",
    );
    checkshape_named(s, "r_3");
    bopcheck_ok(
        s,
        "r_3",
        &bop.options,
        "Error: r_3 is self-intersecting shape",
    );
    checkshape_named(s, "r_4");
    bopcheck_ok(
        s,
        "r_4",
        &bop.options,
        "Error: r_4 is self-intersecting shape",
    );
    checkshape_named(s, "r_5");
    bopcheck_ok(
        s,
        "r_5",
        &bop.options,
        "Error: r_5 is self-intersecting shape",
    );
    checkprops(s, "r_0", Prop::Area, Expected::Mass(62185.2));
    checkprops(s, "r_0", Prop::Volume, Expected::Mass(1.1761e+06));
    checknbshapes(
        s,
        "r_0",
        Counting::Typed,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(9),
            NbCount::Wires(5),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_1", Prop::Area, Expected::Mass(1.85327e+06));
    checkprops(s, "r_1", Prop::Volume, Expected::Mass(5.92874e+07));
    checknbshapes(
        s,
        "r_1",
        Counting::Typed,
        &[
            NbCount::Vertices(15),
            NbCount::Edges(24),
            NbCount::Wires(11),
            NbCount::Faces(11),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_2", Prop::Area, Expected::Mass(1.85376e+06));
    checkprops(s, "r_2", Prop::Volume, Expected::Mass(5.78639e+07));
    checknbshapes(
        s,
        "r_2",
        Counting::Typed,
        &[
            NbCount::Vertices(14),
            NbCount::Edges(21),
            NbCount::Wires(9),
            NbCount::Faces(9),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_3", Prop::Area, Expected::Mass(37189.7));
    checkprops(s, "r_3", Prop::Volume, Expected::Mass(247431.));
    checknbshapes(
        s,
        "r_3",
        Counting::Typed,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(9),
            NbCount::Wires(5),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_4", Prop::Length, Expected::Mass(952.189));
    checksection_expecting(s, "r_4", 2);
    checknbshapes(
        s,
        "r_4",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(7)],
    );
    checkprops(s, "r_5", Prop::Area, Expected::Mass(2.01533e+06));
    checkprops(s, "r_5", Prop::Volume, Expected::Mass(6.04635e+07));
    checknbshapes(
        s,
        "r_5",
        Counting::Typed,
        &[
            NbCount::Vertices(15),
            NbCount::Edges(26),
            NbCount::Wires(15),
            NbCount::Faces(15),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    checkview(s, &CheckView::of("r_2"));
}

pub fn bug30092(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("========");
    println!("OCC30092: Modeling Algorithms - Invalid result of Section operation");
    println!("========");
    println!();
    restore(s, "bug26795_c.brep", "c");
    explode(s, "c", Some(ShapeEnum::Face));
    let var_l = line(
        [20004.7983876926, 2164.22470816234, 210.858020395251],
        [-0.716336050762455, 0.65536108806844, 0.239508886314894],
    );
    mkedge_range(s, "e", &var_l, -0.2, 0.2);
    settolerance(s, "e", 0.04);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c_2"]);
    bop.add_tools(s, &["e"]);
    bop.bfillds();
    bop.bsplit(s, "c_2");
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["e", "c_1"]);
    bop.add_tools(s, &["c_2"]);
    bop.bfillds();
    bop.bbop(s, "result", Operation::Section);
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(1250.44));
}

pub fn bug30386_4(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!("=======================================================");
    println!("0030386: Modeling Algorithms - Unable to perform Cut operation");
    println!("=======================================================");
    println!();
    vertex(s, "v", [0., 0., 2.]);
    let var_l = line([5., 5., 0.], [0., 0., 1.]);
    mkedge_range(s, "e", &var_l, -10., 10.);
    let var_p = plane([0., 0., 1.], [0., 0., 1.]);
    mkface_bounded(s, "f", &var_p, (-15., 15., -15., 15.));
    make_box(s, "b", [8., 8., 8.], [5., 5., 5.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["v", "e", "f", "b"]);
    bop.bfillds();
    bop.bbuild(s, "s1");
    make_box(s, "s2", [0., 0., 0.], [10., 10., 10.]);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["s1"]);
    bop.add_tools(s, &["s2"]);
    bop.bfillds();
    bop.bbop(s, "r0", Operation::Common);
    bop.bbop(s, "r2", Operation::Cut);
    checkshape_named(s, "r0");
    bopcheck_ok(
        s,
        "r0",
        &bop.options,
        "Error: the r0 is a self-interfering shape",
    );
    checkshape_named(s, "r2");
    bopcheck_ok(
        s,
        "r2",
        &bop.options,
        "Error: the r2 is a self-interfering shape",
    );
    checknbshapes(
        s,
        "r0",
        Counting::Typed,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(18),
            NbCount::Wires(7),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r0", Prop::Area, Expected::Mass(124.));
    explode(s, "r0", Some(ShapeEnum::Solid));
    checkprops(s, "r0_1", Prop::Volume, Expected::Mass(8.));
    checknbshapes(
        s,
        "r2",
        Counting::Typed,
        &[
            NbCount::Vertices(24),
            NbCount::Edges(30),
            NbCount::Wires(11),
            NbCount::Faces(10),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r2", Prop::Area, Expected::Mass(950.));
    explode(s, "r2", Some(ShapeEnum::Solid));
    checkprops(s, "r2_1", Prop::Volume, Expected::Mass(117.));
    checkview(s, &CheckView::of("r2"));
}

pub fn bug31836(s: &mut dyn Session) {
    let mut bop = ScriptBop::default();
    println!(
        "============================================================================================="
    );
    println!("0031836: regression issue - boolean cut between a cone and a Torus is not done");
    println!(
        "============================================================================================="
    );
    println!();
    pcone(s, "c", 282.842712474619, 100., 300.);
    ttranslate(s, "c", [100., 0., 0.]);
    ptorus(s, "t", 80.2535587508467, 19.7464412491533);
    ttranslate(s, "t", [100., 0., 300.]);
    bop.options.fuzzy = 1.0e-6;
    bop.options.run_parallel = true;
    bop.options.non_destructive = true;
    bop.options.fill_history = false;
    explode(s, "s", None);
    bop.clear_objects();
    bop.clear_tools();
    bop.add_objects(s, &["c"]);
    bop.add_tools(s, &["t"]);
    bop.bfillds();
    bop.bbop(s, "r_0", Operation::Common);
    bop.bbop(s, "r_1", Operation::Fuse);
    bop.bbop(s, "r_2", Operation::Cut);
    bop.bbop(s, "r_3", Operation::Cut21);
    bop.bbop(s, "r_4", Operation::Section);
    bop.bbuild(s, "r_5");
    bop.reset_options();
    checkshape_named(s, "r_0");
    bopcheck_ok(
        s,
        "r_0",
        &bop.options,
        "Error: r_0 is self-intersecting shape",
    );
    checkshape_named(s, "r_1");
    bopcheck_ok(
        s,
        "r_1",
        &bop.options,
        "Error: r_1 is self-intersecting shape",
    );
    checkshape_named(s, "r_2");
    bopcheck_ok(
        s,
        "r_2",
        &bop.options,
        "Error: r_2 is self-intersecting shape",
    );
    checkshape_named(s, "r_3");
    bopcheck_ok(
        s,
        "r_3",
        &bop.options,
        "Error: r_3 is self-intersecting shape",
    );
    checkshape_named(s, "r_4");
    bopcheck_ok(
        s,
        "r_4",
        &bop.options,
        "Error: r_4 is self-intersecting shape",
    );
    checkshape_named(s, "r_5");
    bopcheck_ok(
        s,
        "r_5",
        &bop.options,
        "Error: r_5 is self-intersecting shape",
    );
    checkprops(s, "r_0", Prop::Area, Expected::Mass(51195.4));
    checkprops(s, "r_0", Prop::Volume, Expected::Mass(308846.));
    checknbshapes(
        s,
        "r_0",
        Counting::Typed,
        &[
            NbCount::Wires(3),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_1", Prop::Area, Expected::Mass(716665.));
    checkprops(s, "r_1", Prop::Volume, Expected::Mass(3.74689e+07));
    checknbshapes(
        s,
        "r_1",
        Counting::Typed,
        &[
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_2", Prop::Area, Expected::Mass(716665.));
    checkprops(s, "r_2", Prop::Volume, Expected::Mass(3.68513e+07));
    checknbshapes(
        s,
        "r_2",
        Counting::Typed,
        &[
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_3", Prop::Area, Expected::Mass(51195.4));
    checkprops(s, "r_3", Prop::Volume, Expected::Mass(308846.));
    checknbshapes(
        s,
        "r_3",
        Counting::Typed,
        &[
            NbCount::Wires(3),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "r_4", Prop::Length, Expected::Mass(1008.5));
    checksection_expecting(s, "r_4", 0);
    checknbshapes(s, "r_4", Counting::Located, &[NbCount::Edges(2)]);
    checkprops(s, "r_5", Prop::Area, Expected::Mass(819056.));
    checkprops(s, "r_5", Prop::Volume, Expected::Mass(3.74689e+07));
    checknbshapes(
        s,
        "r_5",
        Counting::Typed,
        &[
            NbCount::Wires(7),
            NbCount::Faces(6),
            NbCount::Shells(3),
            NbCount::Solids(3),
        ],
    );
    checkview(s, &CheckView::of("r_2"));
}
