//! The `boptuc_2d` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_1_shell.brep", "a");
    restore(s, "case_1_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(392.231));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn A9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_1_shell.brep", "a");
    restore(s, "case_1_edge4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn B1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_1_shell.brep", "a");
    restore(s, "case_1_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn B2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_edge4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(1304.85));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn B3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2302.78));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(18), NbCount::Edges(11)],
    );
}

pub fn B4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2302.78));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(18), NbCount::Edges(11)],
    );
}

pub fn B5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2329.96));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(11), NbCount::Edges(8)],
    );
}

pub fn B6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_solid_r.brep", "a");
    restore(s, "case_2_wire4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2408.18));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(11), NbCount::Edges(8)],
    );
}

pub fn B7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_shell_r.brep", "a");
    restore(s, "case_2_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(1211.16));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(7)],
    );
}

pub fn B8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_shell_r.brep", "a");
    restore(s, "case_2_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(1211.16));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(7)],
    );
}

pub fn B9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_shell_r.brep", "a");
    restore(s, "case_2_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(1304.85));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn C1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_shell_r.brep", "a");
    restore(s, "case_2_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2302.78));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(18), NbCount::Edges(11)],
    );
}

pub fn C2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_shell_r.brep", "a");
    restore(s, "case_2_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(3176.78));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(22), NbCount::Edges(22)],
    );
}

pub fn C3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_shell_r.brep", "a");
    restore(s, "case_2_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2329.96));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(11), NbCount::Edges(8)],
    );
}

pub fn C4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_face_r.brep", "a");
    restore(s, "case_2_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(1211.16));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(14), NbCount::Edges(7)],
    );
}

pub fn C5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_face_r.brep", "a");
    restore(s, "case_2_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(1304.85));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(3)],
    );
}

pub fn C6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_face_r.brep", "a");
    restore(s, "case_2_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2302.78));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(18), NbCount::Edges(11)],
    );
}

pub fn C7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_2_face_r.brep", "a");
    restore(s, "case_2_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(2329.96));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(11), NbCount::Edges(8)],
    );
}

pub fn C8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_solid.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn C9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_solid.brep", "a");
    restore(s, "case_3_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn D1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_solid.brep", "a");
    restore(s, "offset_wire_034.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn D2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_solid.brep", "a");
    restore(s, "case_3_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(103.485));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn D3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_solid.brep", "a");
    restore(s, "case_3_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(234.903));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn D4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_solid.brep", "a");
    restore(s, "case_3_wire4.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Mass(234.903));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(3)],
    );
}

pub fn D5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_solid.brep", "a");
    restore(s, "case_3_wire5.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn D6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_edge1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn D7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "case_3_shell.brep", "a");
    restore(s, "case_3_edge2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}
