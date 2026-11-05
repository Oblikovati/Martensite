//! Captured native regexp boundary cases distinguish nondecimal numbers, marks and supplementary scalars.
#[path = "support/case_bop_fatal_rules.rs"]
#[allow(dead_code)]
mod case_bop_fatal_rules;
#[path = "support/case_caf_rules.rs"]
mod case_caf_rules;
#[path = "support/case_common_bop_rules.rs"]
#[allow(dead_code)]
mod case_common_bop_rules;

#[test]
fn exact_unicode_boundary_controls_match_native_regexp() {
    for (body, expected) in NATIVE_UNICODE_WITNESSES {
        assert_eq!(
            case_caf_rules::accepted_caf_report(body),
            expected,
            "native Unicode case {body:?}"
        );
    }
}

#[test]
fn captured_profile_ranges_and_ascii_definition_are_consistent() {
    use case_caf_rules::caf_word_profile::tests;
    tests::complete_range_endpoints_and_ascii_word_definition_are_consistent();
}

const NATIVE_UNICODE_WITNESSES: [(&str, bool); 106] = [
    ("invalid\u{be}\nTEST COMPLETED\n", false),
    ("Error\u{be}\nTEST COMPLETED\n", false),
    ("\u{be}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{be} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{b2}\nTEST COMPLETED\n", false),
    ("Error\u{b2}\nTEST COMPLETED\n", false),
    ("\u{b2}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{b2} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{2460}\nTEST COMPLETED\n", false),
    ("Error\u{2460}\nTEST COMPLETED\n", false),
    ("\u{2460}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{2460} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{2163}\nTEST COMPLETED\n", false),
    ("Error\u{2163}\nTEST COMPLETED\n", false),
    ("\u{2163}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{2163} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{301}\nTEST COMPLETED\n", false),
    ("Error\u{301}\nTEST COMPLETED\n", false),
    ("\u{301}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{301} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{5b0}\nTEST COMPLETED\n", false),
    ("Error\u{5b0}\nTEST COMPLETED\n", false),
    ("\u{5b0}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{5b0} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{903}\nTEST COMPLETED\n", false),
    ("Error\u{903}\nTEST COMPLETED\n", false),
    ("\u{903}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{903} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid_\nTEST COMPLETED\n", true),
    ("Error_\nTEST COMPLETED\n", true),
    ("_Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation_ ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{e9}\nTEST COMPLETED\n", true),
    ("Error\u{e9}\nTEST COMPLETED\n", true),
    ("\u{e9}Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation\u{e9} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{661}\nTEST COMPLETED\n", true),
    ("Error\u{661}\nTEST COMPLETED\n", true),
    ("\u{661}Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation\u{661} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{ff10}\nTEST COMPLETED\n", true),
    ("Error\u{ff10}\nTEST COMPLETED\n", true),
    ("\u{ff10}Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation\u{ff10} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{10400}\nTEST COMPLETED\n", false),
    ("Error\u{10400}\nTEST COMPLETED\n", false),
    ("\u{10400}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{10400} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{1d7ce}\nTEST COMPLETED\n", false),
    ("Error\u{1d7ce}\nTEST COMPLETED\n", false),
    ("\u{1d7ce}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{1d7ce} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{20000}\nTEST COMPLETED\n", false),
    ("Error\u{20000}\nTEST COMPLETED\n", false),
    ("\u{20000}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{20000} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{200d}\nTEST COMPLETED\n", false),
    ("Error\u{200d}\nTEST COMPLETED\n", false),
    ("\u{200d}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{200d} ERROR\nTEST COMPLETED\n",
        true,
    ),
    ("invalid\u{345}\nTEST COMPLETED\n", false),
    ("Error\u{345}\nTEST COMPLETED\n", false),
    ("\u{345}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{345} ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{8}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\tSyntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\rSyntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{e}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{1f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        " Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "!Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{84}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{85}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{86}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{9f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{a0}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{a1}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{167f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{1680}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{1681}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{180d}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{180e}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{180f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{1fff}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{2000}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{200b}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{200c}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{2027}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{2028}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{2029}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{202a}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{202e}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{202f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{2030}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{205e}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{205f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{2060}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{2061}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{2fff}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{3000}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{3001}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{fefe}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{feff}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        true,
    ),
    (
        "\u{ff00}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{10400}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{1d7ce}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
];
