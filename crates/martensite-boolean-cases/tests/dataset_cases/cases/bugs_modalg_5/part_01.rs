//! The `bugs_modalg_5` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn bug23849_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("================");
    println!("OCC23849");
    println!("================");
    println!();
    restore(s, "bug23849_segment_1.brep", "b1");
    restore(s, "bug23849_plane_1.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkshape_named(s, "result");
}

pub fn bug23849_3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("================");
    println!("OCC23849");
    println!("================");
    println!();
    restore(s, "bug23849_segment_2.brep", "b1");
    restore(s, "bug23849_plane_2.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkshape_named(s, "result");
}

pub fn bug23876(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC23876");
    println!("============");
    println!();
    vertex(s, "v1", [10., 0., 0.]);
    vertex(s, "v2", [20., 0., 0.]);
    vertex(s, "v3", [10., 0., 50.]);
    vertex(s, "v4", [20., 0., 50.]);
    edge(s, "e1", "v1", "v3");
    edge(s, "e2", "v3", "v4");
    edge(s, "e3", "v2", "v4");
    edge(s, "e4", "v2", "v1");
    wire(s, "w", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "f", "w");
    revol(s, "b1", "f", [0., 0., 0.], [0., 0., 1.], 360.);
    s.view(ViewStep::Disp(&["b1"]));
    ptorus(s, "b2", 15., 5.);
    ttranslate(s, "b2", [0., 0., 50.]);
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(2422.92));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug24003(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("================");
    println!("OCC24003");
    println!("================");
    println!();
    restore(s, "bug24003_fz33.brep", "b1");
    restore(s, "bug24003_fz47.brep", "b2");
    two_step.bop(s, "b1", "b2");
}

pub fn bug24187(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC24187");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    let geom_c1 = circle([-3., 5., 2.], 5.);
    let geom_c2 = circle([-3., 5., 8.], 5.);
    mkedge(s, "e1", &geom_c1);
    mkedge(s, "e2", &geom_c2);
    compound(s, &["e1", "e2"], "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(18.5459));
    checkshape_named(s, "result");
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(4),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(11),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug24359(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("================");
    println!("OCC24359");
    println!("================");
    println!();
    psphere(s, "s01", 2.);
    ttranslate(s, "s01", [1., 1., 1.]);
    psphere(s, "s02", 2.);
    ttranslate(s, "s02", [1., 1., 2.]);
    psphere(s, "s03", 2.);
    ttranslate(s, "s03", [1., 1., 3.]);
    psphere(s, "s04", 2.);
    ttranslate(s, "s04", [1., 2., 1.]);
    psphere(s, "s05", 2.);
    ttranslate(s, "s05", [1., 2., 2.]);
    psphere(s, "s06", 2.);
    ttranslate(s, "s06", [1., 2., 3.]);
    psphere(s, "s07", 2.);
    ttranslate(s, "s07", [1., 3., 1.]);
    psphere(s, "s08", 2.);
    ttranslate(s, "s08", [1., 3., 2.]);
    psphere(s, "s09", 2.);
    ttranslate(s, "s09", [1., 3., 3.]);
    psphere(s, "s10", 2.);
    ttranslate(s, "s10", [2., 1., 1.]);
    psphere(s, "s11", 2.);
    ttranslate(s, "s11", [2., 1., 2.]);
    psphere(s, "s12", 2.);
    ttranslate(s, "s12", [2., 1., 3.]);
    psphere(s, "s13", 2.);
    ttranslate(s, "s13", [2., 2., 1.]);
    psphere(s, "s14", 2.);
    ttranslate(s, "s14", [2., 2., 2.]);
    psphere(s, "s15", 2.);
    ttranslate(s, "s15", [2., 2., 3.]);
    psphere(s, "s16", 2.);
    ttranslate(s, "s16", [2., 3., 1.]);
    psphere(s, "s17", 2.);
    ttranslate(s, "s17", [2., 3., 2.]);
    psphere(s, "s18", 2.);
    ttranslate(s, "s18", [2., 3., 3.]);
    psphere(s, "s19", 2.);
    ttranslate(s, "s19", [3., 1., 1.]);
    psphere(s, "s20", 2.);
    ttranslate(s, "s20", [3., 1., 2.]);
    psphere(s, "s21", 2.);
    ttranslate(s, "s21", [3., 1., 3.]);
    psphere(s, "s22", 2.);
    ttranslate(s, "s22", [3., 2., 1.]);
    psphere(s, "s23", 2.);
    ttranslate(s, "s23", [3., 2., 2.]);
    psphere(s, "s24", 2.);
    ttranslate(s, "s24", [3., 2., 3.]);
    psphere(s, "s25", 2.);
    ttranslate(s, "s25", [3., 3., 1.]);
    psphere(s, "s26", 2.);
    ttranslate(s, "s26", [3., 3., 2.]);
    psphere(s, "s27", 2.);
    ttranslate(s, "s27", [3., 3., 3.]);
    two_step.bop(s, "s01", "s02");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s03");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s04");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s05");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s06");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s07");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s08");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s09");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s10");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s11");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s12");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s13");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s14");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s15");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s16");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s17");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s18");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s19");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s20");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s21");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s22");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s23");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s24");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s25");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s26");
    two_step.result(s, "s", Operation::Fuse);
    two_step.bop(s, "s", "s27");
    two_step.result(s, "result", Operation::Fuse);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(60),
            NbCount::Edges(105),
            NbCount::Wires(29),
            NbCount::Faces(29),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(226),
        ],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(150.187));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug25237(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("================");
    println!("OCC25237");
    println!("================");
    println!();
    restore(s, "bug25237_b26.brep", "e1");
    restore(s, "bug25237_b5.brep", "e2");
    two_step.bop(s, "e1", "e2");
    two_step.result(s, "result", Operation::Common);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(2),
            NbCount::Edges(1),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(4),
        ],
    );
}

pub fn bug25337_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("================");
    println!("OCC25337");
    println!("================");
    println!();
    let geom_c1 = circle([-50., 0., 0.], 10.);
    let geom_c2 = circle([-35., 0., 0.], 10.);
    mkedge(s, "e1", &geom_c1);
    mkedge(s, "e2", &geom_c2);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    two_step.bop(s, "f1", "f2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(582.987));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(6),
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(17),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25337_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("================");
    println!("OCC25337");
    println!("================");
    println!();
    let geom_c1 = circle([-50., 0., 0.], 10.);
    let geom_c2 = circle([-35., 0., 0.], 10.);
    mkedge(s, "e1", &geom_c1);
    mkedge(s, "e2", &geom_c2);
    wire(s, "w1", &["e1"]);
    wire(s, "w2", &["e2"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    two_step.bop(s, "f1", "f2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(45.3312));
    checkshape_named(s, "result");
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
            NbCount::Compounds(1),
            NbCount::Shapes(9),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25346_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("==========");
    println!("OCC25346");
    println!("==========");
    println!();
    restore(s, "CTO902_pro14235a.rle", "b1");
    restore(s, "CTO902_pro14235b.rle", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(2866.34));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_32(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b2", [-10., -10., -10.]);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
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
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug25354_33(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC25354");
    println!("============");
    println!();
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [0., 0., 0.], [10., 10., 10.]);
    ttranslate(s, "b2", [5., -10., -10.]);
    trotate(s, "b2", [5., 0., 0.], [0., 0., 1.], -45.);
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
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
            NbCount::Compounds(1),
            NbCount::Shapes(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
