//! Indexed buffers and caller scalar incidence share one operation-local owner.
use super::*;
use crate::adv_app2var::{
    AdvApp2VarBufferEvaluation, AdvApp2VarEvaluation, AdvApp2VarEvaluationBuffers,
    AdvApp2VarEvaluatorFunc2Var, AdvApp2VarIsoCallerIntegers,
};

/// Combined caller controls with indexed access to the complete callback frame.
/// `call.buffers().set_result(0, 1.)` preserves overlapping parameter identity.
pub type AdvApp2VarCombinedBufferEvaluation<'view, 'call, 'buffer> =
    AdvApp2VarCombinedEvaluation<'view, 'call, &'call mut AdvApp2VarEvaluationBuffers<'buffer>>;
impl<'buffer> AdvApp2VarCombinedBufferEvaluation<'_, '_, 'buffer> {
    /// Access original parameter/result slots without overlapping slices;
    /// `call.buffers().set_parameter(0, 0.5)`.
    pub fn buffers(&mut self) -> &mut AdvApp2VarEvaluationBuffers<'buffer> {
        self.integers.parameters
    }
}

/// Evaluator that composes caller incidence with indexed full-frame access.
/// Matching closures implement it; `let f = |call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>, error: &mut i32| { call.buffers().set_result(0, 1.); *error=0; };`.
pub trait AdvApp2VarCombinedBufferEvaluator {
    /// Evaluate through actual owned slots; `f.evaluate_combined_buffers(&mut call, error)`.
    fn evaluate_combined_buffers(
        &self,
        call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>,
        error: &mut i32,
    );
}
impl<F> AdvApp2VarCombinedBufferEvaluator for F
where
    F: Fn(&mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>, &mut i32),
{
    fn evaluate_combined_buffers(
        &self,
        call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>,
        error: &mut i32,
    ) {
        self(call, error);
    }
}
impl AdvApp2VarIsoOperationFrame {
    /// Approximate using indexed workspace access and shared caller fields in
    /// the existing numerical pipeline; `frame.approximate_iso_buffers(&mut request, &f, &mut output, error)?`.
    pub fn approximate_iso_buffers(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarCombinedBufferEvaluator,
        output: &mut IsoApproxOutput<'_>,
        error: &mut i32,
    ) -> Result<i32, Failure> {
        self.require_borrowed_integer_outputs()?;
        self.reals.require_slice_curve()?;
        self.approximate_iso_stored_curve(request, evaluator, output, error)
    }
    /// Approximate with a caller curve sharing the same initialized owned real slots;
    /// `frame.approximate_iso_owned_curve(&mut request,&f,&mut output,error)?`.
    pub fn approximate_iso_owned_curve(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarCombinedBufferEvaluator,
        output: &mut crate::adv_app2var::approx_f2var::IsoApproxCallerOutput<
            '_,
            crate::adv_app2var::AdvApp2VarCallerCurveSlots,
        >,
        error: &mut i32,
    ) -> Result<i32, Failure> {
        self.require_borrowed_integer_outputs()?;
        self.reals.validate_curve_owner(&output.courbe)?;
        self.approximate_iso_stored_curve(request, evaluator, output, error)
    }
    fn approximate_iso_stored_curve<
        Curve: crate::adv_app2var::approx_f2var::curve_output::CurveCoefficientStorage,
    >(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarCombinedBufferEvaluator,
        output: &mut crate::adv_app2var::approx_f2var::IsoApproxCallerOutput<'_, Curve>,
        error: &mut i32,
    ) -> Result<i32, Failure> {
        self.integers.seed_caller_dimensions(request.ndimse)?;
        self.reals.seed_request(request)?;
        let state = Mutex::new(IndexedCombinedState {
            frame: Some(self),
            failure: None,
        });
        let bridge = IndexedCombinedBridge {
            state: &state,
            evaluator,
        };
        let result =
            crate::adv_app2var::approx_f2var::mma2fnc_curve_store(request, &bridge, output, error);
        let state = state.into_inner().map_err(|_| poisoned_combined_frame())?;
        state.frame.expect("AdvApp2VarIsoOperationFrame: frame=None; expected restored operation frame after evaluation").reals.write_request(request);
        state.failure.map_or(Ok(result), Err)
    }
}
pub(super) type IndexedCombinedState<'frame> =
    super::super::operation_frame_lease::OperationFrameState<'frame, AdvApp2VarIsoOperationFrame>;
type CallbackFrameLease<'state, 'frame> = super::super::operation_frame_lease::OperationFrameLease<
    'state,
    'frame,
    AdvApp2VarIsoOperationFrame,
