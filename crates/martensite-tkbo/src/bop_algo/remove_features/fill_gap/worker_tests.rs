//! Known box incidence and owned history pin the complete worker before ranged composition.
use super::super::fixture::gap_incidence_fixture;
use super::*;
#[test]
fn default_worker_preserves_known_source_incidence_and_history() {
    for count in [0, 1, 6] {
        let fixture = gap_incidence_fixture(count);
        let mut gap = FillGap::new(&fixture.feature, false);
        gap.perform(&fixture.connections());
        assert_eq!(gap.feature_faces.len(), count);
        assert_eq!(gap.solids.extent(), usize::from(count > 0));
        assert_eq!(gap.has_adjacent_faces, count == 1);
        assert_eq!(gap.faces.extent(), if count == 1 { 4 } else { 0 });
        let history = gap
            .history
            .get()
            .expect("history=null; expected an owned worker history");
        assert_eq!(history.has_modified(), count == 1);
        assert!(
            fixture.faces[..count]
                .iter()
                .all(|face| gap.feature_faces.contains(face))
        );
    }
}

#[test]
fn an_empty_feature_parent_pulse_is_not_a_completed_empty_worker() {
    use crate::bop_algo::{
        ProgressCompletion, progress_test_support::parent_census::parent_census_indicator,
    };
    let fixture = gap_incidence_fixture(0);
    let mut gap = FillGap::new(&fixture.feature, false);
    let (indicator, journal) = parent_census_indicator(1);
    let completion = gap.perform_on_progress(&fixture.connections(), &indicator.start());
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert!(journal.observed.load(std::sync::atomic::Ordering::SeqCst));
    assert!(!gap.has_adjacent_faces);
    assert_eq!(gap.faces.extent(), 0);
    assert!(gap.history.get().is_some());
}

use super::super::fixture::GapIncidenceFixture;
use crate::bop_algo::remove_features::helpers::explore;
use martensite_tkg3d::top_abs::ShapeEnum;
fn worker_fixture(profile: usize) -> GapIncidenceFixture {
    let mut fixture = gap_incidence_fixture(if profile == 7 { 1 } else { profile });
    if profile == 7 {
        add_malformed_neighbor(&mut fixture);
    }
    fixture
}
fn add_malformed_neighbor(fixture: &mut GapIncidenceFixture) {
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    let edges = explore(&fixture.faces[0], ShapeEnum::Edge).unwrap();
    let last = edges
        .last()
        .expect("box edges=0; expected four boundary edges");
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1., 2., 3.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    fixture
        .edge_faces
        .change_from_key(last)
        .unwrap()
        .append(vertex);
}
#[test]
fn gap_worker_is_send_and_shared_incidence_maps_are_sync() {
    static_assertions::assert_impl_all!(FillGap: Send);
    static_assertions::assert_impl_all!(Connections<'static>: Send, Sync);
}
#[test]
fn a_nested_intersection_pulse_stays_cancelled_after_raw_reconstruction() {
    use crate::bop_algo::{
        ProgressCompletion, ProgressOperation,
        progress_test_support::parent_census::parent_census_indicator,
    };
    let fixture = worker_fixture(1);
    let mut gap = FillGap::new(&fixture.feature, false);
    let (indicator, _) = parent_census_indicator(12);
    let completion = gap.perform_on_progress(&fixture.connections(), &indicator.start());
    assert!(matches!(
        completion,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::PaveFiller,
            ..
        }
    ));
    assert_eq!(gap.faces.extent(), 4);
    assert!(gap.history.get().unwrap().has_modified());
}
#[test]
fn a_caught_worker_failure_clears_raw_faces_and_retains_completed_history() {
    use crate::bop_algo::ProgressCompletion;
    use martensite_tkernel::message::ProgressRange;
    let fixture = worker_fixture(7);
    let mut gap = FillGap::new(&fixture.feature, false);
    gap.faces.add(
        fixture.faces[0].clone(),
        [fixture.faces[0].clone()].into_iter().collect(),
    );
    let completion = gap.perform_on_progress(&fixture.connections(), &ProgressRange::default());
    let cause = match completion {
        ProgressCompletion::Failed {
            cause: Some(cause), ..
        } => cause,
        other => panic!("status={other:?}; expected owned worker failure"),
    };
    assert_eq!(cause.exception_type(), "Standard_TypeMismatch");
    assert!(gap.has_adjacent_faces);
    assert_eq!(gap.faces.extent(), 0);
    let history = gap.history.get().unwrap();
    for source in fixture.faces.iter().skip(2) {
        assert_eq!(history.modified(source).unwrap().len(), 1);
    }
}

#[path = "worker_census.rs"]
mod census;
