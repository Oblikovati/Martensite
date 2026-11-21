//! Parsing, medians and verdicts of the OCCT parity comparison.
use super::*;

fn runs(origin: &str, rounds: &[&str]) -> SideRuns {
    let mut runs = SideRuns::default();
    for round in rounds {
        runs.add_round(origin, parse_lines(origin, round).unwrap())
            .unwrap();
    }
    runs
}

#[test]
fn case_lines_parse_name_time_and_checksum() {
    let lines = parse_lines("OCCT", "datamap_bind\t15.250\t1000000\n\n").unwrap();
    assert_eq!(
        lines,
        vec![CaseLine {
            name: "datamap_bind".into(),
            nanos: 15.25,
            checksum: 1_000_000,
        }]
    );
}

#[test]
fn a_malformed_line_names_the_line_and_the_expected_shape() {
    let error = parse_lines("Rust", "datamap_bind 15.2").unwrap_err();
    let text = error.to_string();
    assert!(text.contains("\"datamap_bind 15.2\""), "{text}");
    assert!(text.contains("name<TAB>nanoseconds<TAB>checksum"), "{text}");
}

#[test]
fn the_comparison_uses_the_median_of_the_rounds() {
    let occt = runs("OCCT", &["a\t10\t7", "a\t30\t7", "a\t20\t7"]);
    let rust = runs("Rust", &["a\t5\t7", "a\t9\t7"]);
    let compared = compare(&occt, &rust, 0.05).unwrap();
    assert_eq!(compared[0].occt_ns, 20.0);
    assert_eq!(compared[0].rust_ns, 7.0);
    assert_eq!(compared[0].ratio, 0.35);
    assert_eq!(compared[0].standing, Standing::Faster);
}

#[test]
fn the_tolerance_separates_faster_parity_and_slower() {
    assert_eq!(standing(0.94, 0.05), Standing::Faster);
    assert_eq!(standing(0.95, 0.05), Standing::Parity);
    assert_eq!(standing(1.05, 0.05), Standing::Parity);
    assert_eq!(standing(1.06, 0.05), Standing::Slower);
}

#[test]
fn a_checksum_difference_between_the_sides_fails() {
    let occt = runs("OCCT", &["a\t10\t7"]);
    let rust = runs("Rust", &["a\t10\t8"]);
    let text = compare(&occt, &rust, 0.05).unwrap_err().to_string();
    assert!(
        text.contains("case a: Rust checksum 8 differs from OCCT checksum 7"),
        "{text}"
    );
}

#[test]
fn a_checksum_change_between_rounds_fails() {
    let mut occt = runs("OCCT", &["a\t10\t7"]);
    let error = occt
        .add_round("OCCT", parse_lines("OCCT", "a\t10\t9").unwrap())
        .unwrap_err();
    assert!(error.to_string().contains("reported checksum 9 after 7"));
}

#[test]
fn a_case_on_one_side_only_fails() {
    let occt = runs("OCCT", &["a\t10\t7\nb\t1\t1"]);
    let rust = runs("Rust", &["a\t10\t7\nc\t1\t1"]);
    let text = compare(&occt, &rust, 0.05).unwrap_err().to_string();
    assert!(text.contains("only in OCCT: [\"b\"]"), "{text}");
    assert!(text.contains("only in Rust: [\"c\"]"), "{text}");
}

#[test]
fn the_verdict_names_every_slower_case_with_its_ratio() {
    let occt = runs("OCCT", &["a\t10\t1\nb\t10\t1\nc\t10\t1"]);
    let rust = runs("Rust", &["a\t24\t1\nb\t10\t1\nc\t200\t1"]);
    let compared = compare(&occt, &rust, 0.05).unwrap();
    let text = verdict(&compared).unwrap_err().to_string();
    assert!(
        text.contains("2 of 3 cases are slower than OCCT: a (2.40x), c (20.00x)"),
        "{text}"
    );
    assert!(verdict(&compared[1..2]).is_ok());
}

#[test]
fn the_table_has_one_row_per_case() {
    let occt = runs("OCCT", &["a\t2\t1"]);
    let rust = runs("Rust", &["a\t1\t1"]);
    let table = render_table(&compare(&occt, &rust, 0.05).unwrap());
    assert!(
        table.ends_with("| a | 2.000 | 1.000 | 0.50 | Faster |\n"),
        "{table}"
    );
}
