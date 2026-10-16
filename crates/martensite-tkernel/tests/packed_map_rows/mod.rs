//! Shared exact-row decoder for the packed map fixtures.
use std::collections::BTreeMap;

pub type ExpectedRows = BTreeMap<&'static str, Vec<u64>>;

pub fn decode_expected_rows(fixture: &'static str) -> ExpectedRows {
    fixture.lines().map(decode_expected_row).collect()
}

fn decode_expected_row(line: &'static str) -> (&'static str, Vec<u64>) {
    let (label, values) = line.split_once(' ').unwrap();
    let columns = values
        .split_whitespace()
        .map(|value| value.parse().unwrap())
        .collect();
    (label, columns)
}
