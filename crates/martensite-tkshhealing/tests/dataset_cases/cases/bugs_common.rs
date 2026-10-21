//! Script lines the tests/bugs case bodies share: the `puts` banners and the readings of
//! `tolerance` and `nbshapes` results the scripts take with `regexp`.
use crate::support::{checks::nbshapes, log, sewing::tolerance_text, text::numbers_after};
use martensite_tkbrep::topods::Shape;

/// The `puts` banner lines of a script.
pub fn banner(lines: &[&str]) {
    for line in lines {
        log::puts(line);
    }
}

/// `regexp {Tolerance +MAX=([-0-9.+eE]+)} [tolerance shape]`, the result logged.
pub fn tolerance_max(shape: &Shape) -> f64 {
    let text = tolerance_text(shape);
    log::result(&text);
    numbers_after(&text, "Tolerance MAX=", 1, "").expect("Tolerance MAX=")[0]
}

/// `regexp {KIND +: +([-0-9.+eE]+)} [nbshapes shape]`, the result logged.
pub fn nbshapes_of(shape: &Shape, kind: &str) -> f64 {
    let text = nbshapes(shape);
    log::result(&text);
    let at = text
        .find(&format!("{kind} "))
        .unwrap_or_else(|| panic!("nbshapes prints no {kind}"));
    let rest = text[at + kind.len()..]
        .trim_start()
        .trim_start_matches(':')
        .trim_start();
    rest.split_whitespace().next().unwrap().parse().unwrap()
}
