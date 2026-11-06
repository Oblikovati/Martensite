//! The `bopcommon_2d` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn M1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_shell.brep", "a");
    restore(s, "case_7_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(482.392));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_face.brep", "a");
    restore(s, "case_7_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(520.393));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn M6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(181.079));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn M7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(49.966));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn M8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(3.83899));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn M9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(36.0874));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(4)],
    );
}

pub fn N1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(126.81));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(4)],
    );
}

pub fn N2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire6.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn N3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn N4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn N5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn N6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn N7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(79.9002));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn N8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn N9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn O1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(79.9002));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn O2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn O3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn O4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn O5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn O6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn O7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn O8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn O9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}
