//! Exact source rule controls run once instead of repeating in every including test host.
#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;

const FAULTY_WORD_CASES: [(&str, bool); 8] = [
    ("Faulty", true),
    ("before Faulty after", true),
    ("Error: performance Faulty failed", true),
    ("faulty", false),
    ("FAULTY", false),
    ("NotFaulty", false),
    ("Faulty_suffix", false),
    ("prefixFaulty", false),
];

#[test]
fn faulty_group_preserves_exact_case_and_word_boundaries() {
    for (value, expected) in FAULTY_WORD_CASES {
        assert_eq!(
            case_bop_fatal_rules::has_faulty_word(value),
            expected,
            "{value:?}"
        );
    }
}

#[test]
fn the_common_policy_keeps_its_case_word_rules() {
    assert!(case_bop_fatal_rules::accepted_bop_report(
        "unfailed BOPAlgoAlertFailure OK"
    ));
    assert!(!case_bop_fatal_rules::accepted_bop_report(
        "Warning: operation failed"
    ));
}

#[path = "support/case_perf_bop_rules.rs"]
mod case_perf_bop_rules;
#[path = "support/perf_bop_rule_cases.rs"]
mod perf_bop_rule_cases;

#[test]
fn completed_perf_reports_preserve_independent_native_rule_precedence() {
    for (name, report, expected) in perf_bop_rule_cases::NATIVE_POLICY_CASES {
        let completed = format!("{report}\nTEST COMPLETED\n");
        assert_eq!(
            case_perf_bop_rules::accepted_perf_bop_report(&completed),
            expected,
            "source policy control {name}: {report:?}"
        );
    }
}

#[path = "support/bugs_bop_rule_cases.rs"]
mod bugs_bop_rule_cases;
#[path = "support/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[path = "support/case_common_bop_rules.rs"]
mod case_common_bop_rules;

#[test]
fn completed_bugs_reports_preserve_independent_native_rule_precedence() {
    for (name, report, expected) in bugs_bop_rule_cases::NATIVE_BUGS_CASES {
        let completed = format!("{report}\nTEST COMPLETED\n");
        assert_eq!(
            case_bugs_bop_rules::accepted_bugs_bop_report(&completed),
            expected,
            "bugs source policy control {name}: {report:?}"
        );
    }
}