>;
pub(super) struct IndexedCombinedBridge<'state, 'frame, 'eval, E> {
    pub(super) state: &'state Mutex<IndexedCombinedState<'frame>>,
    pub(super) evaluator: &'eval E,
}
impl<E: AdvApp2VarCombinedBufferEvaluator> AdvApp2VarEvaluatorFunc2Var
    for IndexedCombinedBridge<'_, '_, '_, E>
{
    fn caller_iso_domain(&self, _: [f64; 4]) -> [f64; 4] {
        self.with_resident_frame(|frame| frame.reals.caller_domain())
    }
    fn caller_iso_roots<'a>(&'a self, fallback: &'a [f64]) -> std::borrow::Cow<'a, [f64]> {
        self.with_resident_frame(|frame| {
            frame.reals.caller_array_projection(
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
            frame.reals.caller_array_projection(
                crate::adv_app2var::AdvApp2VarCallerRealArray::SubspaceTolerances,
                fallback,
            )
        })
    }
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], error: &mut i32) {
        *error = 1;
    }
    fn caller_subspace_dimensions<'a>(
        &'a self,
        fallback: &'a [i32],
    ) -> std::borrow::Cow<'a, [i32]> {
        self.with_resident_frame(|frame| {
            frame.integers.caller_dimensions().map_or(
                std::borrow::Cow::Borrowed(fallback),
                std::borrow::Cow::Owned,
            )
        })
    }

    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        callers: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        let mut lease = match self.take_callback_frame() {
            Ok(lease) => lease,
            Err(failure) => {
                self.record_failure(failure);
                *error = 1;
                return;
            }
        };
        let caller_output_slots = lease
            .frame_mut("AdvApp2VarIsoOperationFrame: missing leased output metadata")
            .integer_outputs;
        let endpoint_origins = lease
            .frame_mut("AdvApp2VarIsoOperationFrame: missing leased endpoint metadata")
            .endpoint_origins;
        let error_origins = lease
            .frame_mut("AdvApp2VarIsoOperationFrame: missing leased error metadata")
            .error_origins;
        let folded_origins = lease
            .frame_mut("AdvApp2VarIsoOperationFrame: missing leased folded metadata")
            .folded_origins;
        let cut_origin = lease
            .frame_mut("AdvApp2VarIsoOperationFrame: missing leased cut metadata")
            .cut_origin;
        let coefficient_origin = lease
            .frame_mut("AdvApp2VarIsoOperationFrame: missing leased count metadata")
            .coefficient_origin;
        let AdvApp2VarIsoOperationFrame { integers, reals,.. } = lease.frame_mut("AdvApp2VarIsoOperationFrame: leased frame=None; expected active exclusive callback frame");
        let outcome = integers.evaluate_integer_arguments(
            call, Some(callers.subspace_count), Some(callers.orders),
            callers.limits.as_deref_mut(), callers.sampling.as_deref_mut(),
            |call, values, bindings, subspace_count, subspace_slot, caller_orders, caller_order_slots, caller_limits, caller_limit_slots, caller_sampling, caller_sampling_slots, caller_dimension_slots| {
                let subspace_count = subspace_count.expect("AdvApp2VarIsoOperationFrame: subspace context=None; expected supplied iso context");
                let mut integers = AdvApp2VarScalarEvaluation {
                    values, bindings, u_start_end: call.u_start_end, v_start_end: call.v_start_end,
                    const_param: call.const_param, parameters: &mut call.buffers,
                };
                self.evaluator.evaluate_combined_buffers(&mut AdvApp2VarCombinedEvaluation {
                    integers: &mut integers, reals, subspace_count, subspace_slot,
                    caller_orders, caller_order_slots, caller_limits, caller_limit_slots,
                    caller_sampling, caller_sampling_slots, caller_dimension_slots,caller_output_slots,coefficient_origin,cut_origin,folded_origins,error_origins,endpoint_origins,
                }, error);
                Ok(())
            },
        );
        *call.const_param = reals.real(AdvApp2VarRealField::ConstantParameter);
        if let Err(failure) = outcome {
            self.record_failure(failure);
            *error = 1;
        }
    }
}

