//! The work column: its pairs, its rejection of malformed pairs and its table.
use super::{WorkValue, parse_work, render_work_table};

fn value(name: &str, value: f64) -> WorkValue {
    WorkValue {
        name: name.into(),
        value,
    }
}

#[test]
fn pairs_parse_in_their_printed_order() {
    let work = parse_work("buffers=2 upload_kib=0.5").unwrap();
    assert_eq!(work, vec![value("buffers", 2.0), value("upload_kib", 0.5)]);
}

#[test]
fn an_empty_field_has_no_counters() {
    assert_eq!(parse_work(""), Some(vec![]));
}

#[test]
fn a_pair_without_a_number_or_a_name_is_malformed() {
    assert_eq!(parse_work("buffers"), None);
    assert_eq!(parse_work("buffers=two"), None);
    assert_eq!(parse_work("=2"), None);
}

#[test]
fn the_table_lists_each_reporting_case_under_the_first_case_columns() {
    let first = [value("buffers", 2.0), value("scene_ms", 1.25)];
    let second = [value("scene_ms", 0.5)];
    let rows = [("a", &first[..]), ("quiet", &[][..]), ("b", &second[..])];
    let table = render_work_table(&rows).unwrap();
    assert!(
        table.starts_with("| Case | buffers | scene_ms |\n| --- | ---: | ---: |\n"),
        "{table}"
    );
    assert!(table.contains("| a | 2.00 | 1.25 |\n"), "{table}");
    assert!(table.ends_with("| b | - | 0.50 |\n"), "{table}");
    assert!(!table.contains("quiet"), "{table}");
}

#[test]
fn no_table_when_no_case_reported_work() {
    assert_eq!(render_work_table(&[("a", &[][..])]), None);
}
