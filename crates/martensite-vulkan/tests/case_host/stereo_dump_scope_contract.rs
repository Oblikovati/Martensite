use super::super::{SessionRun, Stage};
use super::*;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::null_driver::NullGraphicDriver;
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};

#[derive(Clone, Copy)]
enum FakeCaptureOutcome {
    Complete(bool),
    Error(bool),
    Panic(bool),
}

#[derive(Debug)]
struct FakeDumpFailure;
impl fmt::Display for FakeDumpFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("owned stereo dump failure")
    }
}
impl Error for FakeDumpFailure {}

struct FakeStageCapture {
    run: SessionRun,
    outcome: FakeCaptureOutcome,
    seen: Vec<V3dStereoDumpOptions>,
}

impl FakeStageCapture {
    fn new(option: V3dStereoDumpOptions, outcome: FakeCaptureOutcome) -> Self {
        let session =
            ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 37, 19).unwrap();
        session.set_stereo_dump(option);
        Self {
            run: SessionRun {
                #[cfg(feature = "vertex-output-diagnostics")]
                vertex_output: None,
                stage: Some(Stage::Open(Handle::default(), session)),
                pending_witness_error: None,
                observed_dump_ordinal: 0,
            },
            outcome,
            seen: Vec::new(),
        }
    }
}

impl StereoDumpTarget for FakeStageCapture {
    type Output = bool;
    fn projection(&mut self) -> Option<V3dStereoDumpOptions> {
        self.run.projection()
    }
    fn set_projection(&mut self, option: V3dStereoDumpOptions) {
        self.run.set_projection(option)
    }
    fn capture_dump(&mut self, _: &'static str) -> Result<bool, Box<dyn Error>> {
        self.seen.push(self.run.projection().unwrap());
        match self.outcome {
            FakeCaptureOutcome::Complete(result) => Ok(result),
            FakeCaptureOutcome::Error(consumed) => {
                if consumed {
                    self.run.stage.take();
                }
                Err(Box::new(FakeDumpFailure))
            }
            FakeCaptureOutcome::Panic(consumed) => {
                if consumed {
                    self.run.stage.take();
                }
                panic!("owned stereo unwind")
            }
        }
    }
}

#[test]
fn stereo_scope_success_and_false_restore_the_surviving_prior_option() {
    for result in [true, false] {
        let mut target = FakeStageCapture::new(
            V3dStereoDumpOptions::RightEye,
            FakeCaptureOutcome::Complete(result),
        );
        assert_eq!(
            with_stereo_dump(&mut target, "owned.png", V3dStereoDumpOptions::Blended).unwrap(),
            result
        );
        assert_eq!(target.projection(), Some(V3dStereoDumpOptions::RightEye));
        assert_eq!(target.seen, [V3dStereoDumpOptions::Blended]);
    }
}

#[test]
fn stereo_scope_error_restores_only_a_surviving_owner_and_returns_the_case_cause() {
    for consumed in [false, true] {
        let mut target = FakeStageCapture::new(
            V3dStereoDumpOptions::LeftEye,
            FakeCaptureOutcome::Error(consumed),
        );
        let result = with_stereo_dump(&mut target, "owned.png", V3dStereoDumpOptions::Blended);
        assert!(
            result
                .err()
                .unwrap()
                .downcast_ref::<FakeDumpFailure>()
                .is_some()
        );
        assert_eq!(
            target.projection(),
            (!consumed).then_some(V3dStereoDumpOptions::LeftEye)
        );
    }
}

#[test]
fn stereo_scope_unwind_restores_survivors_without_recreating_consumed_owners() {
    for consumed in [false, true] {
        let mut target = FakeStageCapture::new(
            V3dStereoDumpOptions::RightEye,
            FakeCaptureOutcome::Panic(consumed),
        );
        let result = catch_unwind(AssertUnwindSafe(|| {
            with_stereo_dump(&mut target, "owned.png", V3dStereoDumpOptions::Blended)
        }));
        assert!(result.is_err());
        assert_eq!(
            target.projection(),
            (!consumed).then_some(V3dStereoDumpOptions::RightEye)
        );
    }
}

#[test]
fn stereo_scope_absent_real_stage_returns_an_error_without_panicking() {
    let mut run = SessionRun {
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output: None,
        stage: None,
        pending_witness_error: None,
        observed_dump_ordinal: 0,
    };
    let result = run.dump_stereo("owned.png", V3dStereoDumpOptions::Blended);
    let error = result.expect_err("absent Stage must refuse the request");
    assert!(error.downcast_ref::<std::io::Error>().is_some());
}

#[test]
fn stereo_scope_consumed_real_stage_keeps_the_case_error_instead_of_panicking() {
    let mut target =
        FakeStageCapture::new(V3dStereoDumpOptions::Mono, FakeCaptureOutcome::Error(true));
    let result = with_stereo_dump(&mut target, "owned.png", V3dStereoDumpOptions::Blended);
    assert!(
        result
            .err()
            .unwrap()
            .downcast_ref::<FakeDumpFailure>()
            .is_some()
    );
    assert_eq!(target.projection(), None);
}
