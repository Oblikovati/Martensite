//! Protocol controls are not witnesses for the concrete Face or Solid geometry stages.
use super::{BuilderArea, BuilderAreaHooks, perform_area_stages};
use crate::bop_algo::{AlertNullInputShapes, alerts::alert};
use martensite_tkernel::{message::ProgressRange, standard::Failure};

struct RecordingAreaReceiver {
    area: BuilderArea,
    calls: Vec<u8>,
    report_at: Option<u8>,
}

impl RecordingAreaReceiver {
    fn new(report_at: Option<u8>) -> Self {
        Self {
            area: BuilderArea::default(),
            calls: Vec::new(),
            report_at,
        }
    }

    fn record(&mut self, stage: u8, range: &ProgressRange<'_>) -> Result<(), Failure> {
        assert!(!range.user_break());
        self.calls.push(stage);
        if self.report_at == Some(stage) {
            self.area
                .options_mut()
                .add_error(&alert(AlertNullInputShapes));
        }
        Ok(())
    }
}

impl BuilderAreaHooks for RecordingAreaReceiver {
    fn area(&self) -> &BuilderArea {
        // The generic driver must inspect the receiver's actual report, not a base copy.
        &self.area
    }

    fn area_mut(&mut self) -> &mut BuilderArea {
        // Preparation belongs to the actual receiver, using this same owned area state.
        &mut self.area
    }

    fn perform_shapes_to_avoid(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let stage: u8 = 0;
        self.record(stage, range)
    }

    fn perform_loops(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let stage: u8 = 1;
        self.record(stage, range)
    }

    fn perform_areas(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let stage: u8 = 2;
        self.record(stage, range)
    }

    fn perform_internal_shapes(&mut self, range: &ProgressRange<'_>) -> Result<(), Failure> {
        let stage: u8 = 3;
        self.record(stage, range)
    }
}

#[test]
fn actual_area_receiver_runs_each_required_hook() {
    let mut receiver = RecordingAreaReceiver::new(None);
    receiver.area_mut().options_mut().set_run_parallel(true);
    perform_area_stages(&mut receiver, &ProgressRange::default()).unwrap();
    assert_eq!(receiver.calls, [0, 1, 2, 3]);
    assert!(receiver.area().options().run_parallel());
}

#[test]
fn actual_area_report_stops_later_hooks() {
    for stage in 0..4 {
        let mut receiver = RecordingAreaReceiver::new(Some(stage));
        perform_area_stages(&mut receiver, &ProgressRange::default()).unwrap();
        assert_eq!(receiver.calls, (0..=stage).collect::<Vec<u8>>());
        assert!(receiver.area().options().has_errors());
    }
}

#[test]
fn actual_failure_keeps_its_type_and_legacy_result() {
    use super::{classify_area_progress, legacy_unbound_area_result};
    use crate::bop_algo::{ProgressCancellation, ProgressOperation};
    let cause = Failure::with_type("Standard_DomainError", "input 7; expected wire", "owned");
    let result = classify_area_progress(
        ProgressOperation::BuilderFace,
        &crate::bop_algo::Options::new(),
        Err(cause.clone()),
        &ProgressCancellation::default(),
    );
    assert_eq!(legacy_unbound_area_result(result), Err(cause));
}

#[test]
fn actual_report_failure_keeps_legacy_ok_with_errors() {
    use super::{classify_area_progress, legacy_unbound_area_result};
    use crate::bop_algo::{Options, ProgressCancellation, ProgressOperation};
    let options = Options::new();
    options.add_error(&alert(AlertNullInputShapes));
    let result = classify_area_progress(
        ProgressOperation::BuilderFace,
        &options,
        Ok(()),
        &ProgressCancellation::default(),
    );
    assert!(is_report_only_failure(&result));
    assert_eq!(legacy_unbound_area_result(result), Ok(()));
    assert!(options.has_errors());
}

fn is_report_only_failure(result: &crate::bop_algo::ProgressCompletion) -> bool {
    use crate::bop_algo::{ProgressCompletion, ProgressFailureClass};
    matches!(
        result,
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            cause: None,
            ..
        }
    )
}
