//! Borrowed-slice integer callback adaptor; numerical stages stay in the canonical pipeline.
use super::*;
pub(super) type CombinedState<'frame> =
    crate::adv_app2var::operation_frame_lease::OperationFrameState<'frame, IntegerFrame>;
pub(super) struct CombinedBridge<'state, 'frame, 'eval, E> {
    pub(super) state: &'state Mutex<CombinedState<'frame>>,
    pub(super) evaluator: &'eval E,
}
impl<E: AdvApp2VarCombinedEvaluator> AdvApp2VarRealEvaluator for CombinedBridge<'_, '_, '_, E> {
    fn caller_subspace_dimensions<'a>(
        &'a self,
        fallback: &'a [i32],
    ) -> std::borrow::Cow<'a, [i32]> {
        self.with_resident_frame(|integers| {
            integers.caller_dimensions().map_or(
                std::borrow::Cow::Borrowed(fallback),
                std::borrow::Cow::Owned,
            )
        })
    }

    fn evaluate_reals(
        &self,
        call: &mut AdvApp2VarRealEvaluation<'_, '_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        let mut lease = match crate::adv_app2var::operation_frame_lease::OperationFrameLease::take(
            self.state,
            poisoned_combined_frame,
            active_integer_callback,
        ) {
            Ok(lease) => lease,
            Err(failure) => {
                self.record_failure(failure);
                *error = 1;
                return;
            }
        };
        let outcome = call.evaluate_integer_frame(lease.frame_mut("AdvApp2VarIsoOperationFrame: integers=None; expected leased callback storage"), |integers, subspace_count, subspace_slot, caller_orders, caller_order_slots, caller_limits, caller_limit_slots, caller_sampling, caller_sampling_slots, caller_dimension_slots, reals| {
            let subspace_count = subspace_count.ok_or_else(|| Failure::with_message("Standard_DomainError", "AdvApp2VarIsoOperationFrame: subspace context=None; expected caller subspace count during iso evaluation"))?;
            self.evaluator.evaluate_combined(&mut AdvApp2VarCombinedEvaluation {integers, reals, subspace_count, subspace_slot, caller_orders, caller_order_slots, caller_limits, caller_limit_slots, caller_sampling, caller_sampling_slots, caller_dimension_slots,caller_output_slots:None,coefficient_origin:None,cut_origin:None,folded_origins:None,error_origins:None,endpoint_origins:None}, result, error);
            Ok(())
        });
        if let Err(failure) = outcome {
            self.record_failure(failure);
            *error = 1;
        }
    }
}

impl<E> CombinedBridge<'_, '_, '_, E> {
    fn record_failure(&self, failure: Failure) {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .failure = Some(failure);
    }
    fn with_resident_frame<Value>(&self, operation: impl FnOnce(&IntegerFrame) -> Value) -> Value {
        let mut state = self.state.lock().expect("AdvApp2VarIsoOperationFrame: poisoned=true; expected running operation with unpoisoned integer storage");
        let Some(integers) = state.frame.as_deref() else {
            state.failure = Some(active_integer_callback());
            drop(state);
            panic!(
                "AdvApp2VarIsoOperationFrame: integers=None; expected resident stage storage outside callback"
            );
        };
        operation(integers)
    }
}
fn active_integer_callback() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: integer callback active=true; expected resident storage outside evaluator",
    )
}

#[cfg(test)]
#[path = "combined_slice_bridge_tests.rs"]
mod tests;
