//! The `bugs_modalg_2` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn bug20413(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC20413");
    println!("============");
    println!();
    println!("Load first shape ...");
    restore(s, "OCC20413_a1.brep", "b1");
    println!("Load second shape ...");
    restore(s, "OCC20413_a2.brep", "b2");
    println!("Prepare boolean operation ...");
    two_step.bop(s, "b1", "b2");
    println!("Start boolean operation ...");
    two_step.section(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Length, Expected::Mass(233.024));
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

pub fn bug23214(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC23214");
    println!("============");
    println!();
    restore(s, "OCC23214-face_1188.brep", "b1");
    restore(s, "OCC23214-shell_for_line_1188.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(2.94086));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug407_1(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC407 ");
    println!("(case 1)");
    println!("========================");
    println!();
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 60.);
    bfuse(s, "result", "b2", "b1");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(46.8319));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug407_2(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC407 ");
    println!("(case 2)");
    println!("========================");
    println!();
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 60.);
    bcut(s, "result", "b2", "b1");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(31.4159));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug409(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC409 ");
    println!("========================");
    println!();
    restore(s, "f2", "b1");
    checkshape_named(s, "b1");
    restore(s, "f6", "b2");
    checkshape_named(s, "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(45.5929));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug410_1(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC410 ");
    println!("(case 1)");
    println!("========================");
    println!();
    restore(s, "f7", "b1");
    checkshape_named(s, "b1");
    restore(s, "f3", "b2");
    checkshape_named(s, "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(63.4508));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug410_2(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC410 ");
    println!("(case 2)");
    println!("========================");
    println!();
    restore(s, "f7", "b1");
    checkshape_named(s, "b1");
    restore(s, "f3", "b2");
    checkshape_named(s, "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(172.169));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug410_3(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC410 ");
    println!("(case 3)");
    println!("========================");
    println!();
    restore(s, "f7", "b1");
    checkshape_named(s, "b1");
    restore(s, "f3", "b2");
    checkshape_named(s, "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(15.0891));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug411(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC411 ");
    println!("========================");
    println!();
    restore(s, "f4", "b1");
    checkshape_named(s, "b1");
    restore(s, "f5", "b2");
    checkshape_named(s, "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(314.159));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug412(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC412 ");
    println!("========================");
    println!();
    restore(s, "f2", "b1");
    checkshape_named(s, "b1");
    restore(s, "f3", "b2");
    checkshape_named(s, "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(43.6332));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug413_1(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC413 ");
    println!("(case 1)");
    println!("========================");
    println!();
    restore(s, "so1", "b1");
    checkshape_named(s, "b1");
    restore(s, "so4", "b2");
    checkshape_named(s, "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(549.779));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug413_2(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC413 ");
    println!("(case 2)");
    println!("========================");
    println!();
    restore(s, "so1", "b1");
    checkshape_named(s, "b1");
    restore(s, "so4", "b2");
    checkshape_named(s, "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(863.938));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug417(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC417 ");
    println!("CTS21128");
    println!("========================");
    println!();
    restore(s, "cts21128c.rle", "c");
    checkshape_named(s, "c");
    restore(s, "cts21128d.rle", "d");
    checkshape_named(s, "d");
    bcut(s, "result", "c", "d");
    checkprops(s, "result", Prop::Area, Expected::Mass(2814.01));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug418_1(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC418 ");
    println!("CTS20370");
    println!("(case 1)");
    println!("========================");
    println!();
    restore(s, "CTO904_cts20370-part.rle", "part");
    checkshape_named(s, "part");
    restore(s, "cts20370-tool.rle", "tool");
    checkshape_named(s, "tool");
    bsection(s, "sec", "part", "tool");
    checkshape_top(s, "sec");
    bcut(s, "result", "part", "tool");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(273365.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug418_2(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC418 ");
    println!("CTS20370");
    println!("(case 2)");
    println!("========================");
    println!();
    restore(s, "CTO904_cts20370-part.rle", "a");
    checkshape_named(s, "a");
    restore(s, "cts20370-tool.rle", "b");
    checkshape_named(s, "b");
    bsection(s, "sec", "a", "b");
    bfuse(s, "result", "a", "b");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(261265.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug419(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC419 ");
    println!("FRA50047");
    println!("========================");
    println!();
    restore(s, "CTO904_fra50047a.rle", "a");
    checkshape_named(s, "a");
    restore(s, "fra50047b.rle", "b");
    checkshape_named(s, "b");
    bsection(s, "sec", "a", "b");
    checkshape_top(s, "sec");
    bfuse(s, "result", "a", "b");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(508496.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug476_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC476");
    println!("============");
    println!();
    restore(s, "OCC485b.brep", "s1");
    restore(s, "OCC485a.brep", "s2");
    checkshape_named(s, "s1");
    checkshape_named(s, "s2");
    two_step.bop(s, "s1", "s2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(9333.96));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(26),
            NbCount::Wires(11),
            NbCount::Faces(11),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(67),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug476_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC476");
    println!("============");
    println!();
    restore(s, "OCC485b.brep", "s1");
    restore(s, "OCC485a.brep", "s2");
    checkshape_named(s, "s1");
    checkshape_named(s, "s2");
    two_step.bop(s, "s2", "s1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(9333.96));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(26),
            NbCount::Wires(11),
            NbCount::Faces(11),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(67),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug476_3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC476");
    println!("============");
    println!();
    restore(s, "OCC485b.brep", "s1");
    restore(s, "OCC485a.brep", "s2");
    checkshape_named(s, "s1");
    checkshape_named(s, "s2");
    two_step.bop(s, "s1", "s2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(99.5159));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(12),
            NbCount::Edges(19),
            NbCount::Wires(9),
            NbCount::Faces(9),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(52),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug476_4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC476");
    println!("============");
    println!();
    restore(s, "OCC485b.brep", "s1");
    restore(s, "OCC485a.brep", "s2");
    checkshape_named(s, "s1");
    checkshape_named(s, "s2");
    two_step.bop(s, "s2", "s1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(9340.03));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(19),
            NbCount::Edges(33),
            NbCount::Wires(14),
            NbCount::Faces(14),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(83),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
