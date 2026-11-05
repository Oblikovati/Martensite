//! The `bopfuse_simple` case bodies, part 7; each statement is one line of its script.
use super::*;

pub fn ZP4(s: &mut dyn Session) {
    pcone(s, "pc", 10., 0., 20.);
    explode(s, "pc", Some(ShapeEnum::Face));
    prism(s, "pcy", "pc_2", [0., 0., 10.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "pc", "pcy");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(1353.72));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["pc", "pcy"], Prop::Area),
    );
}

pub fn ZP5(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r_1", "b1"], Prop::Area),
    );
}

pub fn ZP6(s: &mut dyn Session) {
    println!("TODO CR33225 Linux: Error : The area of result shape is 138625, expected 197700");
    ptorus(s, "a0", 100., 20.);
    tcopy(s, "a0", "a1");
    tcopy(s, "a0", "a2");
    trotate(s, "a1", [0., 0., 0.], [1., 0., 0.], 90.);
    trotate(s, "a2", [0., 0., 0.], [0., 1., 0.], 90.);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "a0", "a1");
    two_step.result(s, "x1", Operation::Fuse);
    two_step.bop(s, "x1", "a2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(197700.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["x1", "a2"], Prop::Area),
    );
}
