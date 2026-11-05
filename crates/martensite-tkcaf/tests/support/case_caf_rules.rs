//! CAF group rules precede exact inherited common rules; no BOP-only Faulty rule is added.
use crate::{
    case_bop_fatal_rules::report_words_with,
    case_common_bop_rules::accepted_inherited_common_line_with,
};
#[path = "caf_word_profile.rs"]
pub(crate) mod caf_word_profile;
use caf_word_profile::word_character;

/// Grade the complete original stream after its end; `accepted_caf_report("TEST COMPLETED\n")`.
pub(crate) fn accepted_caf_report(reports: &str) -> bool {
    let completed = reports.contains("TEST COMPLETED");
    completed && reports.lines().all(accepted_caf_line)
}

fn accepted_caf_line(line: &str) -> bool {
    if line.contains("on screen buffer is used for image dump - content might be invalid")
        || ignored_presentation(line)
    {
        return true;
    }
    if report_words_with(line, word_character).any(|word| word == "invalid") {
        return false;
    }
    accepted_inherited_common_line_with(line, word_character)
}

fn ignored_presentation(line: &str) -> bool {
    let Some(tail) = line
        .trim_start_matches(caf_word_profile::space_character)
        .strip_prefix("Syntax error: '0:2' label has no ")
    else {
        return false;
    };
    tail.starts_with("presentation")
        && report_words_with(tail, word_character).next() == Some("presentation")
}
