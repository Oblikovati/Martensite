//! Literal output predicates, including their intentionally weak newline/token contracts.
pub(super) const B1_ROW: &str =
    "b1                   Displayed                   Type:  AisShape (AisShape)";
pub(super) const B2_ROW: &str =
    "b2                   Displayed Selected Detected Type:  AisShape (AisShape)";

/// Keep the source lsearch token test; `has_token(text, "not")`.
pub(super) fn has_token(text: &str, token: &str) -> bool {
    // Match a separate output token, never a digit inside an object name.
    text.split_whitespace().any(|word| word == token)
}

/// Retain equality to the original newline-free literal; `missing_selected_literal(text)`.
pub(super) fn missing_selected_literal(text: &str) -> bool {
    // Native VState adds a newline; the original equality literal deliberately does not.
    text == "aSelected doesn't exist!"
}

/// Keep the case-sensitive source substring test; `line_was_reported(text)`.
pub(super) fn line_was_reported(text: &str) -> bool {
    // A neutral fallback includes all named datums even when no owner was selected.
    text.contains("AisLine")
}

/// Preserve the first source OR predicate; `first_face_errors(text)`.
pub(super) fn first_face_errors(text: &str) -> Vec<&'static str> {
    let rows = newline_rows(text);
    if rows.len() != 3
        || item(&rows, 0, false) != "Locally selected sub-shapes within b1:"
        || item(&rows, 1, true) != "Face #2"
    {
        return vec!["ERROR: vstate output for selected face is incorrect"];
    }
    Vec::new()
}

/// Retain printed rows before independent diagnostics; `mixed_report(text)`.
pub(super) fn mixed_report(text: &str) -> (Vec<String>, Vec<&'static str>) {
    let rows = newline_rows(text);
    let first = item(&rows, 1, true);
    let second = item(&rows, 2, true);
    let output = vec![first.to_owned(), second.to_owned(), rows.len().to_string()];
    let mut errors = Vec::new();
    if rows.len() != 6 {
        errors.push("ERROR: vstate output for shift selection is incorrect");
    }
    if first != B1_ROW && second != B1_ROW {
        errors.push("ERROR: no info about object b1 in vstate output");
    }
    if first != B2_ROW && second != B2_ROW {
        errors.push("ERROR: no info about object b2 in vstate output");
    }
    if item(&rows, 4, true) != "Face #2" {
        errors.push("ERROR: vstate output for selected face in test part 2 is incorrect");
    }
    (output, errors)
}

fn item<'a>(rows: &'a [&'a str], index: usize, trim: bool) -> &'a str {
    let value = rows.get(index).copied().unwrap_or("");
    if trim { value.trim() } else { value }
}

fn newline_rows(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    text.split('\n').collect()
}
