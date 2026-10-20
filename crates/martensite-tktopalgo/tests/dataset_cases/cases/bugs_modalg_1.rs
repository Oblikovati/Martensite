//! `bugs_modalg_1` case bodies
//! whose inputs are in the public dataset.
use crate::support::{
    checkshape::checkshape,
    log,
    props::{Prop, checkprops},
    restore::restore_as,
    session::{Session, checkview_display, set},
    topology::{SAMEPARAMETER_TOLERANCE, checksection, mkplane, sameparameter_in_place},
};

/// `restore file var; [checkshape var]; mkplane result var; checkprops result -s area;
/// checkshape result; checkview`.
pub(crate) fn planar_face(
    s: &mut dyn Session,
    (file, var): (&str, &str),
    check_input: bool,
    area: f64,
    image: &str,
) {
    let a = restore_as(s, file, var).shape();
    if check_input {
        checkshape(&a);
    }
    match mkplane(&a) {
        Ok(face) => {
            let result = set(s, "result", face);
            checkprops(&result, Prop::Area, area);
            checkshape(&result);
            checkview_display(s, Some("result"), image);
        }
        Err(message) => {
            log::result(&message);
            // checkprops of an undefined variable: "Error: The command cannot be built".
            log::puts("Error: The command cannot be built");
        }
    }
}

/// Mkplane on the restored wire.
pub fn buc60635(s: &mut dyn Session) {
    planar_face(s, ("buc60635a.brep", "a"), true, 2035.58, "buc60635");
}

/// Sameparameter of the edge in place (default tolerance, 1e-2, 1000) must not
/// hang; its length, checkshape and checksection.
pub fn buc60896(s: &mut dyn Session) {
    let result = restore_as(s, "BUC60896_edge.rle", "result").shape();
    // `sameparameter result`: `Script::Atof("result")` is 0, so the default 1e-7.
    sameparameter_in_place(&result, SAMEPARAMETER_TOLERANCE, false);
    sameparameter_in_place(&result, 1.0e-2, false);
    sameparameter_in_place(&result, 1000.0, false);
    set(s, "result", result.clone());
    checkprops(&result, Prop::Length, 48288.3);
    checkshape(&result);
    checksection(&result);
    checkview_display(s, Some("result"), "buc60896");
}

/// `bug1013` and `bug906`: mkplane on the restored wire.
pub fn bug1013(s: &mut dyn Session) {
    planar_face(s, ("DXF906.rle", "wire"), true, 92131.6, "bug1013");
}
