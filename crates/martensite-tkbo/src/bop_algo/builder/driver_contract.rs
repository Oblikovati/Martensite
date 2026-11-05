//! Contracts pin actual-receiver dispatch before progress is threaded through callbacks.
use super::{BOPAlgoBuilder, BOPAlgoPaveFiller, BuilderHooks};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use std::cell::Cell;

#[derive(Default)]
struct DerivedDispatchProbe {
    builder: BOPAlgoBuilder,
    calls: usize,
}

impl BuilderHooks for DerivedDispatchProbe {
    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder {
        // Substituting another receiver's base would defeat this dispatch probe.
        &mut self.builder
    }

    fn perform_internal1(&mut self, _: &mut BOPAlgoPaveFiller) -> Result<(), Failure> {
        self.calls += 1;
        Ok(())
    }
}

#[test]
fn default_driver_keeps_actual_derived_hook() {
    let mut actual = DerivedDispatchProbe::default();
    super::perform_on(&mut actual, List::default());
    assert_eq!(actual.calls, 1);
    assert!(actual.builder.filler.is_some());
}

#[test]
fn operation_local_callbacks_run_once_before_filler_retention() {
    let mut actual = DerivedDispatchProbe::default();
    let phase = Cell::new(0);
    let outcome = super::perform_on_using(
        &mut actual,
        List::default(),
        |_| phase.set(phase.get() + 1),
        |receiver, _, ()| {
            assert_eq!(phase.get(), 1);
            assert!(receiver.builder.filler.is_none());
            receiver.calls += 1;
            phase.set(2);
            Ok(())
        },
    );
    assert!(outcome.is_ok());
    assert_eq!((phase.get(), actual.calls), (2, 1));
    assert!(actual.builder.filler.is_some());
}

#[test]
fn selected_callback_keeps_existing_failure_alert_boundary() {
    let mut actual = DerivedDispatchProbe::default();
    let mut filler = BOPAlgoPaveFiller::new();
    let outcome: Result<(), Failure> =
        super::perform_internal_using(&mut actual, &mut filler, |_, _| {
            Err(Failure::with_type(
                "Standard_Failure",
                "owned callback failure",
                "",
            ))
        });
    assert_eq!(outcome.unwrap_err().exception_type(), "Standard_Failure");
    assert!(actual.builder.options().has_errors());
    assert_eq!(actual.calls, 0);
}

#[test]
fn initializer_status_reaches_actual_receiver_without_side_channel() {
    let mut actual = DerivedDispatchProbe::default();
    let outcome = super::perform_on_using(
        &mut actual,
        List::default(),
        |_| 7_u32,
        |receiver, _, prepared| {
            receiver.calls += 1;
            Ok(prepared * 2)
        },
    );
    assert_eq!(outcome.unwrap(), 14);
    assert_eq!(actual.calls, 1);
}
