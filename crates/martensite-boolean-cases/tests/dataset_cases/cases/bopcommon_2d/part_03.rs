//! The `bopcommon_2d` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn H4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_5_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(887.185));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn H5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(829.541));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn H6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1747.73));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn H7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1747.73));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn H8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
}

pub fn H9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_5_wire4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
}

pub fn I1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_5_wire5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(3000.52));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(2)],
    );
}

pub fn I2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_shell.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(887.185));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn I3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_shell.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(829.541));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn I4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_shell.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1747.73));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn I5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_shell.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
}

pub fn I6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_shell.brep", "a");
    restore(s, "case_5_wire5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(3000.52));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(2)],
    );
}

pub fn I7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_face.brep", "a");
    restore(s, "case_4_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(887.185));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn I8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_face.brep", "a");
    restore(s, "case_4_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(829.541));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn I9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_face.brep", "a");
    restore(s, "case_4_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1747.73));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn J1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_5_face.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
}

pub fn K8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
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

pub fn K9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_edge2.brep", "b");
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

pub fn L1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
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

pub fn L2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_edge4.brep", "b");
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

pub fn L3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
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

pub fn L4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_wire2.brep", "b");
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

pub fn L5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
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

pub fn L6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_solid.brep", "a");
    restore(s, "case_7_wire4.brep", "b");
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

pub fn L7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_shell.brep", "a");
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

pub fn L8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_shell.brep", "a");
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

pub fn L9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_7_shell.brep", "a");
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
