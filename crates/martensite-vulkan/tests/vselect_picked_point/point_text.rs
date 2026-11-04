//! The original compares six-significant-digit Point tokens as strings, without a tolerance.
use martensite_tkernel::storage::format_general;

/// Keep all matching rows and preserve the empty-foreach case; `source_point_diagnostics(&[], "1 2 3")`.
pub(crate) fn source_point_diagnostics(points: &[[f64; 3]], expected: &str) -> Vec<String> {
    points
        .iter()
        .filter_map(|point| point_diagnostic(*point, expected))
        .collect()
}

/// Preserve the actual Point field's C-locale precision; `source_point_text([1., 2., 3.])`.
pub(crate) fn source_point_text(point: [f64; 3]) -> String {
    point.map(|value| format_general(value, 6)).join(" ")
}

fn point_diagnostic(point: [f64; 3], expected: &str) -> Option<String> {
    let actual = source_point_text(point);
    // The source regexp excludes non-numeric %g words, so nan/inf do not enter its foreach.
    if actual.split(' ').any(|word| !source_coordinate_token(word)) {
        return None;
    }
    (actual != expected)
        .then(|| format!("Error: Calculate point ({actual}) is not equal to target ({expected})"))
}

fn source_coordinate_token(word: &str) -> bool {
    !word.is_empty()
        && word
            .bytes()
            .all(|byte| byte.is_ascii_digit() || b"-+.eE".contains(&byte))
}

#[cfg(test)]
#[path = "point_text_tests.rs"]
mod tests;
