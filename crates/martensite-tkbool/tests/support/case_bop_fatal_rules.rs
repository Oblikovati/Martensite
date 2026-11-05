//! The unchanged inherited fatal-word policy shared by complete original command streams.
fn rejected_word(word: &str) -> bool {
    matches!(word, "Exception" | "exception" | "Faulty")
        || ["error", "failed", "failure"]
            .iter()
            .any(|expected| word.eq_ignore_ascii_case(expected))
        || (word.is_ascii()
            && word.len() == 5
            && word.as_bytes()[4].is_ascii_digit()
            && word[..4].eq_ignore_ascii_case("fail"))
}

/// Share exact rule-word boundaries with group-specific grading; `report_words("shape invalid").count()` .
pub(crate) fn report_words(response: &str) -> impl Iterator<Item = &str> {
    let word_character = rust_word_character;
    report_words_with(response, word_character)
}

fn rust_word_character(character: char) -> bool {
    let alphanumeric = character.is_alphanumeric();
    alphanumeric || character == '_'
}

/// Reuse token traversal with a qualified boundary predicate; `report_words_with(text, qualified_word)`.
pub(crate) fn report_words_with(
    response: &str,
    word_character: fn(char) -> bool,
) -> impl Iterator<Item = &str> {
    // Existing BOP callers retain their historical Rust predicate; qualified callers inject their profile.
    response.split(move |character| !word_character(character))
}

/// Preserve the perf group's case-sensitive first-priority word; `has_faulty_word("Faulty result")`.
#[allow(dead_code)] // Only perf-group callers need this rule separately from the common fatal policy.
pub(super) fn has_faulty_word(response: &str) -> bool {
    // This exact-case group rule must precede the inherited IGNORE branches.
    report_words(response).any(|word| word == "Faulty")
}

fn accepted_report_with(
    response: &str,
    rejected_word: fn(&str) -> bool,
    word_character: fn(char) -> bool,
) -> bool {
    let rejected = report_words_with(response, word_character).any(rejected_word);
    !rejected
        && !response.contains("Process killed by CPU limit")
        && !response.contains("Process killed by elapsed limit")
}

fn rejected_inherited_word(word: &str) -> bool {
    let inherited = word != "Faulty";
    inherited && rejected_word(word)
}

/// Checks inherited fatal markers for generated BOP text; `accepted_bop_report("Warning: small edges")`.
pub(crate) fn accepted_bop_report(response: &str) -> bool {
    let policy = rejected_word;
    accepted_report_with(response, policy, rust_word_character)
}

/// Apply only tests/parse.rules fatal words, without group additions; `accepted_inherited_report("Faulty")`.
#[allow(dead_code)] // Legacy BOP-only hosts do not select the inherited-only policy.
pub(crate) fn accepted_inherited_report(response: &str) -> bool {
    let policy = rejected_inherited_word;
    accepted_report_with(response, policy, rust_word_character)
}

/// Apply inherited fatal words with an explicit captured boundary profile; `accepted_inherited_report_with(text, qualified_word)`.
#[allow(dead_code)] // Historical callers deliberately retain the default predicate.
pub(crate) fn accepted_inherited_report_with(
    response: &str,
    word_character: fn(char) -> bool,
) -> bool {
    let policy = rejected_inherited_word;
    accepted_report_with(response, policy, word_character)
}
