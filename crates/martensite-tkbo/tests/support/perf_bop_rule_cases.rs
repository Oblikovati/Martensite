//! These input/outcome pairs were independently checked through the unchanged source classifier.
pub(super) const NATIVE_POLICY_CASES: [(&str, &str, bool); 16] = [
    ("benign", "Warning: small edge", true),
    ("performance_ignore", "Error: performance regressed", true),
    ("performance_case", "Error: Performance regressed", false),
    (
        "faulty_before_ignore",
        "Error: performance Faulty failed",
        false,
    ),
    (
        "faulty_identifier",
        "Error: performance Faulty_suffix failed",
        true,
    ),
    ("faulty_lowercase", "Error: performance faulty failed", true),
    (
        "other_line_fatal",
        "Error: performance regressed\nWarning: operation failed",
        false,
    ),
    (
        "mass_ignore",
        "Relative error of mass computation : 0.01",
        true,
    ),
    (
        "mass_faulty",
        "Relative error of mass computation : Faulty",
        false,
    ),
    (
        "mass_failure_ignored",
        "Relative error of mass computation : failed",
        true,
    ),
    (
        "duplicate_exception",
        "script Exception: ** Exception **",
        true,
    ),
    (
        "missing_file",
        "script Exception: File sample could not be found",
        false,
    ),
    (
        "missing_before_mass_ignore",
        "Relative error of mass computation : script Exception: file sample could not be found",
        false,
    ),
    (
        "perf_before_missing",
        "Error: performance script Exception: file sample could not be found",
        true,
    ),
    ("cpu_limit", "Process killed by CPU limit", false),
    ("elapsed_limit", "Process killed by elapsed limit", false),
];
