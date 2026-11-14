//! Cancellation latching and cleanup do not depend on a particular geometry workload.
use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
struct DropProbe(Arc<AtomicUsize>);
impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
#[test]
fn post_stage_cancellation_drops_the_completed_value_and_refuses_later_work() {
    let dropped = Arc::new(AtomicUsize::new(0));
    let mut polls = 0;
    let mut cancelled = |_| {
        polls += 1;
        polls == 2
    };
    let mut control = Control::new(&mut cancelled);
    let result = control.run(FamilyStage::Storage, Some(SurfaceRegionId(5)), |_| {
        Ok(DropProbe(dropped.clone()))
    });
    assert!(matches!(result, Err(FamilyFailure::Cancelled(_))));
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    assert_stopped(&mut control);
    assert_eq!(polls, 2);
}
#[test]
fn nested_cancellation_remains_latched_when_the_operation_ignores_it() {
    let mut polls = 0;
    let mut cancelled = |_| {
        polls += 1;
        polls == 2
    };
    let mut control = Control::new(&mut cancelled);
    let result = control.run(FamilyStage::Selection, None, |poll| {
        assert!(poll());
        assert!(poll());
        Ok(())
    });
    assert!(matches!(result, Err(FamilyFailure::Cancelled(_))));
    assert_eq!(polls, 2);
    fn send<T: Send>() {}
    send::<Control<'static>>();
}

fn assert_stopped(control: &mut Control<'_>) {
    let later: Checked<()> = control.run(FamilyStage::Completion, None, |_| {
        panic!("work after cancellation")
    });
    assert!(matches!(
        later,
        Err(FamilyFailure::Cancelled(Checkpoint {
            stage: FamilyStage::Storage,
            ..
        }))
    ));
}
