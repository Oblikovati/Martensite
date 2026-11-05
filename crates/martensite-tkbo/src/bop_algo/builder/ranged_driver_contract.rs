//! Actual range/callback controls qualify orchestration only, never substitute geometry work.
use super::{BuilderProgressProfile, RangedIntersector, perform_on_progress};
use crate::bop_algo::{
    BOPAlgoBuilder, BOPAlgoPaveFiller, ProgressCompletion, ProgressOperation, ProgressStage,
};
use martensite_tkernel::message::{
    ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope,
};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct RecordedAdvance {
    name: Option<Vec<u8>>,
    position: f64,
    portion: f64,
}

#[derive(Clone, Default)]
struct RecordedBuilderProgress {
    advances: Arc<Mutex<Vec<RecordedAdvance>>>,
}

impl ProgressIndicatorHooks for RecordedBuilderProgress {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let advance = RecordedAdvance {
            name: scope.name().map(<[u8]>::to_vec),
            position,
            portion: scope.get_portion(),
        };
        self.advances.lock().unwrap().push(advance);
    }
}

fn finished_interference(
    _: &mut BOPAlgoPaveFiller,
    range: &ProgressRange<'_>,
) -> ProgressCompletion {
    let actual_child = ProgressScope::with_literal(range, "Interference callback", 1.0, false);
    drop(actual_child);
    ProgressCompletion::Completed
}

fn forward_builder_completion(
    _: &mut BOPAlgoBuilder,
    _: &mut BOPAlgoPaveFiller,
    status: ProgressCompletion,
    range: &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure> {
    let actual_child = ProgressScope::with_literal(range, "Builder callback", 1.0, false);
    drop(actual_child);
    Ok(status)
}

fn observed_profile(
    profile: BuilderProgressProfile,
    intersect: RangedIntersector,
) -> (ProgressCompletion, Vec<RecordedAdvance>) {
    let hooks = RecordedBuilderProgress::default();
    let indicator = ProgressIndicator::new(Box::new(hooks.clone()));
    let outcome = perform_on_progress(
        &mut BOPAlgoBuilder::new(),
        List::default(),
        &indicator.start(),
        profile,
        intersect,
        forward_builder_completion,
    )
    .unwrap();
    let advances = hooks.advances.lock().unwrap().clone();
    (outcome, advances)
}

fn assert_profile_weights(profile: BuilderProgressProfile, split: f64) {
    let (outcome, advances) = observed_profile(profile, finished_interference);
    assert_eq!(outcome, ProgressCompletion::Completed);
    let interference = advances
        .iter()
        .find(|row| row.name.as_deref() == Some(b"Interference callback"))
        .unwrap();
    let builder = advances
        .iter()
        .find(|row| row.name.as_deref() == Some(b"Builder callback"))
        .unwrap();
    assert_eq!(interference.position, split);
    assert_eq!(interference.portion, split);
    assert_eq!(builder.portion, 1.0 - split);
    assert_eq!(builder.position, 1.0);
}

#[test]
fn general_fuse_allocates_native_nine_and_one_steps() {
    let profile = BuilderProgressProfile::GeneralFuse;
    assert_eq!(profile.scope_name(), "Performing General Fuse operation");
    assert_profile_weights(profile, 9.0 / 10.0);
}

#[test]
fn intersecting_volume_allocates_native_nine_and_one_steps() {
    let profile = BuilderProgressProfile::MakerVolume { intersect: true };
    assert_eq!(profile.scope_name(), "Performing MakeVolume operation");
    assert_profile_weights(profile, 9.0 / 10.0);
}

#[test]
fn separated_volume_allocates_native_half_and_nine_half_steps() {
    let profile = BuilderProgressProfile::MakerVolume { intersect: false };
    assert_eq!(profile.scope_name(), "Performing MakeVolume operation");
    assert_profile_weights(profile, 0.5 / 10.0);
}

fn cancelled_interference(_: &mut BOPAlgoPaveFiller, _: &ProgressRange<'_>) -> ProgressCompletion {
    ProgressCompletion::Cancelled {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Worker,
        context: None,
    }
}

#[test]
fn presentation_reaching_one_does_not_reclassify_cancelled_child() {
    let (outcome, advances) =
        observed_profile(BuilderProgressProfile::GeneralFuse, cancelled_interference);
    assert_eq!(advances.last().unwrap().position, 1.0);
    assert!(matches!(
        outcome,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::PaveFiller,
            ..
        }
    ));
    let profile = BuilderProgressProfile::GeneralFuse;
    let _: &dyn Send = &profile;
    let _: &dyn Sync = &profile;
}
