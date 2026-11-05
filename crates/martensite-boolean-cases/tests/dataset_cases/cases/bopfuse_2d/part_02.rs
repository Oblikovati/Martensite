//! The `bopfuse_2d` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn D1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Length, Expected::Mass(1694.32));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(7)],
    );
}
