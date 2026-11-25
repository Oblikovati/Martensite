// SPDX-License-Identifier: GPL-2.0-only
use super::super::{
    paired_execution::captured_query,
    paired_tests::{curved_pair, paired},
};
use super::*;

#[test]
fn borrowed_report_binding_matches_literal_captures_after_source_edits() {
    let mut census = paired(&curved_pair());
    assert_bindings(&census, [true, true, true]);
    census.inputs.segments[0].controls[0][0] = -0.;
    assert_bindings(&census, [true, true, true]);
    census.inputs.segments[1].controls.clear();
    assert_bindings(&census, [true, false, false]);
    census.inputs.segments[0].controls[0][0] = f64::NAN;
    assert_bindings(&census, [false, false, false]);
}

#[test]
fn borrowed_report_binding_preserves_distinct_order_and_self_source_slot() {
    let mut census = paired(&curved_pair());
    let records = &mut census.paired.as_mut().unwrap().reports;
    assert_eq!(records[0].mode, PairedQueryMode::SelfOffDiagonal);
    assert_eq!(records[1].mode, PairedQueryMode::Distinct);
    records[0].sources[1] = 1;
    records[1].sources.swap(0, 1);
    assert_bindings(&census, [true, false, true]);
}

fn assert_bindings(census: &BoundaryCensus, expected: [bool; 3]) {
    let records = &census.paired.as_ref().unwrap().reports;
    assert_eq!(records.len(), expected.len());
    for (record, expected) in records.iter().zip(expected) {
        assert_eq!(literal_report_binding(census, record), expected);
        assert_eq!(report_source_current(census, record), expected);
    }
}

fn literal_report_binding(census: &BoundaryCensus, record: &PairedStoredReport) -> bool {
    let Ok(report) = &record.outcome else {
        return false;
    };
    captured_query(census, record.sources, record.mode).is_ok_and(|query| report.source() == &query)
}
