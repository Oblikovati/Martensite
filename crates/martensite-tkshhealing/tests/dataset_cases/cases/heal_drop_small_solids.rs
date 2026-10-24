//! heal drop small solids regression cases:
//! each sets `cmd` (a sequence of the dataset's `res` resource file) and `ref`; the grid
//! end points `CSF_resDefaults` to the directory of `res`, runs `DT_ApplySeq result a res
//! $cmd` on test.brep, then compares `result` with the reference shape `$ref.brep`: the
//! checkshape text, the statshape tables and the printed properties (centre of gravity
//! and matrix of inertia of vprops, sprops and lprops, within 1e-8); end shows
//! `result` with `checkview -display result -2d`.
use crate::support::{
    checkshape::checkshape,
    dataset::{dataset_directory, located_bytes},
    log,
    props::Prop,
    props_full::cmpprops,
    restore::restore_as,
    session::{Session, checkview_display, set},
    swdraw::{dt_apply_seq, set_resource_defaults, statshape},
};

/// `set env(CSF_resDefaults) [file dirname [locate_data_file res]]`, with the pinned
/// member checked first.
fn res_defaults() {
    located_bytes("res");
    let member = crate::support::dataset::pinned("res");
    set_resource_defaults("res", &dataset_directory().join(member.directory));
}

/// The grid end for `cmd` and `reference`.
fn drop_small_solids(s: &mut dyn Session, cmd: &str, reference: &str, image: &str) {
    res_defaults();
    let a = restore_as(s, "test.brep", "a").shape();
    let result = set(s, "result", dt_apply_seq(&a, "res", cmd));
    let r = restore_as(s, &format!("{reference}.brep"), "r").shape();
    checkshape(&result);
    let (result_table, r_table) = (statshape(&result), statshape(&r));
    log::result(&result_table);
    log::result(&r_table);
    if result_table != r_table {
        log::puts("Error: STATSHAPE");
    }
    for (kind, name) in [
        (Prop::Volume, "VPROPS"),
        (Prop::Area, "SPROPS"),
        (Prop::Length, "LPROPS"),
    ] {
        if !cmpprops(kind, &result, &r) {
            log::puts(&format!("Error: {name}"));
        }
    }
    checkview_display(s, Some("result"), image);
}

/// drop small solids — M1.
pub fn m1(s: &mut dyn Session) {
    drop_small_solids(s, "merge0v", "test", "M1");
}

/// drop small solids — M2.
pub fn m2(s: &mut dyn Session) {
    drop_small_solids(s, "merge1v", "merge1", "M2");
}

/// drop small solids — M3.
pub fn m3(s: &mut dyn Session) {
    drop_small_solids(s, "merge2v", "merge2", "M3");
}

/// drop small solids — R1.
pub fn r1(s: &mut dyn Session) {
    drop_small_solids(s, "remove0v", "test", "R1");
}

/// drop small solids — R2.
pub fn r2(s: &mut dyn Session) {
    drop_small_solids(s, "remove1v", "remove1", "R2");
}

/// drop small solids — R3.
pub fn r3(s: &mut dyn Session) {
    drop_small_solids(s, "remove2v", "remove2", "R3");
}

/// drop small solids — R4.
pub fn r4(s: &mut dyn Session) {
    drop_small_solids(s, "remove0w", "test", "R4");
}

/// drop small solids — R5.
pub fn r5(s: &mut dyn Session) {
    drop_small_solids(s, "remove1w", "remove1", "R5");
}

/// drop small solids — R6.
pub fn r6(s: &mut dyn Session) {
    drop_small_solids(s, "remove2w", "remove2", "R6");
}

/// drop small solids — R7.
pub fn r7(s: &mut dyn Session) {
    drop_small_solids(s, "remove1vw", "remove1", "R7");
}

/// drop small solids — R8.
pub fn r8(s: &mut dyn Session) {
    drop_small_solids(s, "remove1wv", "remove1", "R8");
}
