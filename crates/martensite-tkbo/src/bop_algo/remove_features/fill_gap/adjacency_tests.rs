//! Owned box incidence laws pin source identities before introducing ranged collection.
use super::super::fixture::{
    GapIncidenceFixture as AdjacencyFixture, gap_incidence_fixture as adjacency_fixture,
};
use super::*;

fn require_box_incidence(
    fixture: &AdjacencyFixture,
    gap: &FillGap,
    adjacent: &IndexedMapOfShape,
    count: usize,
    neighbors: usize,
) {
    assert_eq!(gap.feature_faces.len(), count);
    assert_eq!(adjacent.extent(), neighbors);
    assert_eq!(gap.solids.extent(), usize::from(count > 0));
    assert!(gap.solids.iter().all(|solid| solid.is_same(&fixture.solid)));
    assert!(
        adjacent
            .iter()
            .all(|face| fixture.faces.iter().any(|source| source.is_same(face)))
    );
    assert!(
        fixture.faces[..count]
            .iter()
            .all(|face| gap.feature_faces.contains(face))
    );
    assert!(
        adjacent
            .iter()
            .all(|face| !gap.feature_faces.contains(face))
    );
}
#[test]
fn default_gap_adjacency_keeps_exact_box_source_incidence() {
    for (count, neighbors) in [(0, 0), (1, 4), (2, 4), (6, 0)] {
        let fixture = adjacency_fixture(count);
        let mut gap = FillGap::new(&fixture.feature, false);
        let adjacent = gap.find_adjacent_faces(&fixture.connections()).unwrap();
        require_box_incidence(&fixture, &gap, &adjacent, count, neighbors);
    }
}

#[test]
fn first_adjacency_pulse_keeps_cached_feature_faces_without_neighbor_work() {
    use crate::bop_algo::{
        ProgressCancellation, progress_test_support::parent_census::parent_census_indicator,
    };
    let fixture = adjacency_fixture(1);
    let mut gap = FillGap::new(&fixture.feature, false);
    let (indicator, journal) = parent_census_indicator(1);
    let observed = ProgressCancellation::default();
    let adjacent = gap
        .find_adjacent_faces_on_progress(&fixture.connections(), &indicator.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert!(journal.observed.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(adjacent.extent(), 0);
    assert_eq!(gap.feature_faces.len(), 1);
    assert_eq!(gap.solids.extent(), 0);
    assert!(!gap.has_adjacent_faces);
}

#[path = "adjacency_census.rs"]
mod census;
