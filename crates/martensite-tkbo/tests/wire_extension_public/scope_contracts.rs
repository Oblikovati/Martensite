//! Actual progress callbacks distinguish named scope closure from a final position alone.
use super::{fixture, support::*};
use martensite_tkbo::bop_algo::{WireOperationContext, WireSplitter, WireSplitterHooks};
use martensite_tkernel::standard::Failure;
use std::sync::{Arc, atomic::Ordering};

struct CheckExitWireHooks {
    receiver: WireSplitter,
    trace: Arc<WireCallbackTrace>,
    gets: usize,
    report: bool,
}

impl WireSplitterHooks for CheckExitWireHooks {
    fn wire_splitter(&mut self) -> &mut WireSplitter {
        self.gets += 1;
        if self.gets == if self.report { 3 } else { 2 } {
            assert!(
                self.trace
                    .names
                    .lock()
                    .unwrap()
                    .iter()
                    .any(|name| name == "Building wires")
            );
        }
        &mut self.receiver
    }

    fn check_data(&mut self, _: &WireOperationContext) -> Result<(), Failure> {
        assert!(
            !self
                .trace
                .names
                .lock()
                .unwrap()
                .iter()
                .any(|name| name == "Building wires")
        );
        if !self.report {
            return Err(authored_failure());
        }
        add_error(&self.receiver, false);
        Ok(())
    }
}

#[test]
fn check_failure_and_report_stop_close_named_scope_before_classifying() {
    for report in [false, true] {
        let (indicator, trace) = counted_indicator(usize::MAX);
        let mut hooks = CheckExitWireHooks {
            receiver: fixture::empty_splitter(),
            trace: Arc::clone(&trace),
            gets: 0,
            report,
        };
        let outcome = hooks.perform_with_progress(&indicator.start());
        if report {
            require_report_failure(outcome);
        } else {
            require_kernel_failure(outcome, "Standard_DomainError");
        }
        assert_eq!(indicator.get_position(), 1.0);
        assert_eq!(trace.polls.load(Ordering::Acquire), 0);
        assert!(hooks.receiver.context().is_null());
    }
}
