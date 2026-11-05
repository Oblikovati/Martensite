//! The `bopcut_simple` case bodies, part 7; each statement is one line of its script.
use super::*;

pub fn ZP3(s: &mut dyn Session) {
    println!("=================================");
    println!("CUT of box and halfspace");
    println!("Case is duplicate of chl 918 B4, bug OCC297 ");
    println!("=================================");
    vertex(s, "v1", [250., 250., 0.]);
    vertex(s, "v2", [-250., 250., 0.]);
    vertex(s, "v3", [-250., -250., 0.]);
    vertex(s, "v4", [250., -250., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "f", "w1", false);
    halfspace(s, "hs", "f", [0., 0., -100.]);
    make_box(s, "b", [0., 0., -30.], [150., 200., 200.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "b", "hs");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(179000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP4(s: &mut dyn Session) {
    println!("=================================");
    println!("CUT of box and halfspace");
    println!("Case is duplicate of chl 918 CIN001_b5.rle, bug OCC297 ");
    println!("=================================");
    vertex(s, "v1", [250., 250., 0.]);
    vertex(s, "v2", [-250., 250., 0.]);
    vertex(s, "v3", [-250., -250., 0.]);
    vertex(s, "v4", [250., -250., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "f", "w1", false);
    halfspace(s, "hs", "f", [0., 0., 100.]);
    make_box(s, "b", [0., 0., -30.], [150., 200., 200.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "b", "hs");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(81000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP5(s: &mut dyn Session) {
    println!("=================================");
    println!("CUT of box and halfspace");
    println!("Case is duplicate of chl 918 B6, bug OCC297 ");
    println!("=================================");
    vertex(s, "v1", [250., 250., 0.]);
    vertex(s, "v2", [-250., 250., 0.]);
    vertex(s, "v3", [-250., -250., 0.]);
    vertex(s, "v4", [250., -250., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "f", "w1", false);
    halfspace(s, "hs", "f", [0., 0., -100.]);
    make_box(s, "b", [0., 0., -80.], [150., 200., 200.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "b", "hs");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(144000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP6(s: &mut dyn Session) {
    println!("=================================");
    println!("CUT of box and halfspace");
    println!("Case is duplicate of chl 918 B7, bug OCC297 ");
    println!("=================================");
    vertex(s, "v1", [250., 250., 0.]);
    vertex(s, "v2", [-250., 250., 0.]);
    vertex(s, "v3", [-250., -250., 0.]);
    vertex(s, "v4", [250., -250., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "f", "w1", false);
    halfspace(s, "hs", "f", [0., 0., 100.]);
    make_box(s, "b", [0., 0., -80.], [150., 200., 200.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "b", "hs");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(116000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP7(s: &mut dyn Session) {
    vertex(s, "v1", [-1., -1., 0.]);
    vertex(s, "v2", [2., -1., 0.]);
    edge(s, "ea", "v1", "v2");
    prism(s, "fa", "ea", [0., 3.5, 0.]);
    prism(s, "ba", "fa", [0., 0., 2.]);
    vertex(s, "v3", [0., 0., 0.]);
    vertex(s, "v4", [1., 0., 0.]);
    vertex(s, "v5", [1., 0., 2.]);
    vertex(s, "v6", [0., 0., 2.]);
    edge(s, "eb", "v3", "v4");
    edge(s, "ec", "v4", "v5");
    edge(s, "ed", "v5", "v6");
    wire(s, "wb", &["eb", "ec", "ed"]);
    revol(s, "bb", "wb", [0., 0., 0.], [0., 0., 1.], 360.);
    emptycopy(s, "s", "ba");
    add(s, "bb", "s");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "ba", "s");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(53.2832));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "s"], Prop::Area),
    );
}

pub fn ZP8(s: &mut dyn Session) {
    pcone(s, "pc", 10., 0., 20.);
    explode(s, "pc", Some(ShapeEnum::Face));
    prism(s, "pcy", "pc_2", [0., 0., 10.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "pc", "pcy");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(254.16));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["pc", "pcy"], Prop::Area),
    );
}

pub fn ZP9(s: &mut dyn Session) {
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [2., 2., 2.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "b", "c");
    two_step.result(s, "r", Operation::Cut);
    explode(s, "r", Some(ShapeEnum::Shell));
    orientation(s, "r_2", Orientation::Reversed);
    emptycopy(s, "b1", "b");
    add(s, "r_2", "b1");
    explode(s, "r", Some(ShapeEnum::Solid));
    two_step.bop(s, "r_1", "b1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(696.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r_1", "b1"], Prop::Area),
    );
}

pub fn ZQ1(s: &mut dyn Session) {
    println!("============");
    println!("CR23431");
    println!("============");
    println!();
    let pl1 = plane([0., 0., 0.]);
    mkface_bounded(s, "face_1", &pl1, (-100., 100., -100., 100.));
    let c1 = circle([0., 0., 0.], 20.);
    let c2 = circle([0., 0., 0.], 30.);
    mkedge(s, "e1", &c1);
    wire(s, "w1", &["e1"]);
    mkface_of_wire(s, "d1", &pl1, "w1");
    mkedge(s, "e2", &c2);
    wire(s, "w2", &["e2"]);
    mkface_of_wire(s, "d2", &pl1, "w2");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "d2", "d1");
    two_step.result(s, "D_coupe_1", Operation::Cut);
    two_step.bop(s, "D_coupe_1", "d1");
    two_step.result(s, "Union_1", Operation::Fuse);
    two_step.bop(s, "face_1", "Union_1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(37172.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["face_1", "Union_1"], Prop::Area),
    );
}
