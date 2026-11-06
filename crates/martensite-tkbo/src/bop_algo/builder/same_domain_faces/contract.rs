//! Functional and raw cancellation controls; full-scope schema is shared, never reconstructed here.
use super::fixtures::{
    SameDomainFixture, SameDomainFixtureKind as Kind, bind_image, same_domain_fixture,
};
use super::query;
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion,
    progress_test_support::{indicator, pulse_indicator},
};
use std::sync::atomic::Ordering;

fn completed_fixture(kind: Kind, parallel: bool) -> SameDomainFixture {
    let mut fixture = same_domain_fixture(kind);
    fixture.builder.options_mut().set_run_parallel(parallel);
    fixture
        .builder
        .fill_same_domain_faces(&fixture.structure)
        .unwrap();
    assert!(!fixture.builder.options().has_errors());
    fixture
}

fn assert_representatives(fixture: &SameDomainFixture) {
    let representative = fixture.sources[0].clone().into_shape();
    assert_eq!(fixture.builder.shapes_sd().len(), fixture.sources.len());
    for source in &fixture.sources {
        assert!(
            fixture
                .builder
                .shapes_sd()
                .seek(source)
                .unwrap()
                .is_same(&representative)
        );
        let images = fixture.builder.images().seek(source).unwrap();
        assert_eq!(images.len(), 1);
        assert!(images.iter().next().unwrap().is_same(&representative));
    }
    assert_representative_origins(fixture, &representative);
}

#[test]
fn ordinary_default_binds_native_planar_and_cylindrical_representatives() {
    for parallel in [false, true] {
        for kind in [Kind::Planar, Kind::Cylinder, Kind::MultiPair] {
            let fixture = completed_fixture(kind, parallel);
            assert_representatives(&fixture);
        }
    }
}

#[test]
fn no_ff_native_skip_needs_no_context_or_stage_queries() {
    let mut fixture = same_domain_fixture(Kind::NoIntersections);
    let observed = ProgressCancellation::default();
    let (progress, trace) = indicator(0);
    let actual = fixture
        .builder
        .fill_same_domain_faces_progress_raw(&fixture.structure, &progress.start(), &observed)
        .unwrap();
    assert_eq!(actual, ProgressCompletion::Completed);
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(fixture.builder.context.is_null());
    assert!(fixture.builder.images().is_empty());
}

#[test]
fn zero_task_checking_scope_closes_after_map_scope_at_native_positions() {
    let mut fixture = same_domain_fixture(Kind::Filtered);
    let observed = ProgressCancellation::default();
    let (progress, trace) = indicator(0);
    let actual = fixture
        .builder
        .fill_same_domain_faces_progress_raw(&fixture.structure, &progress.start(), &observed)
        .unwrap();
    assert_eq!(actual, ProgressCompletion::Completed);
    assert_eq!(trace.queries.load(Ordering::SeqCst), 2);
    assert!(fixture.builder.context.is_null());
    let expected = native_zero_pair_events();
    assert_eq!(*trace.events.lock().unwrap(), expected);
}

#[test]
fn native_uncancelled_serial_poll_census_is_preserved() {
    for (kind, expected) in [
        (Kind::Planar, 6),
        (Kind::Cylinder, 7),
        (Kind::SameParent, 5),
        (Kind::MultiPair, 11),
    ] {
        let mut fixture = same_domain_fixture(kind);
        let observed = ProgressCancellation::default();
        let (progress, trace) = indicator(0);
        fixture
            .builder
            .fill_same_domain_faces_progress_raw(&fixture.structure, &progress.start(), &observed)
            .unwrap();
        assert_eq!(trace.queries.load(Ordering::SeqCst), expected);
        assert!(!observed.is_cancelled());
    }
}

#[test]
fn parent_exclusion_preserves_seeded_piece_identity_and_raw_origins() {
    for parallel in [false, true] {
        let fixture = completed_fixture(Kind::SameParent, parallel);
        assert!(fixture.builder.shapes_sd().is_empty());
        assert_seeded_relations(&fixture);
    }
}

