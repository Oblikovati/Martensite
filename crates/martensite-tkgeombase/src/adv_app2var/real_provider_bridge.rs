//! Borrowed-slice real callback adaptor shares the existing approximation pipeline.
use super::*;
pub(super) type RealFrameState<'frame> =
    crate::adv_app2var::operation_frame_lease::OperationFrameState<'frame, AdvApp2VarIsoRealFrame>;
pub(super) struct IsoRealAdapter<'state, 'frame, 'eval, E> {
    pub(super) frame: &'state Mutex<RealFrameState<'frame>>,
    pub(super) evaluator: &'eval E,
}
impl<E: AdvApp2VarRealEvaluator> AdvApp2VarEvaluatorFunc2Var for IsoRealAdapter<'_, '_, '_, E> {
    fn caller_iso_roots<'a>(&'a self, fallback: &'a [f64]) -> std::borrow::Cow<'a, [f64]> {
        self.with_resident_frame(|frame| {
            frame.caller_array_projection(
                crate::adv_app2var::AdvApp2VarCallerRealArray::IsoRoots,
                fallback,
            )
        })
    }
    fn caller_subspace_tolerances<'a>(
        &'a self,
        fallback: &'a [f64],
    ) -> std::borrow::Cow<'a, [f64]> {
        self.with_resident_frame(|frame| {
            frame.caller_array_projection(
                crate::adv_app2var::AdvApp2VarCallerRealArray::SubspaceTolerances,
                fallback,
            )
        })
    }

    fn caller_subspace_dimensions<'a>(
        &'a self,
        fallback: &'a [i32],
    ) -> std::borrow::Cow<'a, [i32]> {
        self.evaluator.caller_subspace_dimensions(fallback)
    }
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], error: &mut i32) {
        *error = 1;
    }
    fn evaluate_slots(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self.evaluate_with_callers(call, RealCallerIntegers::default(), result, error);
    }
    fn evaluate_iso_slots(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        subspace_count: &mut i32,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self.evaluate_with_callers(
            call,
            RealCallerIntegers {
                subspace_count: Some(subspace_count),
                ..Default::default()
            },
            result,
            error,
        );
    }
    fn evaluate_iso_callers(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        callers: &mut crate::adv_app2var::AdvApp2VarIsoCallerIntegers<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self.evaluate_with_callers(
            call,
            RealCallerIntegers {
                subspace_count: Some(callers.subspace_count),
                orders: Some(callers.orders),
                limits: callers.limits.as_deref_mut(),
                sampling: callers.sampling.as_deref_mut(),
            },
            result,
            error,
        );
    }
}
impl<E: AdvApp2VarRealEvaluator> IsoRealAdapter<'_, '_, '_, E> {
    fn evaluate_with_callers(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        callers: RealCallerIntegers<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        let mut lease = match crate::adv_app2var::operation_frame_lease::OperationFrameLease::take(
            self.frame,
            poisoned_real_frame,
            active_real_callback,
        ) {
            Ok(lease) => lease,
            Err(failure) => {
                self.record_failure(failure);
                *error = 1;
                return;
            }
        };
        let mut view = AdvApp2VarRealEvaluation {
            frame: lease
                .frame_mut("AdvApp2VarIsoRealFrame: frame=None; expected leased callback storage"),
            call,
            callers,
        };
        self.evaluator.evaluate_reals(&mut view, result, error);
        *view.call.const_param = view.frame.real(AdvApp2VarRealField::ConstantParameter);
    }
}

impl<E> IsoRealAdapter<'_, '_, '_, E> {
    fn record_failure(&self, failure: Failure) {
        self.frame
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .failure = Some(failure);
    }
    fn with_resident_frame<Value>(
        &self,
        operation: impl FnOnce(&AdvApp2VarIsoRealFrame) -> Value,
    ) -> Value {
        let mut state = self.frame.lock().expect("AdvApp2VarIsoRealFrame: poisoned=true; expected running operation with unpoisoned real storage");
        let Some(frame) = state.frame.as_deref() else {
            state.failure = Some(active_real_callback());
            drop(state);
            panic!(
                "AdvApp2VarIsoRealFrame: frame=None; expected resident stage storage outside callback"
            );
        };
        operation(frame)
    }
}
fn active_real_callback() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoRealFrame: real callback active=true; expected resident storage outside evaluator",
    )
}

#[cfg(test)]
#[path = "real_provider_bridge_tests.rs"]
mod tests;
