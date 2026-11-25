// SPDX-License-Identifier: GPL-2.0-only
//! Actual complete records remain required after the occurrence cache is admitted.
use super::*;

use super::super::super::super::PreparedProfile;
use crate::sketch_region_prism::region_certificate::loop_region::{
    LoopRegionCause, LoopRegionFailure,
};

fn compare_refusal(prepared: &PreparedProfile) {
    let captures = Cell::new(0);
    let owner = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    if prepared.request.integral_program.is_some() {
        compare_program(prepared, SourceAccess::Owned(&owner));
        return;
    }
    let expected = inspect_ordinary(prepared, SourceAccess::Live);
    let actual = inspect_ordinary(prepared, SourceAccess::Owned(&owner));
    assert!(matches!(
        expected.as_ref().unwrap_err().cause,
        LoopRegionCause::PairUnavailable { .. }
    ));
    assert_eq!(actual, expected);
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}
fn inspect_ordinary(
    prepared: &PreparedProfile,
    access: SourceAccess<'_>,
) -> Result<GraphRegionCertificate, LoopRegionFailure> {
    super::super::super::super::regions::inspect_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || false,
        access,
    )
}
fn compare_program(prepared: &PreparedProfile, access: SourceAccess<'_>) {
    let expected = inspect_program(prepared, SourceAccess::Live);
    let actual = inspect_program(prepared, access);
    assert!(matches!(
        expected.as_ref().unwrap_err().failure.cause,
        LoopRegionCause::PairUnavailable { .. }
    ));
    assert_eq!(actual, expected);
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}
fn inspect_program(
    prepared: &PreparedProfile,
    access: SourceAccess<'_>,
) -> Result<GraphRegionCertificate, Box<crate::sketch_region_prism::ProgramGraphFailure>> {
    super::super::super::super::regions::inspect_program_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || false,
        prepared.request.integral_program.unwrap(),
        access,
    )
}

#[test]
fn conditioned_first_and_later_metadata_refusals_keep_live_entities_and_error_order() {
    for version in 3..=6 {
        for index in [0, 2] {
            check_metadata(version, index);
        }
    }
}
fn check_metadata(version: u32, index: usize) {
    for mutation in 0..4 {
        let mut prepared = prepared(version);
        let census = &mut prepared.bridge.census;
        let record = &mut census.paired.as_mut().unwrap().occurrences[index];
        match mutation {
            0 => record.relations.clear(),
            1 => record.physical_source = usize::MAX,
            2 => record.source.reversed = !record.source.reversed,
            3 => record.source.interval[0] = -0.0,
            _ => panic!("mutation={mutation}; expected 0..4"),
        }
        compare_refusal(&prepared);
    }
}
#[test]
fn conditioned_cached_reads_preserve_parameter_axis_and_actual_query_mode_binding() {
    for version in 3..=6 {
        compare_changed_domains(version);
    }
}
fn compare_changed_domains(version: u32) {
    let prepared = prepared(version);
    let census = &prepared.bridge.census;
    let captures = Cell::new(0);
    let owner = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let owned = SourceAccess::Owned(&owner).certificates(census).unwrap();
    let live = DomainCertificates::new(census).unwrap();
    for domain in &census.domains {
        let mut changed = crate::sketch_region_certification::BoundaryDomain {
            occurrences: domain.occurrences,
            evidence: domain.evidence,
        };
        changed.occurrences.swap(0, 1);
        assert_eq!(
            owned
                .unified_certificate(&changed)
                .map(|p| p.contact_count()),
            live.unified_certificate(&changed)
                .map(|p| p.contact_count())
        );
    }
}
#[test]
fn conditioned_report_source_mode_and_policy_failures_are_not_cached_as_valid() {
    for version in 3..=6 {
        for last in [false, true] {
            check_reports(version, last);
        }
    }
}
fn check_reports(version: u32, last: bool) {
    for mutation in 0..3 {
        let mut prepared = prepared(version);
        corrupt_report(&mut prepared.bridge.census, last, mutation);
        compare_refusal(&prepared);
    }
}
fn corrupt_report(census: &mut BoundaryCensus, last: bool, mutation: usize) {
    let records = &mut census.paired.as_mut().unwrap().exclusions;
    let index = if last { records.len() - 1 } else { 0 };
    let alternate = records
        .iter()
        .find(|row| row.mode != records[index].mode)
        .unwrap()
        .mode;
    let record = &mut records[index];
    match mutation {
        0 => record.sources[0] = usize::MAX,
        1 => record.mode = alternate,
        2 => record.effective.requested_world_error *= 2.,
        _ => panic!("mutation={mutation}; expected 0..3"),
    }
}
