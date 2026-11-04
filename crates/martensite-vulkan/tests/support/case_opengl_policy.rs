//! The OpenGL original group composes its literal rules before the inherited common policy.
use crate::{case_bop_fatal_rules, case_common_bop_rules, case_word_profile};

/// Grade finalized source bytes with native marker presence; `accepted_report("TEST COMPLETED\n")`.
pub(crate) fn accepted_report(report: &str) -> bool {
    let completed = report.contains("TEST COMPLETED");
    let accepted = report.lines().all(accepted_line);
    completed && accepted
}

fn accepted_line(line: &str) -> bool {
    let word = case_word_profile::word_character;
    if case_bop_fatal_rules::report_words_with(line, word).any(|part| part == "Faulty") {
        return false;
    }
    if line.contains("Could not open: TKIVtkDraw") {
        return true;
    }
    case_common_bop_rules::accepted_inherited_common_line_with(line, word)
}
