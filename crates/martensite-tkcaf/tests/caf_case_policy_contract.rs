//! Native classifier witnesses distinguish CAF-only invalid/IGNORE rules from other groups.
#[path = "support/case_bop_fatal_rules.rs"]
#[allow(dead_code)]
mod case_bop_fatal_rules;
#[path = "support/case_caf_rules.rs"]
mod case_caf_rules;
#[path = "support/case_common_bop_rules.rs"]
#[allow(dead_code)]
mod case_common_bop_rules;

#[test]
fn exact_caf_and_common_priority_matches_native_witnesses() {
    for (body, accepted) in NATIVE_WITNESSES {
        assert_eq!(
            case_caf_rules::accepted_caf_report(body),
            accepted,
            "source CAF policy {body:?}"
        );
    }
}

const NATIVE_WITNESSES: [(&str, bool); 15] = [
    ("TEST COMPLETED\n", true),
    ("shape invalid\nTEST COMPLETED\n", false),
    ("shape INVALID\nTEST COMPLETED\n", true),
    ("invalid_name invalidated\nTEST COMPLETED\n", true),
    ("Faulty\nTEST COMPLETED\n", true),
    (
        "on screen buffer is used for image dump - content might be invalid ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "  Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "Syntax error: '0:3' label has no presentation\nTEST COMPLETED\n",
        false,
    ),
    (
        "Syntax error: '0:2' label has no presentations\nTEST COMPLETED\n",
        false,
    ),
    (
        "Relative error of mass computation : invalid\nTEST COMPLETED\n",
        false,
    ),
    (
        "Relative error of mass computation : 0\nTEST COMPLETED\n",
        true,
    ),
    ("Error: first command\nNo error\nTEST COMPLETED\n", false),
    ("shape is valid\n", false),
    (
        "script Exception: File missing could not be found\nTEST COMPLETED\n",
        false,
    ),
    ("failure\nTEST COMPLETED\n", false),
];
