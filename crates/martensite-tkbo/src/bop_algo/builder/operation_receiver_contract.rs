//! Actual hook boundaries stop on reports without substituting another receiver.
use super::*;
use crate::bop_algo::{
    AlertBuilderFailed, BOPAlgoBuilder, alerts::alert, perform_with_filler_progress,
};
use martensite_tkbrep::topods::Shape;

struct RecordedReceiver {
    builder: BOPAlgoBuilder,
    results: Vec<ShapeEnum>,
    reject_at: Option<ShapeEnum>,
    reject_prepare: bool,
}
impl BuilderHooks for RecordedReceiver {
    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.builder
    }
    fn prepare(&mut self) -> Result<(), Failure> {
        self.builder.prepare();
        if self.reject_prepare {
            self.builder.options().add_error(&alert(AlertBuilderFailed));
        }
        Ok(())
    }
    fn build_result(&mut self, kind: ShapeEnum, _: &DataStructure) -> Result<(), Failure> {
        self.results.push(kind);
        if self.reject_at == Some(kind) {
            self.builder.options().add_error(&alert(AlertBuilderFailed));
        }
        self.builder.build_result_default(kind)
    }
    fn perform_internal1_with_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        run_general_fuse(self, filler, range, BuildMode::Ranged)
    }
}
fn recorded_receiver() -> (RecordedReceiver, BOPAlgoPaveFiller) {
    let shapes: [Shape; 2] = [super::tests::vertex(0.), super::tests::vertex(10.)];
    let (builder, filler) = super::tests::prepared_case(&shapes);
    (
        RecordedReceiver {
            builder,
            results: Vec::new(),
            reject_at: None,
            reject_prepare: false,
        },
        filler,
    )
}
#[test]
fn all_eight_virtual_results_run_in_native_order() {
    let (mut receiver, mut filler) = recorded_receiver();
    let completion =
        perform_with_filler_progress(&mut receiver, &mut filler, &ProgressRange::default())
            .unwrap();
    assert_eq!(completion, ProgressCompletion::Completed);
    assert_eq!(receiver.results, IMAGE_KINDS);
    assert!(receiver.builder.has_filled_history());
}
#[test]
fn each_result_report_stops_before_next_stage_and_history() {
    for (index, kind) in IMAGE_KINDS.into_iter().enumerate() {
        let (mut receiver, mut filler) = recorded_receiver();
        receiver.reject_at = Some(kind);
        let completion =
            perform_with_filler_progress(&mut receiver, &mut filler, &ProgressRange::default())
                .unwrap();
        assert!(matches!(completion, ProgressCompletion::Failed { .. }));
        assert_eq!(receiver.results, IMAGE_KINDS[..=index]);
        assert!(!receiver.builder.has_filled_history());
    }
}
#[test]
fn prepare_and_argument_reports_cannot_complete_or_start_images() {
    for reject_prepare in [false, true] {
        let (mut receiver, mut filler) = recorded_receiver();
        receiver.reject_prepare = reject_prepare;
        if !reject_prepare {
            receiver.builder.clear();
        }
        let completion =
            perform_with_filler_progress(&mut receiver, &mut filler, &ProgressRange::default())
                .unwrap();
        assert!(matches!(completion, ProgressCompletion::Failed { .. }));
        assert!(receiver.results.is_empty());
        assert!(!receiver.builder.has_filled_history());
    }
}
