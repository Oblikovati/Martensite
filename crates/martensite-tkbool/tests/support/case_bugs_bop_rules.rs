//! The bugs group accepts mass diagnostics only after its three higher-priority refusals.
use crate::case_bop_fatal_rules::{has_faulty_word, report_words_with};

/// Grade completed bugs reports without perf-only ignores; `accepted_bugs_bop_report("TEST COMPLETED\n")`.
pub(super) fn accepted_bugs_bop_report(reports: &str) -> bool {
    // Missing/skipped cases remain unaccepted; this wrapper never grants an exclusion.
    reports.lines().all(accepted_bugs_line)
}

fn accepted_bugs_line(line: &str) -> bool {
    let faulty = has_faulty_word(line);
    let common = crate::case_common_bop_rules::accepted_common_bop_line;
    accepted_bugs_line_with(line, faulty, common)
}

fn accepted_bugs_line_with(line: &str, faulty: bool, common: impl FnOnce(&str) -> bool) -> bool {
    if faulty
        || line.contains("failure reading attribute")
        || line.contains("Error: unsupported locale specification")
    {
        return false;
    }
    if line.contains("Relative error of mass computation") {
        return true;
    }
    common(line)
}

/// Keep bugs priority with a caller-qualified word profile; `accepted_bugs_bop_report_with(text, qualified_word)`.
#[allow(dead_code)] // Existing callers preserve the historical default predicate.
pub(crate) fn accepted_bugs_bop_report_with(
    reports: &str,
    word_character: fn(char) -> bool,
) -> bool {
    reports.lines().all(|line| {
        let faulty = report_words_with(line, word_character).any(|word| word == "Faulty");
        let common = |text: &str| {
            crate::case_common_bop_rules::accepted_inherited_common_line_with(text, word_character)
        };
        accepted_bugs_line_with(line, faulty, common)
    })
}
