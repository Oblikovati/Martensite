//! The `bcommon_2d` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn N3(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn P1(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn P2(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn P3(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn P4(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn P5(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn P6(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(143.787));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn P7(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}

pub fn P8(s: &mut dyn Session) {
    restore(s, "case_9_edge.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcommon(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Empty);
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(0), NbCount::Edges(0)],
    );
}
