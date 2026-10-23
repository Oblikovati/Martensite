//! Shared face construction acceptance checks, which every face case
//! runs: when `result` is drawn, `puts [checkshape result]`, `puts [tolerance result]`
//! and the face, edge and vertex maximum tolerances against the `MaxFTol`, `MaxETol` and
//! `MaxVTol` the case sets (1% relative slack above a nonzero reference, exactly 0 for a
//! zero one), then `checkview -display result -2d`; otherwise `Error : The mkface can not
//! be built.`.
use crate::support::{
    checkshape::checkshape, log, session::checkview_display, sewing::tolerance_text,
    text::numbers_after, vars::Vars,
};

/// The `set MaxFTol/MaxETol/MaxVTol` values of a case (`None` when not set).
#[derive(Clone, Copy, Default)]
pub struct MaxTolerances {
    pub face: Option<f64>,
    pub edge: Option<f64>,
    pub vertex: Option<f64>,
}

/// One `if { [info exists MaxXTol] } { ... }` block of the end script.
fn check_max(reference: Option<f64>, measured: f64, wrong: &str, equal: &str) {
    let Some(reference) = reference else { return };
    if reference != 0. && measured > reference && (reference - measured).abs() / reference > 0.01 {
        log::puts(&format!(
            "Error : The resulting shape is WRONG. MAX tolerance of {wrong} should be equal to {reference} instead of {measured}"
        ));
    } else if reference == 0. && measured != 0. {
        log::puts("Error : The resulting shape is WRONG.");
    } else {
        log::puts(&format!(
            "MAX tolerance of {equal} of the resulting shape is equal to {measured}"
        ));
    }
}

/// The `regexp { *KIND +: +MAX=([-0-9.+eE]+)}` value of a `tolerance` text.
fn max_of(text: &str, kind: &str) -> f64 {
    let line = text
        .lines()
        .find(|line| line.trim_start().starts_with(kind))
        .unwrap_or_else(|| panic!("tolerance prints no {kind} row"));
    numbers_after(line, "MAX=", 1, "").expect("MAX=")[0]
}

/// The mkface group end.
pub fn mkface_end(v: &mut Vars, max: MaxTolerances, image: &str) {
    if !v.is_drawn("result") {
        log::puts("Error : The mkface can not be built.");
        return;
    }
    let result = v.shape("result");
    checkshape(&result);
    let tolerance = tolerance_text(&result);
    log::result(&tolerance);
    log::puts(&tolerance);
    check_max(max.face, max_of(&tolerance, "FACE"), "a FACE", "a FACE");
    check_max(max.edge, max_of(&tolerance, "EDGE"), "an EDGE", "an EDGE");
    check_max(
        max.vertex,
        max_of(&tolerance, "VERTEX"),
        "VERTEX",
        "a VERTEX",
    );
    checkview_display(v.session(), Some("result"), image);
}
