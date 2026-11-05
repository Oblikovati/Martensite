//! The `bugs_modalg_2` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn bug476_5(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut21);
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

pub fn bug476_6(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut21);
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

pub fn bug476_7(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(105.587));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(15),
            NbCount::Edges(26),
            NbCount::Wires(12),
            NbCount::Faces(12),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(68),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug476_8(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(105.587));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(15),
            NbCount::Edges(26),
            NbCount::Wires(12),
            NbCount::Faces(12),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(68),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug4993_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC4993");
    println!("============");
    println!();
    restore(s, "OCC4993-s1.brep", "s1");
    restore(s, "OCC4993-s2.brep", "s2");
    checkshape_named(s, "s1");
    checkshape_named(s, "s2");
    two_step.bop(s, "s1", "s2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(4854.51));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(40),
            NbCount::Edges(64),
            NbCount::Wires(24),
            NbCount::Faces(24),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(155),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug4993_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC4993");
    println!("============");
    println!();
    restore(s, "OCC4993-s1.brep", "s1");
    restore(s, "OCC4993-s2.brep", "s2");
    checkshape_named(s, "s1");
    checkshape_named(s, "s2");
    two_step.bop(s, "s2", "s1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(4854.51));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(40),
            NbCount::Edges(64),
            NbCount::Wires(24),
            NbCount::Faces(24),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(155),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
