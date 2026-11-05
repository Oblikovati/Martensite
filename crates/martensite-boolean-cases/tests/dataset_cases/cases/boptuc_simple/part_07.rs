//! The `boptuc_simple` case bodies, part 7; each statement is one line of its script.
use super::*;

pub fn ZP4(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(96.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r_1", "b1"], Prop::Area),
    );
}
