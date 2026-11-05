//! Named actual receivers and callback observations, with no simulated geometry backend.
use martensite_tkbo::bop_algo::{
    AlertNullInputShapes, AlertTooFewArguments, ProgressCompletion, ProgressFailureClass,
    ProgressOperation, ProgressStage, WireEdgeSet, WireOperationContext, WireSplitter,
    WireSplitterHooks,
};
use martensite_tkernel::{
    message::{AlertHandle, ProgressIndicator, ProgressIndicatorHooks, ProgressScope},
    standard::{Failure, raise_native_failure},
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

pub(crate) enum WireCheckPolicy {
    Delegate,
    SupplyEmpty,
    ReportError,
    ReturnFailure,
}

pub(crate) struct AuthoredWireHooks {
    pub(crate) receiver: WireSplitter,
    pub(crate) checks: usize,
    pub(crate) policy: WireCheckPolicy,
}

impl AuthoredWireHooks {
    pub(crate) fn new(receiver: WireSplitter, policy: WireCheckPolicy) -> Self {
        Self {
            receiver,
            policy,
            checks: 0,
        }
    }
}

impl WireSplitterHooks for AuthoredWireHooks {
    fn wire_splitter(&mut self) -> &mut WireSplitter {
        // Tests must observe this stored receiver rather than a detached options copy.
        &mut self.receiver
    }

    fn check_data(&mut self, operation: &WireOperationContext) -> Result<(), Failure> {
        self.checks += 1;
        match self.policy {
            WireCheckPolicy::Delegate => operation.check_data_default(self),
            WireCheckPolicy::SupplyEmpty => {
                self.receiver.set_wes(WireEdgeSet::new());
                operation.check_data_default(self)
            }
            WireCheckPolicy::ReportError => {
                add_error(&self.receiver, false);
                Ok(())
            }
            WireCheckPolicy::ReturnFailure => Err(authored_failure()),
        }
    }
}

pub(crate) fn authored_failure() -> Failure {
    Failure::with_type(
        "Standard_DomainError",
        "authored wire input check",
        "wire-owned-stack",
    )
}

pub(crate) fn add_error(receiver: &WireSplitter, null_input: bool) {
    let alert: AlertHandle = if null_input {
        Arc::new(AlertNullInputShapes)
    } else {
        Arc::new(AlertTooFewArguments)
    };
    receiver.options().add_error(&alert);
}

pub(crate) fn require_report_failure(outcome: ProgressCompletion) {
    assert!(matches!(
        outcome,
        ProgressCompletion::Failed {
            operation: ProgressOperation::WireSplitter,
            stage: ProgressStage::Completion,
            class: ProgressFailureClass::AlgorithmReport,
            cause: None,
            context: None,
        }
    ));
}

pub(crate) fn require_kernel_failure(outcome: ProgressCompletion, kind: &str) -> Failure {
    let ProgressCompletion::Failed {
        operation: ProgressOperation::WireSplitter,
        stage: ProgressStage::Completion,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause),
        context: None,
    } = outcome
    else {
        panic!("outcome={outcome:?}; expected an owned typed kernel failure");
    };
    assert_eq!(cause.exception_type(), kind);
    cause
}

#[derive(Default)]
pub(crate) struct WireCallbackTrace {
    pub(crate) polls: AtomicUsize,
    pub(crate) names: Mutex<Vec<String>>,
}

pub(crate) struct CountedWirePolls {
    pub(crate) trace: Arc<WireCallbackTrace>,
    pub(crate) cancel_at: usize,
    pub(crate) throw_at: usize,
}

impl ProgressIndicatorHooks for CountedWirePolls {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let name = scope
            .name()
            .map_or("<unnamed>", |value| std::str::from_utf8(value).unwrap());
        self.trace.names.lock().unwrap().push(name.to_owned());
        std::hint::black_box(position);
    }

    fn user_break(&self) -> bool {
        let ordinal = self.trace.polls.fetch_add(1, Ordering::AcqRel) + 1;
        if ordinal == self.throw_at {
            return raise_native_failure(Err(authored_failure()));
        }
        ordinal == self.cancel_at
    }
}

pub(crate) fn counted_indicator(cancel_at: usize) -> (ProgressIndicator, Arc<WireCallbackTrace>) {
    let trace = Arc::new(WireCallbackTrace::default());
    let hooks = CountedWirePolls {
        trace: Arc::clone(&trace),
        cancel_at,
        throw_at: 0,
    };
    (ProgressIndicator::new(Box::new(hooks)), trace)
}

pub(crate) fn throwing_prepared_indicator() -> ProgressIndicator {
    let hooks = CountedWirePolls {
        trace: Arc::new(WireCallbackTrace::default()),
        cancel_at: 0,
        throw_at: 3,
    };
    ProgressIndicator::new(Box::new(hooks))
}
