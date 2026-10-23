//! heal fix face size regression cases whose
//! inputs are in the public dataset: the grid begin points `CSF_MDTVDefaults` to
//! tests/heal/data and prints it; each case restores `a`, then the grid end runs
//! `puts [DT_ApplySeq result a MDTV ToV4]` (the ToV4 sequence of the MDTV resource file)
//! and `puts [checkshape result]`; tests/heal/end shows `result` with
//! `checkview -display result -2d`.
use crate::support::{
    checkshape::checkshape,
    log,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::{dt_apply_seq, heal_data_directory, set_resource_defaults},
};

/// The grid begin, `restore file a`, then the grid end.
fn fix_face_size(s: &mut dyn Session, file: &str, image: &str) {
    let data = heal_data_directory();
    set_resource_defaults("MDTV", &data);
    log::puts(&data.display().to_string());
    let a = restore_as(s, file, "a").shape();
    let result = set(s, "result", dt_apply_seq(&a, "MDTV", "ToV4"));
    log::puts("");
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// tests/heal/fix_face_size/A1.
pub fn a1(s: &mut dyn Session) {
    fix_face_size(s, "CCH_coco1.rle", "A1");
}

/// tests/heal/fix_face_size/A4.
pub fn a4(s: &mut dyn Session) {
    fix_face_size(s, "CFI_5_l12fgq.rle", "A4");
}

/// tests/heal/fix_face_size/A7.
pub fn a7(s: &mut dyn Session) {
    fix_face_size(s, "CIN001_a13.rle", "A7");
}

/// tests/heal/fix_face_size/A8.
pub fn a8(s: &mut dyn Session) {
    fix_face_size(s, "CIN001_b49.rle", "A8");
}

/// tests/heal/fix_face_size/A9.
pub fn a9(s: &mut dyn Session) {
    fix_face_size(s, "CIN001_b62.rle", "A9");
}

/// tests/heal/fix_face_size/B2.
pub fn b2(s: &mut dyn Session) {
    fix_face_size(s, "CTO900_buc60127-part.rle", "B2");
}

/// tests/heal/fix_face_size/B4.
pub fn b4(s: &mut dyn Session) {
    // puts "TODO OCC24156 MacOS: Faulty shapes in variables faulty_1 to faulty_" (not this platform)
    fix_face_size(s, "CTO900_pro5363c.rle", "B4");
}

/// tests/heal/fix_face_size/B8.
pub fn b8(s: &mut dyn Session) {
    fix_face_size(s, "aaa.rle", "B8");
}

/// tests/heal/fix_face_size/C1 (`bug23197` is never set, so its TODO applies).
pub fn c1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    // puts "TODO OCC24156 MacOS: Faulty shapes in variables faulty_1 to faulty_" (not this platform)
    fix_face_size(s, "rrr.rle", "C1");
}
