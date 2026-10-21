//! tests/bugs/modalg_1 case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 owned by
//! TKShHealing whose inputs are in the public dataset.
use super::bugs_common::{banner, tolerance_max};
use crate::support::{
    checkshape::checkshape,
    log,
    logged_checks::checknbshapes_some,
    props::{Prop, checkprops},
    restore::restore_as,
    session::{Session, checkview_display, set},
    sewing::{checkfreebounds, checkmaxtol, sewing},
    topology::{checksection, sameparameter_in_place},
};

/// tests/bugs/modalg_1/buc60523.
/// Sewing at 1e-7 keeps two free edges.
pub fn buc60523(s: &mut dyn Session) {
    banner(&[
        "========================",
        "BUC60523",
        "========================",
    ]);
    let a = restore_as(s, "buc60523a.brep", "a").shape();
    checkshape(&a);
    let result = sewing(1.0e-7, &[&a])
        .map(|shape| set(s, "result", shape))
        .expect("checkmaxtol: result is not a shape");
    checkmaxtol(&result, "result", 0.001);
    let mut shells = [None; 9];
    shells[4] = Some(1);
    checknbshapes_some(&result, shells);
    checkfreebounds(&result, "result", 2);
    checkprops(&result, Prop::Area, 156.919);
    checkshape(&result);
    checkview_display(s, Some("result"), "buc60523");
}

/// tests/bugs/modalg_1/buc60882.
/// `fsameparameter result 1e-5` must not increase the tolerance.
pub fn buc60882(s: &mut dyn Session) {
    banner(&["============", "BUC60882", "============", ""]);
    let result = restore_as(s, "BUC60882_samepar.brep", "result").shape();
    if checkshape(&result).text.contains("Faulty") {
        log::puts(
            "Warning BUC60882: Source shape is NOT correct. It was detected by Checkshape command",
        );
    } else {
        log::puts("BUC60882 OK: Source shape is valid");
    }
    log::puts("*** Before : ");
    let max_tol = tolerance_max(&result);
    log::puts(&format!("MaxTolerance = {max_tol}"));
    if max_tol > 1. {
        log::puts("Warning : Tolerance of Source edge is more then 1.0");
    } else {
        log::puts(" Tolerance of source edge is less then 1.0");
    }
    sameparameter_in_place(&result, 1e-5, true);
    log::puts("*** After : ");
    let max_tol_1 = tolerance_max(&result);
    log::puts(&format!("MaxTolerance = {max_tol_1}"));
    if max_tol_1 > max_tol {
        log::puts(&format!(
            "Faulty BUC60882 : Tolerance of Result edge increased and it is equal to {max_tol_1}"
        ));
    } else {
        log::puts(
            " BUC60882 OK: Tolerance of Result edge is less or equal to tolerance of Sourse edge.",
        );
    }
    checkprops(&result, Prop::Length, 1768.53);
    checkshape(&result);
    log::result(&format!(" nb alone Vertices : {}\n", checksection(&result)));
    checkview_display(s, Some("result"), "buc60882");
}
