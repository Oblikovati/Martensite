//! Admission and close contracts are independent from driver availability.
use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
#[derive(Default)]
struct FakeDeviceDrain {
    calls: AtomicUsize,
}
impl FakeDeviceDrain {
    fn drain(&self, failure: Option<i32>) -> RetirementAttempt {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let outcome = match failure {
            Some(code) => Err(RenderError::device_backend(
                "wait idle",
                code,
                format!("fake idle: {code}"),
            )),
            None => Ok(()),
        };
        RetirementAttempt::from(outcome)
    }
}
#[test]
fn ordinary_failure_does_not_falsely_poison_device_admission() {
    let lifecycle = DeviceLifecycle::default();
    let error = RenderError::device_backend("allocate", -1, "allocation failed".into());
    assert!(lifecycle.observe::<()>(Err(error)).is_err());
    assert_eq!(lifecycle.state(), RenderDeviceState::Ready);
    lifecycle.admit("next frame").unwrap();
}
#[test]
fn observed_cleanup_device_loss_persistently_rejects_new_work() {
    let lifecycle = DeviceLifecycle::default();
    let primary = RenderError::device_backend("fence", -1, "fence failed".into());
    let cleanup = RenderError::device_backend("idle", -4, "device lost".into());
    assert!(
        lifecycle
            .observe::<()>(Err(primary.with_observed_failure(cleanup)))
            .is_err()
    );
    assert_eq!(lifecycle.state(), RenderDeviceState::Lost);
    lifecycle.observe(Ok(())).unwrap();
    assert!(lifecycle.admit("new allocation").is_err());
}
#[test]
fn close_rejects_during_drain_and_is_idempotent_after_success() {
    let lifecycle = DeviceLifecycle::default();
    let calls = FakeDeviceDrain::default();
    lifecycle
        .close_with(|| {
            assert_eq!(lifecycle.state(), RenderDeviceState::Closing);
            assert!(lifecycle.admit("during close").is_err());
            calls.drain(None)
        })
        .unwrap();
    lifecycle.close_with(|| calls.drain(None)).unwrap();
    assert_eq!(calls.calls.load(Ordering::SeqCst), 1);
    assert_eq!(lifecycle.state(), RenderDeviceState::Closed);
    assert!(lifecycle.admit("after close").is_err());
}
#[test]
fn unknown_retirement_is_retryable_but_never_ready_or_closed() {
    let lifecycle = DeviceLifecycle::default();
    let calls = FakeDeviceDrain::default();
    assert!(lifecycle.close_with(|| calls.drain(Some(-1))).is_err());
    assert_eq!(lifecycle.state(), RenderDeviceState::RetirementIncomplete);
    assert!(lifecycle.admit("pending frame").is_err());
    lifecycle.close_with(|| calls.drain(None)).unwrap();
    assert_eq!(calls.calls.load(Ordering::SeqCst), 2);
    assert_eq!(lifecycle.state(), RenderDeviceState::Closed);
}
#[test]
fn prior_device_loss_is_not_erased_by_successful_close_or_repeated_close() {
    let lifecycle = DeviceLifecycle::default();
    let loss = RenderError::device_backend("submit", -4, "lost at submit".into());
    let calls = FakeDeviceDrain::default();
    assert!(lifecycle.observe::<()>(Err(loss.clone())).is_err());
    assert_eq!(
        lifecycle.close_with(|| calls.drain(None)),
        Err(loss.clone())
    );
    assert_eq!(lifecycle.close_with(|| calls.drain(None)), Err(loss));
    assert_eq!(calls.calls.load(Ordering::SeqCst), 1);
    assert_eq!(lifecycle.state(), RenderDeviceState::Closed);
}
#[test]
fn named_reentrant_drain_observes_closing_and_can_record_actual_loss() {
    let lifecycle = DeviceLifecycle::default();
    let loss = RenderError::device_backend("reentrant observation", -4, "lost".into());
    let result = lifecycle.close_with(|| {
        assert_eq!(lifecycle.state(), RenderDeviceState::Closing);
        assert!(lifecycle.admit("reentrant draw").is_err());
        RetirementAttempt::from(lifecycle.observe::<()>(Err(loss.clone())))
    });
    assert!(result.is_err());
    assert_eq!(lifecycle.state(), RenderDeviceState::Closed);
}
struct FakeBlockingDrain {
    entered: std::sync::mpsc::Sender<()>,
    resume: std::sync::mpsc::Receiver<()>,
}
impl FakeBlockingDrain {
    fn drain(&self, calls: &FakeDeviceDrain) -> RetirementAttempt {
        self.entered.send(()).unwrap();
        self.resume.recv().unwrap();
        calls.drain(None)
    }
}
#[test]
fn simultaneous_close_refuses_without_a_second_drain() {
    let lifecycle = &DeviceLifecycle::default();
    let calls = &FakeDeviceDrain::default();
    std::thread::scope(|scope| {
        let (entered, observed) = std::sync::mpsc::channel();
        let (resume, released) = std::sync::mpsc::channel();
        let blocking = FakeBlockingDrain {
            entered,
            resume: released,
        };
        let worker = scope.spawn(move || lifecycle.close_with(|| blocking.drain(calls)));
        observed.recv().unwrap();
        assert!(lifecycle.close_with(|| calls.drain(None)).is_err());
        assert_eq!(calls.calls.load(Ordering::SeqCst), 0);
        resume.send(()).unwrap();
        worker.join().unwrap().unwrap();
    });
    assert_eq!(calls.calls.load(Ordering::SeqCst), 1);
}
struct FakePoisonedLifecycle;
impl FakePoisonedLifecycle {
    fn poison(lifecycle: &DeviceLifecycle) {
        let outcome = std::panic::catch_unwind(|| {
            let _record = lifecycle.record.lock().unwrap();
            panic!("named lifecycle-record failure");
        });
        assert!(outcome.is_err());
    }
}
#[test]
fn poisoned_observation_preserves_original_loss_and_still_refuses_dispatch() {
    let lifecycle = DeviceLifecycle::default();
    FakePoisonedLifecycle::poison(&lifecycle);
    let loss = RenderError::device_backend("submit", -4, "original lost cause".into());
    let error = lifecycle.observe::<()>(Err(loss.clone())).unwrap_err();
    assert_eq!(error.primary_failure(), Some(&loss));
    assert!(
        error
            .observed_failure()
            .unwrap()
            .to_string()
            .contains("poisoned device lifecycle")
    );
    assert_eq!(lifecycle.state(), RenderDeviceState::Lost);
    assert!(lifecycle.admit("later frame").is_err());
}
#[test]
fn poisoned_ready_state_becomes_incomplete_without_claiming_device_loss() {
    let lifecycle = DeviceLifecycle::default();
    FakePoisonedLifecycle::poison(&lifecycle);
    assert!(lifecycle.admit("new frame").is_err());
    assert_eq!(lifecycle.state(), RenderDeviceState::RetirementIncomplete);
}
#[test]
fn old_loss_cannot_replace_qualification_of_a_later_close_attempt() {
    let lifecycle = DeviceLifecycle::default();
    let loss = RenderError::device_backend("old submit", -4, "old device loss".into());
    assert!(lifecycle.observe::<()>(Err(loss)).is_err());
    let calls = FakeDeviceDrain::default();
    assert!(lifecycle.close_with(|| calls.drain(Some(-1))).is_err());
    assert_eq!(lifecycle.state(), RenderDeviceState::RetirementIncomplete);
    assert!(lifecycle.close_with(|| calls.drain(None)).is_err());
    assert_eq!(lifecycle.state(), RenderDeviceState::Closed);
}
