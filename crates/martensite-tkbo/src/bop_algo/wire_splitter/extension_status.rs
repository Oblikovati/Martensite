//! Classify actual receiver state without acquiring an unchecked replacement receiver.
use super::{WireOperationContext, WireSplitter, WireSplitterHooks};
use crate::bop_algo::{
    ProgressCompletion, ProgressFailureClass, ProgressOperation, ProgressStage,
    builder_area::progress::classify_area_progress,
};
use martensite_tkernel::standard::Failure;

impl WireOperationContext {
    pub(super) fn classify_hooks<H: WireSplitterHooks + ?Sized>(
        &self,
        hooks: &mut H,
        result: Result<(), Failure>,
    ) -> ProgressCompletion {
        let result = self.guard_result(result);
        match self.checked_hooks(hooks) {
            Ok(receiver) => self.classify_direct(receiver, result),
            Err(cause) => wire_binding_failure(result.err().unwrap_or(cause)),
        }
    }

    pub(super) fn classify_direct(
        &self,
        receiver: &WireSplitter,
        result: Result<(), Failure>,
    ) -> ProgressCompletion {
        let result = self.guard_result(result);
        if let Err(cause) = self.check_direct(receiver) {
            return wire_binding_failure(result.err().unwrap_or(cause));
        }
        classify_area_progress(
            ProgressOperation::WireSplitter,
            receiver.options(),
            result,
            self.observation(),
        )
    }
}

pub(super) fn wire_binding_failure(cause: Failure) -> ProgressCompletion {
    ProgressCompletion::Failed {
        operation: ProgressOperation::WireSplitter,
        stage: ProgressStage::Completion,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause),
        context: None,
    }
}
