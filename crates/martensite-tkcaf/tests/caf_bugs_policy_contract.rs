//! Qualified bugs/caf rules retain their distinct group precedence on native Unicode witnesses.
#[path = "support/caf_word_profile.rs"]
#[allow(dead_code)] // Bugs has no CAF-only leading-space presentation ignore.
mod caf_word_profile;
#[path = "support/case_bop_fatal_rules.rs"]
#[allow(dead_code)]
mod case_bop_fatal_rules;
#[path = "support/case_bugs_bop_rules.rs"]
#[allow(dead_code)]
mod case_bugs_bop_rules;
#[path = "support/case_common_bop_rules.rs"]
#[allow(dead_code)]
mod case_common_bop_rules;

#[test]
fn qualified_bugs_caf_chain_matches_native_witnesses() {
    for (body, expected) in NATIVE_BUGS_WITNESSES {
        let completed = body
            .lines()
            .filter(|line| *line == "TEST COMPLETED")
            .count()
            == 1;
        let accepted = case_bugs_bop_rules::accepted_bugs_bop_report_with(
            body,
            caf_word_profile::word_character,
        );
        assert_eq!(
            completed && accepted,
            expected,
            "bugs/caf native body {body:?}"
        );
    }
}

const NATIVE_BUGS_WITNESSES: [(&str, bool); 115] = [
    ("invalid\u{00be}\nTEST COMPLETED\n", true),
    ("Error\u{00be}\nTEST COMPLETED\n", false),
    ("\u{00be}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{00be} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{00b2}\nTEST COMPLETED\n", true),
    ("Error\u{00b2}\nTEST COMPLETED\n", false),
    ("\u{00b2}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{00b2} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{2460}\nTEST COMPLETED\n", true),
    ("Error\u{2460}\nTEST COMPLETED\n", false),
    ("\u{2460}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{2460} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{2163}\nTEST COMPLETED\n", true),
    ("Error\u{2163}\nTEST COMPLETED\n", false),
    ("\u{2163}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{2163} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{0301}\nTEST COMPLETED\n", true),
    ("Error\u{0301}\nTEST COMPLETED\n", false),
    ("\u{0301}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{0301} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{05b0}\nTEST COMPLETED\n", true),
    ("Error\u{05b0}\nTEST COMPLETED\n", false),
    ("\u{05b0}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{05b0} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{0903}\nTEST COMPLETED\n", true),
    ("Error\u{0903}\nTEST COMPLETED\n", false),
    ("\u{0903}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{0903} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid_\nTEST COMPLETED\n", true),
    ("Error_\nTEST COMPLETED\n", true),
    ("_Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation_ ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{00e9}\nTEST COMPLETED\n", true),
    ("Error\u{00e9}\nTEST COMPLETED\n", true),
    ("\u{00e9}Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation\u{00e9} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{0661}\nTEST COMPLETED\n", true),
    ("Error\u{0661}\nTEST COMPLETED\n", true),
    ("\u{0661}Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation\u{0661} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{ff10}\nTEST COMPLETED\n", true),
    ("Error\u{ff10}\nTEST COMPLETED\n", true),
    ("\u{ff10}Error\nTEST COMPLETED\n", true),
    (
        "Syntax error: '0:2' label has no presentation\u{ff10} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{10400}\nTEST COMPLETED\n", true),
    ("Error\u{10400}\nTEST COMPLETED\n", false),
    ("\u{10400}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{10400} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{1d7ce}\nTEST COMPLETED\n", true),
    ("Error\u{1d7ce}\nTEST COMPLETED\n", false),
    ("\u{1d7ce}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{1d7ce} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{20000}\nTEST COMPLETED\n", true),
    ("Error\u{20000}\nTEST COMPLETED\n", false),
    ("\u{20000}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{20000} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{200d}\nTEST COMPLETED\n", true),
    ("Error\u{200d}\nTEST COMPLETED\n", false),
    ("\u{200d}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{200d} ERROR\nTEST COMPLETED\n",
        false,
    ),
    ("invalid\u{0345}\nTEST COMPLETED\n", true),
    ("Error\u{0345}\nTEST COMPLETED\n", false),
    ("\u{0345}Error\nTEST COMPLETED\n", false),
    (
        "Syntax error: '0:2' label has no presentation\u{0345} ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{8}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\tSyntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\rSyntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{000e}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{001f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        " Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "!Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{0084}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{0085}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{0086}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{009f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{00a0}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{00a1}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{167f}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
    ),
    (
        "\u{1680}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
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
        false,
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
        false,
    ),
    (
        "\u{200b}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
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
        false,
    ),
    (
        "\u{2029}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
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
        false,
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
        false,
    ),
    (
        "\u{2060}Syntax error: '0:2' label has no presentation ERROR\nTEST COMPLETED\n",
        false,
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
        false,
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
        false,
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
    ("Faulty\nTEST COMPLETED\n", false),
    ("Faulty\u{00be}\nTEST COMPLETED\n", false),
    (
        "Faulty Relative error of mass computation\nTEST COMPLETED\n",
        false,
    ),
    (
        "Relative error of mass computation Error\nTEST COMPLETED\n",
        true,
    ),
    (
        "Error: unsupported locale specification\nTEST COMPLETED\n",
        false,
    ),
    ("failure reading attribute\nTEST COMPLETED\n", false),
    ("Could not read file absent\nTEST COMPLETED\n", true),
    ("ordinary text\n", false),
    ("invalid\nTEST COMPLETED\n", true),
];
