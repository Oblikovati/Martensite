//! `tests/heal/split_number`: a face split into a given number of parts.
use crate::support::{
    commands::{explode, pcylinder, ttranslate},
    heal_commands::{dt_split_by_number, native_split_counts, pcylinder_on, psphere_band},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    session::Session,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The cut sphere of the `A` scripts.
fn cut_sphere(s: &mut dyn Session) {
    psphere_band(s, "a", 10., 0., 90.);
    pcylinder(s, "cc", 4., 15.);
    ttranslate(s, "cc", [0., -5., 0.]);
    bcut(s, "b", "a", "cc");
    explode(s, "b", Some(ShapeEnum::Face));
}

/// The twice-cut cylinder of the `D` scripts.
fn cut_cylinder(s: &mut dyn Session) {
    pcylinder(s, "a", 10., 50.);
    pcylinder_on(s, "b1", ([-20., 0., 10.], [1., 0., 0.]), 3., 40.);
    pcylinder_on(s, "b2", ([0., -20., 30.], [0., 1., 0.]), 5., 40.);
    bcut(s, "a", "a", "b1");
    bcut(s, "a", "a", "b2");
    explode(s, "a", Some(ShapeEnum::Face));
}

pub fn A3(s: &mut dyn Session) {
    cut_sphere(s);
    dt_split_by_number(s, "result", "b_1", (10, None));
    checknbshapes(s, "result", Counting::Typed, &[NbCount::Faces(10)]);
    native_split_counts(s, [15, 30, 10, 10, 1, 0, 0, 0, 66]);
}

pub fn A4(s: &mut dyn Session) {
    cut_sphere(s);
    dt_split_by_number(s, "result", "b_1", (100, None));
    checknbshapes(s, "result", Counting::Typed, &[NbCount::Faces(101)]);
    native_split_counts(s, [114, 233, 101, 101, 1, 0, 0, 0, 550]);
}

pub fn D1(s: &mut dyn Session) {
    cut_cylinder(s);
    dt_split_by_number(s, "result", "a_1", (10, None));
    checknbshapes(s, "result", Counting::Typed, &[NbCount::Faces(6)]);
}

pub fn D2(s: &mut dyn Session) {
    cut_cylinder(s);
    dt_split_by_number(s, "result", "a_1", (100, None));
    checknbshapes(s, "result", Counting::Typed, &[NbCount::Faces(99)]);
}
