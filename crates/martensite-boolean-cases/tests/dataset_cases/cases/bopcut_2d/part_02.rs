//! The `bopcut_2d` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn C9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "0");
    kind_is(s, "a", ShapeEnum::Edge);
    printed_mass(s, "b", Prop::Area, "0");
    kind_is(s, "b", ShapeEnum::Edge);
}

pub fn D1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "0");
    kind_is(s, "a", ShapeEnum::Edge);
    printed_mass(s, "b", Prop::Area, "0");
    kind_is(s, "b", ShapeEnum::Edge);
}

pub fn D2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(469.89));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "0");
    kind_is(s, "a", ShapeEnum::Edge);
    printed_mass(s, "b", Prop::Area, "0");
    kind_is(s, "b", ShapeEnum::Wire);
}

pub fn D3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "0");
    kind_is(s, "a", ShapeEnum::Edge);
    printed_mass(s, "b", Prop::Area, "0");
    kind_is(s, "b", ShapeEnum::Wire);
}

pub fn D4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "0");
    kind_is(s, "a", ShapeEnum::Edge);
    printed_mass(s, "b", Prop::Area, "0");
    kind_is(s, "b", ShapeEnum::Wire);
}
