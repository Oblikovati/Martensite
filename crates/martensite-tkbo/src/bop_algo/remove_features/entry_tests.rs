//! Whole-entry baselines finalize removals, unification history and the declared result type.
use super::super::{feature::fixture::parent_fixture, helpers::explore};
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};
fn configured_whole_remover(profile: usize) -> BOPAlgoRemoveFeatures {
    let prepared = parent_fixture(profile, false);
    let mut remover = BOPAlgoRemoveFeatures::new();
    remover.set_shape(prepared.input_shape());
    for feature in &prepared.features {
        remover.add_face_to_remove(feature);
    }
    remover.set_to_fill_history(profile != 7);
    remover
}
fn assert_whole_base_box(remover: &BOPAlgoRemoveFeatures) {
    assert!(!remover.options().has_errors());
    assert!(!remover.options().has_warnings());
    assert_eq!(
        remover.shape().shape_type().unwrap(),
        remover.input_shape().shape_type().unwrap()
    );
    assert_eq!(explore(remover.shape(), ShapeEnum::Face).unwrap().len(), 6);
    assert!(
        BRepCheckAnalyzer::new(remover.shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let mut properties = GProps::default();
    let error =
        BRepGProp::volume_properties_eps(remover.shape(), &mut properties, 1e-9, true, false)
            .unwrap();
    assert!(error <= 1e-9, "integration error={error}; expected <=1e-9");
    assert!(
        (properties.mass() - 500.).abs() <= 1e-9,
        "volume={}; expected 500",
        properties.mass()
    );
}
#[test]
fn whole_defaults_restore_declared_type_and_finalize_feature_deletions_when_history_is_enabled() {
    for profile in [6, 7] {
        let mut remover = configured_whole_remover(profile);
        let requested: Vec<_> = remover
            .faces_to_remove
            .iter()
            .flat_map(|feature| explore(feature, ShapeEnum::Face).unwrap())
            .collect();
        remover.perform();
        assert_whole_base_box(&remover);
        for face in requested {
            assert_eq!(remover.is_deleted(&face).unwrap(), profile != 7);
        }
    }
}

fn configured_whole_facade(profile: usize) -> crate::brep_algo_api::BRepAlgoAPIDefeaturing {
    let remover = configured_whole_remover(profile);
    let mut api = crate::brep_algo_api::BRepAlgoAPIDefeaturing::new();
    api.set_shape(remover.input_shape());
    api.add_faces_to_remove(remover.faces_to_remove());
    api.set_to_fill_history(profile != 7);
    api
}
#[test]
fn a_child_stop_survives_final_history_and_unification_even_with_legacy_done() {
    use crate::bop_algo::{
        ProgressCompletion, progress_test_support::parent_census::parent_census_indicator,
    };
    let mut api = configured_whole_facade(6);
    let (indicator, journal) = parent_census_indicator(10);
    let completion = api.build_with_progress(&indicator.start());
    assert!(journal.observed.load(std::sync::atomic::Ordering::SeqCst));
    assert!(!matches!(completion, ProgressCompletion::Completed));
    assert!(api.is_done());
    assert_eq!(explore(api.shape(), ShapeEnum::Face).unwrap().len(), 6);
    assert!(api.has_deleted());
}
#[test]
fn final_history_and_simplification_stops_retain_observation_without_a_new_query() {
    use crate::bop_algo::{
        ProgressCompletion, progress_test_support::parent_census::parent_census_indicator,
    };
    for target in [530, 610] {
        let mut api = configured_whole_facade(6);
        let (indicator, journal) = parent_census_indicator(target);
        let completion = api.build_with_progress(&indicator.start());
        assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
        assert_eq!(
            journal.queries.load(std::sync::atomic::Ordering::SeqCst),
            target
        );
        assert!(!api.is_done());
        assert!(api.shape().is_null());
        assert!(api.has_modified());
    }
}
#[test]
fn later_boundary_failure_keeps_the_first_owned_child_cause() {
    use crate::bop_algo::{ProgressFailureClass, ProgressStage};
    let mut execution = RemovalExecution::default();
    let first = ProgressCompletion::Failed {
        operation: ProgressOperation::MakerVolume,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(Failure::new("first child cause")),
        context: None,
    };
    execution.retain(first.clone());
    let remover = BOPAlgoRemoveFeatures::new();
    let later = remover.classify_removal_boundary(
        Err(Failure::new("later boundary cause")),
        &execution.observed,
    );
    assert_eq!(execution.finish(later), first);
    assert!(remover.options().has_errors());
}

#[path = "entry_fixture.rs"]
mod fixture;
#[test]
fn direct_receiver_and_facade_execute_identical_weighted_callback_programs() {
    use crate::bop_algo::progress_test_support::parent_census::{
        parent_census_events, parent_census_indicator,
    };
    for profile in 0..11 {
        for target in [0, 1] {
            let mut remover = fixture::whole_entry_receiver(profile);
            let (indicator, receiver_journal) = parent_census_indicator(target);
            let receiver = remover.perform_with_progress(&indicator.start());
            let (mut api, _) = fixture::whole_entry_facade(profile, false);
            let (indicator, facade_journal) = parent_census_indicator(target);
            let facade = api.build_with_progress(&indicator.start());
            assert_eq!(receiver, facade);
            assert_eq!(
                receiver_journal
                    .queries
                    .load(std::sync::atomic::Ordering::SeqCst),
                facade_journal
                    .queries
                    .load(std::sync::atomic::Ordering::SeqCst)
            );
            assert_eq!(
                parent_census_events(&receiver_journal),
                parent_census_events(&facade_journal)
            );
        }
    }
}

#[path = "entry_census.rs"]
mod census;

#[test]
fn facade_reuse_rejects_later_bad_input_without_renewing_legacy_done() {
    use crate::bop_algo::ProgressCompletion;
    use martensite_tkernel::message::ProgressRange;
    let mut api = configured_whole_facade(6);
    assert_eq!(
        api.build_with_progress(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
    let previous = api.shape().clone();
    let (vertex, _) = fixture::whole_entry_inputs(2);
    api.set_shape(&vertex);
    assert!(matches!(
        api.build_with_progress(&ProgressRange::default()),
        ProgressCompletion::Failed { .. }
    ));
    assert!(!api.is_done());
    assert!(api.has_errors());
    assert!(api.shape().is_same(&previous));
}