fn assert_seeded_relations(fixture: &SameDomainFixture) {
    assert_eq!(fixture.builder.images().len(), 2);
    assert_eq!(fixture.builder.origins().len(), 2);
    for (source, piece) in fixture.sources.iter().zip(&fixture.pieces) {
        let images = fixture.builder.images().seek(source).unwrap();
        assert!(images.iter().next().unwrap().is_same(piece));
        let origins = fixture.builder.origins().seek(piece).unwrap();
        assert_eq!(origins.len(), 1);
        assert!(origins.iter().next().unwrap().is_same(source));
    }
}

#[test]
fn missing_surface_candidate_is_an_ordinary_negative_not_an_invented_failure() {
    for parallel in [false, true] {
        let fixture = completed_fixture(Kind::MissingSurface, parallel);
        assert!(fixture.builder.shapes_sd().is_empty());
        assert_seeded_relations(&fixture);
        let actual = query::same_domain_pair(
            &fixture.pieces[0],
            &fixture.pieces[1],
            fixture.builder.context.get().unwrap(),
            fixture.builder.options().fuzzy_value(),
        )
        .unwrap();
        assert!(!actual);
    }
}

fn cancelled_fixture(query: usize) -> (SameDomainFixture, ProgressCompletion, bool) {
    let mut fixture = same_domain_fixture(Kind::Cylinder);
    let observed = ProgressCancellation::default();
    let (progress, _) = pulse_indicator(query);
    let actual = fixture
        .builder
        .fill_same_domain_faces_progress_raw(&fixture.structure, &progress.start(), &observed)
        .unwrap();
    (fixture, actual, observed.is_cancelled())
}

#[test]
fn collection_postjoin_and_group_cancellation_stop_at_actual_parent_poll() {
    for query in [1, 2, 3, 4, 6, 7] {
        let (fixture, actual, observed) = cancelled_fixture(query);
        assert!(matches!(actual, ProgressCompletion::Cancelled { .. }));
        assert!(observed);
        assert!(fixture.builder.options().has_errors());
        assert!(fixture.builder.images().is_empty());
        assert!(fixture.builder.shapes_sd().is_empty());
    }
}

#[test]
fn nonsticky_worker_stop_keeps_native_raw_origin_update_without_parent_alert() {
    let mut fixture = same_domain_fixture(Kind::Cylinder);
    for source in &fixture.sources {
        bind_image(&mut fixture.builder, source, source);
    }
    let observed = ProgressCancellation::default();
    let (progress, trace) = pulse_indicator(5);
    let actual = fixture
        .builder
        .fill_same_domain_faces_progress_raw(&fixture.structure, &progress.start(), &observed)
        .unwrap();
    assert!(matches!(actual, ProgressCompletion::Cancelled { .. }));
    assert!(observed.is_cancelled());
    assert!(!fixture.builder.options().has_errors());
    assert!(fixture.builder.shapes_sd().is_empty());
    assert_eq!(fixture.builder.origins().len(), 2);
    assert_eq!(trace.queries.load(Ordering::SeqCst), 6);
}

fn assert_representative_origins(fixture: &SameDomainFixture, representative: &super::Shape) {
    let origins = fixture.builder.origins().seek(representative).unwrap();
    assert_eq!(origins.len(), fixture.sources.len());
    for (actual, expected) in origins.iter().zip(&fixture.sources) {
        assert!(actual.is_same(expected));
    }
}

fn native_zero_pair_events() -> Vec<(u64, Vec<String>)> {
    vec![
        (0.0_f64.to_bits(), vec![]),
        (0.1_f64.to_bits(), vec![]),
        (
            0.4_f64.to_bits(),
            vec!["Filling same domain faces map".to_owned()],
        ),
        (1.0_f64.to_bits(), vec!["Checking SD faces".to_owned()]),
    ]
}
