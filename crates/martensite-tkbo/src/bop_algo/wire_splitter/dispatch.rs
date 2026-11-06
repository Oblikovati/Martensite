//! One bound actual-receiver program preserves the default and Face callback boundaries.
use super::{WireOperationContext, WireSplitterHooks, observe_user_break};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Failure,
};

pub(super) fn perform_wire_with_progress<H: WireSplitterHooks + ?Sized>(
    hooks: &mut H,
    range: &ProgressRange<'_>,
) -> ProgressCompletion {
    let operation = WireOperationContext::new();
    let result = perform_wire_observed(hooks, range, &operation, operation.observation());
    operation.classify_hooks(hooks, result)
}

pub(super) fn perform_wire_observed<H: WireSplitterHooks + ?Sized>(
    hooks: &mut H,
    range: &ProgressRange<'_>,
    operation: &WireOperationContext,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    operation.bind_hooks(hooks)?.options_mut().clear();
    let scope = ProgressScope::with_literal(range, "Building wires", 1., false);
    operation.guard_result(hooks.check_data(operation))?;
    perform_checked_blocks(hooks, &scope, operation, observed)
}

fn perform_checked_blocks<H: WireSplitterHooks + ?Sized>(
    hooks: &mut H,
    scope: &ProgressScope<'_>,
    operation: &WireOperationContext,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    let receiver = operation.checked_hooks(hooks)?;
    if receiver.options.has_errors() {
        return Ok(());
    }
    receiver.prepare_blocks()?;
    operation.mark_prepared()?;
    if observe_user_break(&receiver.options, scope, observed) {
        return Ok(());
    }
    operation.consume_helper(receiver)?;
    receiver.make_wires(&scope.next(1.), observed)
}
