// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::cell::Cell;

#[test]
fn source_pairs_and_query_modes_keep_the_first_existing_report() {
    let distinct = PairedQueryMode::Distinct;
    let self_domain = PairedQueryMode::SelfOffDiagonal;
    let keys = [
        ([2, 7], distinct),
        ([2, 7], self_domain),
        ([2, 7], distinct),
    ];
    let mut index = ReportReuseIndex::from_keys(keys);
    assert_eq!(index.find([2, 7], distinct), Some(0));
    assert_eq!(index.find([2, 7], self_domain), Some(1));
    assert_eq!(index.find([7, 2], distinct), None);
    index.remember([2, 7], distinct, 99);
    assert_eq!(index.find([2, 7], distinct), Some(0));
}

#[test]
fn cold_lookup_and_append_preserve_report_addresses() {
    let mut index = ReportReuseIndex::from_keys([]);
    for report in 0..210 {
        let sources = [report, report + 1];
        assert_eq!(index.find(sources, PairedQueryMode::Distinct), None);
        index.remember(sources, PairedQueryMode::Distinct, report);
        assert_eq!(index.find(sources, PairedQueryMode::Distinct), Some(report));
    }
}

#[test]
fn repeated_lookups_read_the_source_record_inventory_once() {
    let visits = Cell::new(0);
    let keys = (0..210).map(|slot| ([slot, slot], PairedQueryMode::SelfOffDiagonal));
    let observed = keys.inspect(|_| visits.set(visits.get() + 1));
    let mut index = ReportReuseIndex::from_keys(observed);
    for _ in 0..3 {
        for slot in 0..210 {
            assert_eq!(
                index.find([slot, slot], PairedQueryMode::SelfOffDiagonal),
                Some(slot)
            );
        }
    }
    assert_eq!(visits.get(), 210);
}
