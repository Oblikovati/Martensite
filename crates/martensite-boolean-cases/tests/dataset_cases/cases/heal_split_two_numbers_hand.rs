//! `tests/heal/split_two_numbers`: a face split into numbers of parts along U and along V.
use crate::support::{
    commands::{explode, pcylinder, ttranslate},
    heal_commands::{dt_split_by_number, native_split_counts, psphere_band},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    session::Session,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn A4(s: &mut dyn Session) {
    psphere_band(s, "a", 10., 0., 90.);
    pcylinder(s, "cc", 4., 15.);
    ttranslate(s, "cc", [0., -5., 0.]);
    bcut(s, "b", "a", "cc");
    explode(s, "b", Some(ShapeEnum::Face));
    dt_split_by_number(s, "result", "b_1", (3, Some(5)));
    checknbshapes(s, "result", Counting::Typed, &[NbCount::Faces(16)]);
    native_split_counts(s, [23, 42, 16, 16, 1, 0, 0, 0, 98]);
}
