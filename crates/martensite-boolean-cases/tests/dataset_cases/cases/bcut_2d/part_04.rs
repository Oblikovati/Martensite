//! The `bcut_2d` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn N2(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(472.684));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(6)],
    );
}

pub fn N3(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire4.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(517.136));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(9), NbCount::Edges(6)],
    );
}

pub fn N4(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire5.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1349.4));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(9), NbCount::Edges(7)],
    );
}

pub fn N5(s: &mut dyn Session) {
    restore(s, "case_8_solid_repaired.brep", "a");
    restore(s, "case_8_wire6.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1476.21));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(8), NbCount::Edges(8)],
    );
}

pub fn N6(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bcut(s, "result", "b", "a");
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
    restore(s, "case_9_edge1.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn N8(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(111.749));
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
    restore(s, "case_9_edge2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1045.95));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(5)],
    );
}

pub fn O1(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(206.213));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn O2(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn O3(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(81.6175));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn O4(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1045.95));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(6)],
    );
}

pub fn O5(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(270.1));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn O6(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(966.049));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn O7(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(763.834));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn O8(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn O9(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(936.857));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(7), NbCount::Edges(6)],
    );
}

pub fn P1(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(902.162));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}

pub fn P2(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(1000.74));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn P3(s: &mut dyn Session) {
    restore(s, "case_9_wire.brep", "a");
    restore(s, "case_9_wire3.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(966.049));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
}

pub fn P4(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bcut(s, "result", "b", "a");
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
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge1.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(469.89));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
}

pub fn P6(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(111.749));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn P7(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge2.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn P8(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(350.));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn P9(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn Q1(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(81.6175));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn Q2(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge4.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn Q3(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(350.));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
}

pub fn Q4(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge5.brep", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(613.678));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
}

pub fn Q5(s: &mut dyn Session) {
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire1.brep", "b");
    bcut(s, "result", "b", "a");
    checkprops(s, "result", Prop::Length, Expected::Mass(763.834));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Typed,
        &[NbCount::Vertices(5), NbCount::Edges(4)],
    );
}