impl<'state, 'frame, E> IndexedCombinedBridge<'state, 'frame, '_, E> {
    fn take_callback_frame(&self) -> Result<CallbackFrameLease<'state, 'frame>, Failure> {
        CallbackFrameLease::take(self.state, poisoned_combined_frame, active_callback_failure)
    }

    fn record_failure(&self, failure: Failure) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.failure = Some(failure);
    }
    fn with_resident_frame<ResultValue>(
        &self,
        operation: impl FnOnce(&AdvApp2VarIsoOperationFrame) -> ResultValue,
    ) -> ResultValue {
        let mut state = self
            .state
            .lock()
            .expect("AdvApp2VarIsoOperationFrame: poisoned=true; expected unpoisoned stage frame");
        let Some(frame) = state.frame.as_deref() else {
            state.failure = Some(active_callback_failure());
            drop(state);
            panic!(
                "AdvApp2VarIsoOperationFrame: callback active=true; expected stage access outside evaluator callback"
            );
        };
        operation(frame)
    }
}
fn active_callback_failure() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: callback active=true; expected exclusive non-reentrant evaluator dispatch",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adv_app2var::{AdvApp2VarDerivativeOrders, AdvApp2VarRealBindings};
    fn frame() -> AdvApp2VarIsoOperationFrame {
        let mut frame = AdvApp2VarIsoOperationFrame::new(
            vec![0; 5],
            AdvApp2VarIntegerBindings {
                dimension: 0,
                favorite_iso: 1,
                parameter_count: 2,
                u_order: 3,
                v_order: 4,
            },
            vec![0.; 69],
            AdvApp2VarRealBindings {
                domain: [0, 1, 2, 3],
                constant_parameter: 4,
            },
        )
        .unwrap();
        let _output = frame.caller_curve_slots(5, 64).unwrap();
        frame
    }
    fn dispatch(evaluator: &impl AdvApp2VarEvaluatorFunc2Var) -> i32 {
        let (mut dim, mut iso, mut count, mut u, mut v, mut spaces) = (1, 1, 1, 0, 0, 1);
        let (mut ub, mut vb, mut constant, mut parameters, mut results, mut orders) =
            ([0., 1.], [0., 1.], 0.5, [0.5], [0.], [-1, 0]);
        let mut call = AdvApp2VarBufferEvaluation {
            dimension: &mut dim,
            u_start_end: &mut ub,
            v_start_end: &mut vb,
            favor_iso: &mut iso,
            const_param: &mut constant,
            nb_params: &mut count,
            orders: AdvApp2VarDerivativeOrders::Independent {
                u: &mut u,
                v: &mut v,
            },
            buffers: AdvApp2VarEvaluationBuffers::separate(&mut parameters, &mut results, 1),
        };
        let mut error = 0;
        evaluator.evaluate_iso_buffers(
            &mut call,
            &mut AdvApp2VarIsoCallerIntegers::new(&mut spaces, &mut orders),
            &mut error,
        );
        error
    }
    #[test]
    fn callback_releases_state_guard_and_reentrant_bridge_access_fails_explicitly() {
        let mut owner = frame();
        let state = Mutex::new(IndexedCombinedState {
            frame: Some(&mut owner),
            failure: None,
        });
        let evaluator = |call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>,
                         error: &mut i32| {
            assert!(state.try_lock().is_ok());
            call.set_caller_curve_value(0, 0.25).unwrap();
            assert_eq!(call.caller_curve_value(0).unwrap(), 0.25);
            let forbidden = |_: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>,
                             _: &mut i32| {
                panic!("reentrant callback must not execute")
            };
            let nested = IndexedCombinedBridge {
                state: &state,
                evaluator: &forbidden,
            };
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                    || nested.caller_iso_roots(&[0.])
                ))
                .is_err()
            );
            assert_eq!(dispatch(&nested), 1);
            *error = 1;
        };
        assert_eq!(
            dispatch(&IndexedCombinedBridge {
                state: &state,
                evaluator: &evaluator
            }),
            1
        );
        assert!(!state.is_poisoned());
        let state = state.into_inner().unwrap();
        assert_eq!(state.frame.unwrap().caller_curve_value(0).unwrap(), 0.25);
        assert!(
            state
                .failure
                .unwrap()
                .message()
                .contains("callback active=true")
        );
    }
    #[test]
    fn evaluator_unwind_restores_frame_and_leaves_both_mutexes_unpoisoned() {
        let mut owner = frame();
        let state = Mutex::new(IndexedCombinedState {
            frame: Some(&mut owner),
            failure: None,
        });
        let evaluator = |call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>, _: &mut i32| {
            assert!(state.try_lock().is_ok());
            call.set_caller_curve_value(0, 0.75).unwrap();
            panic!("owned evaluator unwind contract");
        };
        let bridge = IndexedCombinedBridge {
            state: &state,
            evaluator: &evaluator,
        };
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dispatch(&bridge))).is_err()
        );
        assert!(!state.is_poisoned());
        let state = state.into_inner().unwrap();
        assert_eq!(state.frame.unwrap().caller_curve_value(0).unwrap(), 0.75);
        assert!(state.failure.is_none());
    }
}
