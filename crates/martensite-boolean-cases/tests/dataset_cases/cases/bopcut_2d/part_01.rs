//! The `bopcut_2d` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_edge1.brep", "b");
    two_step.bop(s, "b", "a");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(1211.16));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(7)],
    );
}

pub fn A2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_edge2.brep", "b");
    two_step.bop(s, "b", "a");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(1211.16));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(7)],
    );
}

pub fn A3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_edge3.brep", "b");
    two_step.bop(s, "b", "a");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(1304.85));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn A4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(289.851));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn A5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_wire.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(289.851));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn A6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(289.851));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn A7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_wire.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(569.46));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(10), NbCount::Edges(9)],
    );
}

pub fn A8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn A9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(1045.95));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(5)],
    );
}

pub fn B1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn B2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(1045.95));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(6)],
    );
}

pub fn B3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(966.049));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn B4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn B5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn B6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Length, Expected::Mass(966.049));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn B7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
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
}

pub fn B8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
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
}

pub fn B9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
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
}

pub fn C1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
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
}

pub fn C2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
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
}

pub fn C3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
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
}

pub fn C4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
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
}

pub fn C5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
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
}

pub fn C6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
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
}

pub fn C7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
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

pub fn C8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
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
