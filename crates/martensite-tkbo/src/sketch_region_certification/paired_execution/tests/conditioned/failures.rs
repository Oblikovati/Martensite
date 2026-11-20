// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use certification::paired_types::PairedQueryMode;

fn failing(case: u8, failure: u8) -> BoundaryCensus {
    let mut census = fresh(case, false);
    match failure {
        0 => census.inputs.segments[0].controls.clear(),
        1 => census.paired.as_mut().unwrap().requested.version = 99,
        2 => census.paired.as_mut().unwrap().requested.signs.version = 99,
        _ => panic!("failure={failure}; expected 0..3"),
    }
    census
}
#[test]
fn conditioned_capture_policy_and_sign_failures_reuse_without_retry_or_extra_work() {
    for case in 0..3 {
        for failure in 0..3 {
            let old = twice(failing(case, failure), &mut LiteralLookup::default());
            let new = twice(failing(case, failure), &mut ReportReuseIndex::from_keys([]));
            assert_eq!(old, new);
        }
    }
}
fn twice(mut census: BoundaryCensus, lookup: &mut impl ReportLookup) -> Vec<String> {
    let mut polls = 0;
    let first = report_slot(observed_evaluation(&mut census, lookup, &mut polls));
    let after_first = polls;
    let second = report_slot(observed_evaluation(&mut census, lookup, &mut polls));
    assert_eq!(polls, after_first);
    assert_eq!((first.0, first.1), (0, false));
    assert_eq!((second.0, second.1), (0, true));
    assert_eq!(census.paired.as_ref().unwrap().exclusions.len(), 1);
    let mut rows = snapshot::census_rows(&census);
    rows.push(format!("references {first:?} {second:?}"));
    rows
}
fn observed_evaluation(
    census: &mut BoundaryCensus,
    lookup: &mut impl ReportLookup,
    polls: &mut usize,
) -> DomainEvidence {
    evaluate_pair(
        census,
        [0, 0],
        &mut || {
            *polls += 1;
            false
        },
        lookup,
    )
}
fn report_slot(evidence: DomainEvidence) -> (usize, bool, [usize; 2]) {
    match evidence {
        DomainEvidence::BernsteinReport {
            report,
            reused,
            source_slots,
        } => (report, reused, source_slots),
        _ => panic!("expected actual conditioned report reference"),
    }
}
fn seeded(case: u8) -> BoundaryCensus {
    let mut census = failing(case, 1);
    let mode = PairedQueryMode::SelfOffDiagonal;
    let mut first = ReportReuseIndex::from_keys([]);
    evaluate_pair(&mut census, [0, 0], &mut || false, &mut first);
    census.paired.as_mut().unwrap().requested.version = 2;
    let mut duplicate = ReportReuseIndex::from_keys([]);
    evaluate_pair(&mut census, [0, 0], &mut || false, &mut duplicate);
    assert_eq!(census.paired.as_ref().unwrap().exclusions[0].mode, mode);
    census
}
#[test]
fn seeded_conditioned_duplicate_keys_keep_first_failure_and_selected_vector_indices() {
    for case in 0..3 {
        check_seeded(case);
    }
}
fn check_seeded(case: u8) {
    let mut census = seeded(case);
    let mut index = ReportReuseIndex::capture(&census);
    assert!(matches!(index, ReportReuseIndex::IndexedMap(_)));
    let before = snapshot::census_rows(&census);
    let work = census.bisections;
    let evidence = evaluate_pair(
        &mut census,
        [0, 0],
        &mut || panic!("seeded retry"),
        &mut index,
    );
    let reference = report_slot(evidence);
    assert_eq!((reference.0, reference.1), (0, true));
    assert_eq!(census.paired.as_ref().unwrap().exclusions.len(), 2);
    assert_seeded_outcomes(&census);
    assert_eq!(census.bisections, work);
    assert_eq!(census.paired.as_ref().unwrap().reused_reports, 1);
    assert_eq!(&before[1..], &snapshot::census_rows(&census)[1..]);
}

fn assert_seeded_outcomes(census: &BoundaryCensus) {
    use certification::conditioned_stage::ConditionedCompletion;
    let records = &census.paired.as_ref().unwrap().exclusions;
    assert_eq!(
        records[0].outcome.as_ref().unwrap().exclusion.completion(),
        ConditionedCompletion::Invalid
    );
    assert_eq!(
        records[1].outcome.as_ref().unwrap().exclusion.completion(),
        ConditionedCompletion::Complete
    );
}

#[test]
fn malformed_mixed_vectors_never_cross_seed_selected_report_addresses() {
    let mut census = seeded(0);
    let generic = cold_execution(&census, [0, 1], PairedQueryMode::Distinct, &mut || false);
    census.paired.as_mut().unwrap().reports.push(generic);
    let mut conditioned = ReportReuseIndex::capture(&census);
    assert!(matches!(conditioned, ReportReuseIndex::IndexedMap(_)));
    assert_eq!(
        conditioned.find([0, 0], PairedQueryMode::SelfOffDiagonal),
        Some(0)
    );
    assert_eq!(conditioned.find([0, 1], PairedQueryMode::Distinct), None);
    assert!(certification::DomainCertificates::new(&census).is_none());
    census.paired.as_mut().unwrap().exclusion_version = None;
    let mut generic = ReportReuseIndex::capture(&census);
    assert_eq!(generic.find([0, 1], PairedQueryMode::Distinct), Some(0));
    assert_eq!(generic.find([0, 0], PairedQueryMode::SelfOffDiagonal), None);
    assert!(certification::DomainCertificates::new(&census).is_none());
}
#[test]
fn conditioned_cold_unique_uses_only_selected_emptiness_and_complete_identity_inventory() {
    let mut census = fresh(0, false);
    let generic = cold_execution(&census, [0, 1], PairedQueryMode::Distinct, &mut || false);
    census.paired.as_mut().unwrap().reports.push(generic);
    assert!(matches!(
        ReportReuseIndex::capture(&census),
        ReportReuseIndex::ColdUnique
    ));
    census.paired.as_mut().unwrap().occurrences.pop();
    assert!(matches!(
        ReportReuseIndex::capture(&census),
        ReportReuseIndex::IndexedMap(_)
    ));
    let mut census = fresh(0, false);
    census.paired.as_mut().unwrap().occurrences[0].physical_source = 1;
    assert!(matches!(
        ReportReuseIndex::capture(&census),
        ReportReuseIndex::IndexedMap(_)
    ));
}
