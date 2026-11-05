//! The `bugs_modalg_1` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn bug102_2(s: &mut dyn Session) {
    println!("================");
    println!("BUC60803");
    println!("OCC102");
    println!("================");
    println!();
    restore(s, "buc60803a.brep", "sh1");
    checkshape_named(s, "sh1");
    restore(s, "buc60803b.brep", "sh2");
    checkshape_named(s, "sh2");
    bfuse(s, "result", "sh2", "sh1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.86064e+07));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug10605_1(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Common);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(233.798));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(10),
            NbCount::Edges(15),
            NbCount::Wires(7),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(42),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10605_2(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Fuse);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(838.752));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(15),
            NbCount::Edges(22),
            NbCount::Wires(10),
            NbCount::Faces(9),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(59),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10605_3(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(779.887));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(13),
            NbCount::Edges(19),
            NbCount::Wires(8),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(50),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10605_4(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut21);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(292.663));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(12),
            NbCount::Edges(18),
            NbCount::Wires(9),
            NbCount::Faces(9),
            NbCount::Shells(2),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(53),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10606_1(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Common);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(66.672));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(5),
            NbCount::Edges(7),
            NbCount::Wires(2),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(19),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10606_2(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Fuse);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(1450.22));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(5),
            NbCount::Edges(7),
            NbCount::Wires(4),
            NbCount::Faces(4),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(23),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10606_3(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(761.355));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(5),
            NbCount::Edges(7),
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(21),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10606_4(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut21);
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(755.54));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(5),
            NbCount::Edges(7),
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(21),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug12507(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC12507");
    println!("============");
    println!();
    println!("First shape ...");
    pcylinder(s, "c", 3., 3.);
    explode(s, "c", Some(ShapeEnum::Edge));
    println!("Second shape ...");
    tcopy(s, "c_3", "e3");
    ttranslate(s, "e3", [0., 6., 0.]);
    two_step.bop(s, "c_3", "e3");
    two_step.result(s, "result", Operation::Fuse);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(3),
            NbCount::Edges(4),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(8),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug12918(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC12918");
    println!("============");
    println!();
    restore(s, "OCC12918_1.brep", "b1");
    restore(s, "OCC12918_2.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(0.00240038));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(24),
            NbCount::Edges(36),
            NbCount::Wires(14),
            NbCount::Faces(14),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(91),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13116_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13116");
    println!("============");
    println!();
    restore(s, "OCC13116_sh1.brep", "b1");
    restore(s, "OCC13116_sh2.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(438194.));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(21),
            NbCount::Edges(37),
            NbCount::Wires(16),
            NbCount::Faces(16),
            NbCount::Shells(2),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(94),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13116_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13116");
    println!("============");
    println!();
    restore(s, "OCC13116_sh1.brep", "b1");
    restore(s, "OCC13116_sh2.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(3769.91));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(3),
            NbCount::Edges(5),
            NbCount::Wires(2),
            NbCount::Faces(2),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(15),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
