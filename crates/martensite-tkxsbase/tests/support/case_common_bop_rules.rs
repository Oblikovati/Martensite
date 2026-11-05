//! Common source rows run only after the caller's higher-priority group rules.
use crate::case_bop_fatal_rules::{
    accepted_bop_report, accepted_inherited_report, accepted_inherited_report_with,
};

/// Grade one line after group precedence; `accepted_common_bop_line("Warning: small edges")`.
pub(super) fn accepted_common_bop_line(line: &str) -> bool {
    let fatal_policy = accepted_bop_report;
    accepted_common_line_with(line, fatal_policy)
}

/// Apply exact inherited common precedence without a caller's group rules; `accepted_inherited_common_line("Faulty")`.
#[allow(dead_code)] // BOP-only hosts keep their original group-aware wrapper.
pub(crate) fn accepted_inherited_common_line(line: &str) -> bool {
    let fatal_policy = accepted_inherited_report;
    accepted_common_line_with(line, fatal_policy)
}

fn accepted_common_line_with(line: &str, fatal_policy: impl FnOnce(&str) -> bool) -> bool {
    if missing_file_exception(line) {
        return false;
    }
    if line.contains("Relative error of mass computation :")
        || line.contains("script Exception: ** Exception **")
    {
        return true;
    }
    fatal_policy(line)
}

fn missing_file_exception(line: &str) -> bool {
    let Some((_, remainder)) = line.split_once("script Exception: ") else {
        return false;
    };
    remainder.match_indices("ile ").any(|(offset, _)| {
        offset > 0
            && matches!(remainder.as_bytes()[offset - 1], b'f' | b'F')
            && remainder[offset + 4..].contains(" could not be found")
    })
}

/// Grade common precedence with a caller's exact boundary profile; `accepted_inherited_common_line_with(text, qualified_word)`.
#[allow(dead_code)] // Existing BOP streams preserve their established wrapper and predicate.
pub(crate) fn accepted_inherited_common_line_with(
    line: &str,
    word_character: fn(char) -> bool,
) -> bool {
    let fatal_policy = |text: &str| accepted_inherited_report_with(text, word_character);
    accepted_common_line_with(line, fatal_policy)
}
