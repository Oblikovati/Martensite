//! Independent native bugs-group outcomes retain group-before-common ordering.
pub(super) const NATIVE_BUGS_CASES: [(&str, &str, bool); 18] = [
    ("benign", "Warning: small edge", true),
    ("perf_not_ignored", "Error: performance regressed", false),
    (
        "mass_no_colon",
        "Relative error of mass computation 0.01",
        true,
    ),
    (
        "mass_failed_ignored",
        "Relative error of mass computation failed",
        true,
    ),
    (
        "mass_faulty_priority",
        "Relative error of mass computation Faulty",
        false,
    ),
    (
        "attribute_priority",
        "Relative error of mass computation failure reading attribute",
        false,
    ),
    (
        "locale_priority",
        "Relative error of mass computation Error: unsupported locale specification",
        false,
    ),
    (
        "attribute_exact_substring",
        "prefailure reading attribute",
        false,
    ),
    (
        "attribute_case_difference",
        "Relative error of mass computation Failure reading attribute",
        true,
    ),
    (
        "locale_skip",
        "Error: unsupported locale specification",
        false,
    ),
    (
        "common_duplicate",
        "script Exception: ** Exception **",
        true,
    ),
    (
        "common_missing",
        "script Exception: File fixture could not be found",
        false,
    ),
    (
        "mass_before_common_missing",
        "Relative error of mass computation script Exception: File fixture could not be found",
        true,
    ),
    (
        "other_line_failed",
        "Relative error of mass computation : 0.01\nWarning: operation failed",
        false,
    ),
    (
        "case_sensitive_mass",
        "relative error of mass computation : 0.01",
        false,
    ),
    (
        "faulty_identifier",
        "Relative error of mass computation Faulty_suffix",
        true,
    ),
    ("cpu_limit", "Process killed by CPU limit", false),
    ("elapsed_limit", "Process killed by elapsed limit", false),
];
