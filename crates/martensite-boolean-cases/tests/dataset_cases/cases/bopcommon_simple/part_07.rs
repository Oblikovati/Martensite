//! The `bopcommon_simple` case bodies, part 7; each statement is one line of its script.
use super::*;

pub fn ZP3(s: &mut dyn Session) {
    println!("=================================");
    println!("COMMON of box and halfspace");
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
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(81000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP4(s: &mut dyn Session) {
    println!("=================================");
    println!("COMMON of box and halfspace");
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
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(179000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP5(s: &mut dyn Session) {
    println!("=================================");
    println!("COMMON of box and halfspace");
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
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(116000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP6(s: &mut dyn Session) {
    println!("=================================");
    println!("COMMON of box and halfspace");
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
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(144000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "hs"], Prop::Area),
    );
}

pub fn ZP7(s: &mut dyn Session) {
    pcone(s, "pc", 10., 0., 20.);
    explode(s, "pc", Some(ShapeEnum::Face));
    prism(s, "pcy", "pc_2", [0., 0., 10.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "pc", "pcy");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(919.56));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["pc", "pcy"], Prop::Area),
    );
}

pub fn ZP8(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r_1", "b1"], Prop::Area),
    );
}

pub fn ZP9(s: &mut dyn Session) {
    make_box(s, "bb", [0., 0., 0.], [100., 100., 100.]);
    let pl1 = plane([98.946735014, 46.491265177, 17.092869659]);
    let pl2 = plane([-1.053264986, 46.491265177, 17.092869659]);
    psphere_on(s, "s1", &pl1, 7.5);
    psphere_on(s, "s2", &pl2, 7.5);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "bb", "s1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(576.293));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["bb", "s1"], Prop::Area),
    );
}
