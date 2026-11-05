//! Actual solids and a custom virtual hook verify receiver selection and typed stops.
use super::*;
use crate::int_tools::Context;
use martensite_tkernel::standard::Handle;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
fn solid_case() -> (BOPAlgoBuilder, DataStructure) {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    let solid = maker.shape().unwrap().clone();
    let mut structure = DataStructure::default();
    structure.set_arguments(&[solid]);
    structure.init(1e-7).unwrap();
    let mut builder = BOPAlgoBuilder::new();
    builder.context = Handle::new(Context::default());
    (builder, structure)
}
#[test]
fn actual_solid_composition_reaches_all_named_child_scopes() {
    let (mut builder, mut structure) = solid_case();
    let (progress, trace) = crate::bop_algo::progress_test_support::indicator(0);
    let completion = super::super::fill_images_solids_with_progress(
        &mut builder,
        &mut structure,
        &progress.start(),
    )
    .unwrap();
    assert_eq!(completion, ProgressCompletion::Completed);
    assert!(!builder.options().has_errors());
    assert!(builder.images().is_empty());
    let events = trace.events.lock().unwrap();
    assert_eq!(events.last().unwrap().0, 1f64.to_bits());
    assert!(
        events
            .iter()
            .any(|(_, names)| names.iter().any(|name| name == "Building splits of solids"))
    );
}
struct LegacyReceiver {
    builder: BOPAlgoBuilder,
    called: bool,
}
impl BuilderHooks for LegacyReceiver {
    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.builder
    }
    fn fill_in_3d_parts(
        &mut self,
        _: &mut DataStructure,
        _: &mut SameDomainShapes,
    ) -> Result<(), Failure> {
        self.called = true;
        Ok(())
    }
}
#[test]
fn legacy_receiver_is_called_and_unknown_ranged_receiver_is_incomplete() {
    let (builder, mut structure) = solid_case();
    let mut hooks = LegacyReceiver {
        builder,
        called: false,
    };
    super::super::fill_images_solids_with(&mut hooks, &mut structure).unwrap();
    assert!(hooks.called);
    hooks.called = false;
    let completion = super::super::fill_images_solids_with_progress(
        &mut hooks,
        &mut structure,
        &ProgressRange::default(),
    )
    .unwrap();
    assert!(matches!(completion, ProgressCompletion::Incomplete { .. }));
    assert!(!hooks.called);
}
#[test]
fn no_solid_is_completed_without_invoking_classifier() {
    let mut hooks = LegacyReceiver {
        builder: BOPAlgoBuilder::new(),
        called: false,
    };
    assert_eq!(
        super::super::fill_images_solids_with_progress(
            &mut hooks,
            &mut DataStructure::default(),
            &ProgressRange::default()
        )
        .unwrap(),
        ProgressCompletion::Completed
    );
    assert!(!hooks.called);
}
#[test]
fn actual_solid_parent_cancellation_stops_later_children() {
    let (mut builder, mut structure) = solid_case();
    let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(1);
    let completion = super::super::fill_images_solids_with_progress(
        &mut builder,
        &mut structure,
        &progress.start(),
    )
    .unwrap();
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(builder.images().is_empty());
}

#[test]
fn all_known_base_classifier_receivers_dispatch_their_ranged_hook() {
    use crate::bop_algo::{
        BOPAlgoBOP, BOPAlgoCellsBuilder, BOPAlgoMakerVolume, BOPAlgoSection, BOPAlgoSplitter,
    };
    assert_base_receiver(BOPAlgoBOP::new());
    assert_base_receiver(BOPAlgoCellsBuilder::new());
    assert_base_receiver(BOPAlgoMakerVolume::new());
    assert_base_receiver(BOPAlgoSplitter::new());
    assert_base_receiver(BOPAlgoSection::new());
}
fn assert_base_receiver<H: BuilderHooks>(mut hooks: H) {
    let (_, mut structure) = solid_case();
    hooks.general_fuse().context = Handle::new(Context::default());
    let completion = super::super::fill_images_solids_with_progress(
        &mut hooks,
        &mut structure,
        &ProgressRange::default(),
    )
    .unwrap();
    assert_eq!(completion, ProgressCompletion::Completed);
    assert!(!hooks.general_fuse().options().has_errors());
}
#[test]
fn a_child_report_cannot_be_returned_as_completed() {
    let (mut builder, mut structure) = solid_case();
    let completion = fill_solids_using(
        &mut builder,
        &mut structure,
        &ProgressRange::default(),
        SolidDispatchMode::Ranged,
        |actual, _, _, _| {
            actual.options().add_error(&crate::bop_algo::alerts::alert(
                crate::bop_algo::AlertBuilderFailed,
            ));
            Ok(ProgressCompletion::Completed)
        },
    )
    .unwrap();
    assert!(matches!(
        completion,
        ProgressCompletion::Failed {
            class: crate::bop_algo::ProgressFailureClass::AlgorithmReport,
            ..
        }
    ));
}

#[test]
fn unbound_adapter_preserves_legacy_result_semantics_without_weakening_ranged_stops() {
    let stop = ProgressCompletion::Incomplete {
        operation: ProgressOperation::Builder,
        stage: crate::bop_algo::ProgressStage::Images,
        class: crate::bop_algo::ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    };
    assert_eq!(
        retain_child(stop.clone(), SolidDispatchMode::Unbound).unwrap(),
        ProgressCompletion::Completed
    );
    assert_eq!(
        retain_child(stop.clone(), SolidDispatchMode::Ranged).unwrap(),
        stop
    );
}
