//! The `bugs_modalg_1` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn bug13116_3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13116");
    println!("============");
    println!();
    restore(s, "OCC13116_sh1.brep", "b1");
    restore(s, "OCC13116_sh2.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(438194.));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(21),
            NbCount::Edges(34),
            NbCount::Wires(15),
            NbCount::Faces(15),
            NbCount::Shells(2),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(89),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13116_4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13116");
    println!("============");
    println!();
    restore(s, "OCC13116_sh1.brep", "b1");
    restore(s, "OCC13116_sh2.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(3769.89));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(3),
            NbCount::Edges(8),
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(20),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13186_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC13186");
    println!("============");
    println!();
    restore(s, "OCC13186_cutcyl11.brep", "b1");
    restore(s, "OCC13186_sphere.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(42.5841));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(19),
            NbCount::Edges(25),
            NbCount::Wires(12),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(66),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13186_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC13186");
    println!("============");
    println!();
    restore(s, "OCC13186_cutcyl11.brep", "b1");
    restore(s, "OCC13186_sphere.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(85.6816));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(21),
            NbCount::Edges(28),
            NbCount::Wires(13),
            NbCount::Faces(8),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(73),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13186_3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC13186");
    println!("============");
    println!();
    restore(s, "OCC13186_cutcyl11.brep", "b1");
    restore(s, "OCC13186_sphere.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(14.9648));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(11),
            NbCount::Wires(7),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(34),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13186_4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC13186");
    println!("============");
    println!();
    restore(s, "OCC13186_cutcyl11.brep", "b1");
    restore(s, "OCC13186_sphere.brep", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(58.0624));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(10),
            NbCount::Edges(14),
            NbCount::Wires(8),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(41),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug13538(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC13538");
    println!("============");
    println!();
    make_box(s, "a", [0., 0., 0.], [100., 100., 100.]);
    make_box(s, "b", [0., 0., 0.], [100., 100., 100.]);
    ttranslate(s, "b", [0., 0., 100.]);
    explode(s, "a", Some(ShapeEnum::Shell));
    explode(s, "b", Some(ShapeEnum::Shell));
    two_step.bop(s, "a_1", "b_1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(10000.));
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
            NbCount::Shells(1),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(12),
        ],
    );
}

pub fn bug14536(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC14536");
    println!("============");
    println!();
    restore(s, "OCC14536-sh1.brep", "s1");
    restore(s, "OCC14536-sh2.brep", "s2");
    two_step.bop(s, "s1", "s2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(330318.));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(7),
            NbCount::Edges(13),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(35),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug14777(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC14777");
    println!("============");
    println!();
    restore(s, "OCC14777-s1.brep", "s1");
    restore(s, "OCC14777-s2.brep", "s2");
    two_step.bop(s, "s1", "s2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(10653.3));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(9),
            NbCount::Wires(5),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(28),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug15850(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC15850");
    println!("============");
    println!();
    restore(s, "OCC15850_bb.brep", "bb");
    restore(s, "OCC15850_bc.brep", "bc");
    two_step.bop(s, "bb", "bc");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(732.453));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(12),
            NbCount::Edges(17),
            NbCount::Wires(9),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(48),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug15850_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC15850");
    println!("============");
    println!();
    restore(s, "OCC15850_bb.brep", "bb");
    restore(s, "OCC15850_bc.brep", "bc");
    two_step.bop(s, "bb", "bc");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(301.996));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(8),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(2),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(31),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug15850_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC15850");
    println!("============");
    println!();
    restore(s, "OCC15850_bb.brep", "bb");
    restore(s, "OCC15850_bc.brep", "bc");
    two_step.bop(s, "bb", "bc");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(245.55));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(5),
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(18),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug15850_3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of ");
    println!("============");
    println!("OCC15850");
    println!("============");
    println!();
    restore(s, "OCC15850_bb.brep", "bb");
    restore(s, "OCC15850_bc.brep", "bc");
    two_step.bop(s, "bb", "bc");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(788.9));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(14),
            NbCount::Edges(20),
            NbCount::Wires(12),
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
