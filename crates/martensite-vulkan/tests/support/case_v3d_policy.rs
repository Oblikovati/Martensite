//! The V3d group has one Faulty rule, followed by the inherited common rules.
use crate::{case_bop_fatal_rules, case_common_bop_rules, case_word_profile};

/// Grade finalized V3d bytes without an OpenGL plugin exemption; `accepted_report("TEST COMPLETED\n")`.
pub(crate) fn accepted_report(report: &str) -> bool {
    let word = case_word_profile::word_character;
    report.contains("TEST COMPLETED")
        && report.lines().all(|line| {
            let faulty =
                case_bop_fatal_rules::report_words_with(line, word).any(|part| part == "Faulty");
            !faulty && case_common_bop_rules::accepted_inherited_common_line_with(line, word)
        })
}
