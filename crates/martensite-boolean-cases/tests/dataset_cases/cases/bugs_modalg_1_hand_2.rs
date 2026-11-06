//! `bugs_modalg_1`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::inspect::checksection_count,
    bugs_commands::repair::{build3d, halfspace_of, revol_copied},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::make_box,
    construction::{edge, mkplane, vertex, wire},
    named_shapes::renamevar,
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcut, bfuse, bsection},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub fn buc60462_1(s: &mut dyn Session) {
    println!("=============");
    println!("BUC60462");
    println!("=============");
    println!("Section");
    println!();
    restore(s, "buc60462a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60462b.brep", "b");
    checkshape_named(s, "b");
    bsection(s, "result", "a", "b");
    if checksection_count(s, "result") != 0 {
        println!("Faulty : Section is incorrect");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(282.421));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60463(s: &mut dyn Session) {
    println!("=============");
    println!("BUC60463");
    println!("=============");
    println!("Section");
    println!();
    println!("CPU and elapsed time limit set to 1000 seconds");
    restore(s, "buc60463a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60463b.brep", "b");
    checkshape_named(s, "b");
    bsection(s, "result", "a", "b");
    if checksection_count(s, "result") != 0 {
        println!("Faulty : Section is incorrect");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(520.961));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60532(s: &mut dyn Session) {
    println!("===========");
    println!("BUC60532");
    println!("===========");
    restore(s, "buc60532a.brep", "p");
    checkshape_named(s, "p");
    restore(s, "buc60532b.brep", "t1");
    checkshape_named(s, "t1");
    restore(s, "buc60532c.brep", "t2");
    checkshape_named(s, "t2");
    halfspace_of(s, "h1", "t1", [20., 20., 0.]);
    halfspace_of(s, "h2", "t2", [20., 20., 0.]);
    bcut(s, "result", "p", "h1");
    checkprops(s, "result", Prop::Area, Expected::Mass(81.8172));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60532_1(s: &mut dyn Session) {
    println!("===========");
    println!("BUC60532");
    println!("===========");
    restore(s, "buc60532a.brep", "p");
    checkshape_named(s, "p");
    restore(s, "buc60532b.brep", "t1");
    checkshape_named(s, "t1");
    restore(s, "buc60532c.brep", "t2");
    checkshape_named(s, "t2");
    halfspace_of(s, "h1", "t1", [20., 20., 0.]);
    halfspace_of(s, "h2", "t2", [20., 20., 0.]);
    bcut(s, "result", "p", "h1");
    checkprops(s, "result", Prop::Area, Expected::Mass(81.8172));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60663_1(s: &mut dyn Session) {
    println!("=====================================");
    println!("BUC60663");
    println!("=====================================");
    println!();
    println!("It takes visual check for this BUG. See V2 point");
    println!("=====================================");
    println!();
    restore(s, "buc60663a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60663b.brep", "b");
    checkshape_named(s, "b");
    bsection(s, "result", "a", "b");
    if checksection_count(s, "result") != 2 {
        println!(" Faulty : Result shape is INcorrect !!! ");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(26.2001));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60703_1(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60703");
    println!("=================================");
    println!("Action with models with VERY BIG tolerance");
    println!("=================================");
    println!("CPU and elapsed time limit set to 1000 seconds");
    restore(s, "buc60703a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60703b.brep", "b");
    checkshape_named(s, "b");
    bfuse(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(173.109));
    checkprops(s, "result", Prop::Volume, Expected::Mass(129.814));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(10),
            NbCount::Faces(10),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn buc60728(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60728");
    println!("========================");
    println!();
    restore(s, "buc60728a.brep", "s1");
    restore(s, "buc60728b.brep", "s2");
    build3d(s, "s1");
    checkshape_named(s, "s1");
    checkshape_named(s, "s2");
    bsection(s, "result", "s1", "s2");
    checkprops(s, "result", Prop::Length, Expected::Mass(238.032));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn buc60801(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60801");
    println!("========================");
    make_box(s, "b", [-10., -10., -10.], [30., 40., 40.]);
    make_box(s, "a1", [-20., 0., -20.], [20., 20., 100.]);
    make_box(s, "a2", [0., 20., -20.], [50., 20., 100.]);
    make_box(s, "c", [-20., -20., 20.], [100., 100., 100.]);
    bcut(s, "r1", "b", "a1");
    bcut(s, "r2", "r1", "a2");
    bcut(s, "r3", "r2", "c");
    checkshape_named(s, "r3");
    renamevar(s, "r3", "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(6400.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug10605_5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC10605");
    println!("============");
    println!();
    restore(s, "OCC10605-1.brep", "b1");
    restore(s, "OCC10605-3.brep", "b2");
    println!("Prepare boolean operation ...");
    two_step.bop(s, "b1", "b2");
    println!("Start boolean operation ...");
    two_step.section(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Length, Expected::Mass(59.8334));
    checkshape_named(s, "result");
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(9),
            NbCount::Edges(9),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(19),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10606_5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC10606");
    println!("============");
    println!();
    restore(s, "OCC10604-1.brep", "b1");
    restore(s, "OCC10606-2.brep", "b2");
    println!("Prepare boolean operation ...");
    two_step.bop(s, "b1", "b2");
    println!("Start boolean operation ...");
    two_step.section(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Length, Expected::Mass(18.3861));
    checkshape_named(s, "result");
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(3),
            NbCount::Edges(3),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(7),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13211_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13211");
    println!("============");
    println!();
    vertex(s, "v1", [136.06, 187.51, 0.0]);
    vertex(s, "v2", [136.06, 143.01, 0.0]);
    edge(s, "e1", "v1", "v2");
    vertex(s, "v3", [136.06, 143.01, 0.0]);
    vertex(s, "v4", [128.08, 143.01, 0.0]);
    edge(s, "e2", "v3", "v4");
    vertex(s, "v5", [128.08, 143.01, 0.0]);
    vertex(s, "v6", [127.88, 164.75, 0.0]);
    edge(s, "e3", "v5", "v6");
    vertex(s, "v7", [127.88, 164.75, 0.0]);
    vertex(s, "v8", [116.11, 176.00, 0.0]);
    edge(s, "e4", "v7", "v8");
    vertex(s, "v9", [116.11, 176.00, 0.0]);
    vertex(s, "v10", [136.06, 187.51, 0.0]);
    edge(s, "e5", "v9", "v10");
    wire(s, "twire", &["e1", "e2", "e3", "e4", "e5"]);
    mkplane(s, "tface", "twire");
    revol_copied(s, "r1", "tface", [153.08, 143.01, 0.0], [1., 0., 0.], 360.);
    revol_copied(s, "r2", "tface", [136.06, 68.01, 0.0], [0., 1., 0.], 360.);
    two_step.bop(s, "r1", "r2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(17308.3));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(25),
            NbCount::Wires(13),
            NbCount::Faces(12),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(66),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13211_3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13211");
    println!("============");
    println!();
    vertex(s, "v1", [136.06, 187.51, 0.0]);
    vertex(s, "v2", [136.06, 143.01, 0.0]);
    edge(s, "e1", "v1", "v2");
    vertex(s, "v3", [136.06, 143.01, 0.0]);
    vertex(s, "v4", [128.08, 143.01, 0.0]);
    edge(s, "e2", "v3", "v4");
    vertex(s, "v5", [128.08, 143.01, 0.0]);
    vertex(s, "v6", [127.88, 164.75, 0.0]);
    edge(s, "e3", "v5", "v6");
    vertex(s, "v7", [127.88, 164.75, 0.0]);
    vertex(s, "v8", [116.11, 176.00, 0.0]);
    edge(s, "e4", "v7", "v8");
    vertex(s, "v9", [116.11, 176.00, 0.0]);
    vertex(s, "v10", [136.06, 187.51, 0.0]);
    edge(s, "e5", "v9", "v10");
    wire(s, "twire", &["e1", "e2", "e3", "e4", "e5"]);
    mkplane(s, "tface", "twire");
    revol_copied(s, "r1", "tface", [153.08, 143.01, 0.0], [1., 0., 0.], 360.);
    revol_copied(s, "r2", "tface", [136.06, 68.01, 0.0], [0., 1., 0.], 360.);
    two_step.bop(s, "r1", "r2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(2990.42));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(23),
            NbCount::Wires(10),
            NbCount::Faces(10),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(59),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13211_4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13211");
    println!("============");
    println!();
    vertex(s, "v1", [136.06, 187.51, 0.0]);
    vertex(s, "v2", [136.06, 143.01, 0.0]);
    edge(s, "e1", "v1", "v2");
    vertex(s, "v3", [136.06, 143.01, 0.0]);
    vertex(s, "v4", [128.08, 143.01, 0.0]);
    edge(s, "e2", "v3", "v4");
    vertex(s, "v5", [128.08, 143.01, 0.0]);
    vertex(s, "v6", [127.88, 164.75, 0.0]);
    edge(s, "e3", "v5", "v6");
    vertex(s, "v7", [127.88, 164.75, 0.0]);
    vertex(s, "v8", [116.11, 176.00, 0.0]);
    edge(s, "e4", "v7", "v8");
    vertex(s, "v9", [116.11, 176.00, 0.0]);
    vertex(s, "v10", [136.06, 187.51, 0.0]);
    edge(s, "e5", "v9", "v10");
    wire(s, "twire", &["e1", "e2", "e3", "e4", "e5"]);
    mkplane(s, "tface", "twire");
    revol_copied(s, "r1", "tface", [153.08, 143.01, 0.0], [1., 0., 0.], 360.);
    revol_copied(s, "r2", "tface", [136.06, 68.01, 0.0], [0., 1., 0.], 360.);
    two_step.bop(s, "r1", "r2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(17292.5));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(26),
            NbCount::Wires(12),
            NbCount::Faces(11),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(65),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13211_5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13211");
    println!("============");
    println!();
    vertex(s, "v1", [136.06, 187.51, 0.0]);
    vertex(s, "v2", [136.06, 143.01, 0.0]);
    edge(s, "e1", "v1", "v2");
    vertex(s, "v3", [136.06, 143.01, 0.0]);
    vertex(s, "v4", [128.08, 143.01, 0.0]);
    edge(s, "e2", "v3", "v4");
    vertex(s, "v5", [128.08, 143.01, 0.0]);
    vertex(s, "v6", [127.88, 164.75, 0.0]);
    edge(s, "e3", "v5", "v6");
    vertex(s, "v7", [127.88, 164.75, 0.0]);
    vertex(s, "v8", [116.11, 176.00, 0.0]);
    edge(s, "e4", "v7", "v8");
    vertex(s, "v9", [116.11, 176.00, 0.0]);
    vertex(s, "v10", [136.06, 187.51, 0.0]);
    edge(s, "e5", "v9", "v10");
    wire(s, "twire", &["e1", "e2", "e3", "e4", "e5"]);
    mkplane(s, "tface", "twire");
    revol_copied(s, "r1", "tface", [153.08, 143.01, 0.0], [1., 0., 0.], 360.);
    revol_copied(s, "r2", "tface", [136.06, 68.01, 0.0], [0., 1., 0.], 360.);
    two_step.bop(s, "r1", "r2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(3006.19));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(11),
            NbCount::Edges(18),
            NbCount::Wires(11),
            NbCount::Faces(11),
            NbCount::Shells(3),
            NbCount::Solids(3),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(58),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
