//! Real-field incidence at the existing iso approximation callback boundary.
use super::approx_f2var::{IsoApproxOutput, IsoApproxRequest, mma2fnc_mut_};
use super::real_slots::{AdvApp2VarIsoRealFrame, AdvApp2VarRealField};
use super::scalar_provider::{AdvApp2VarIntegerField, argument_value, set_argument_value};
use super::{AdvApp2VarEvaluation, AdvApp2VarEvaluationSlots, AdvApp2VarEvaluatorFunc2Var};
use martensite_tkernel::standard::Failure;
use std::sync::Mutex;
#[path = "real_provider_bridge.rs"]
mod bridge;
use bridge::{IsoRealAdapter, RealFrameState};

/// One callback with owned caller-field incidence and disjoint local bounds.
/// `call.set_real(AdvApp2VarRealField::ConstantParameter, .75)` is immediately
/// visible through every caller field sharing the slot.
pub struct AdvApp2VarRealEvaluation<'view, 'call> {
    frame: &'view mut AdvApp2VarIsoRealFrame,
    call: &'view mut AdvApp2VarEvaluationSlots<'call>,
    callers: RealCallerIntegers<'view>,
}
#[derive(Default)]
struct RealCallerIntegers<'a> {
    subspace_count: Option<&'a mut i32>,
    orders: Option<&'a mut [i32; 2]>,
    limits: Option<&'a mut [i32; 2]>,
    sampling: Option<&'a mut [i32; 2]>,
}
impl AdvApp2VarRealEvaluation<'_, '_> {
    /// Read an actual caller-array element through its owned identity;
    /// `call.caller_array_value(super::AdvApp2VarCallerRealArray::SubspaceTolerances,0)?`.
    pub fn caller_array_value(
        &self,
        array: super::AdvApp2VarCallerRealArray,
        index: usize,
    ) -> Result<f64, Failure> {
        self.frame.caller_array_value(array, index)
    }
    /// Write a caller-array element with immediate scalar-alias observation;
    /// `call.set_caller_array_value(super::AdvApp2VarCallerRealArray::SubspaceTolerances,0,1e-6)?`.
    pub fn set_caller_array_value(
        &mut self,
        array: super::AdvApp2VarCallerRealArray,
        index: usize,
        value: f64,
    ) -> Result<(), Failure> {
        self.frame.set_caller_array_value(array, index, value)
    }
    /// Read the current distinct caller input array; `call.caller_array(super::AdvApp2VarCallerRealArray::IsoRoots)?`.
    pub fn caller_array(&self, array: super::AdvApp2VarCallerRealArray) -> Result<&[f64], Failure> {
        self.frame.caller_array(array)
    }
    /// Change owned roots or requested limits; `call.caller_array_mut(super::AdvApp2VarCallerRealArray::SubspaceTolerances)?[0]=1e-6`.
    pub fn caller_array_mut(
        &mut self,
        array: super::AdvApp2VarCallerRealArray,
    ) -> Result<&mut [f64], Failure> {
        self.frame.caller_array_mut(array)
    }
    pub(super) fn evaluate_integer_frame(
        &mut self,
        integers: &mut super::integer_frame::IntegerFrame,
        evaluate: impl FnOnce(
            &mut super::AdvApp2VarScalarEvaluation<'_>,
            Option<&mut i32>,
            Option<usize>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&[usize]>,
            &mut AdvApp2VarIsoRealFrame,
        ) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        integers.evaluate_with_callers(
            self.call,
            self.callers.subspace_count.as_deref_mut(),
            self.callers.orders.as_deref_mut(),
            self.callers.limits.as_deref_mut(),
            self.callers.sampling.as_deref_mut(),
            |call,
             subspaces,
             slot,
             orders,
             order_slots,
             limits,
             limit_slots,
             sampling,
             sampling_slots,
             dimension_slots| {
                evaluate(
                    call,
                    subspaces,
                    slot,
                    orders,
                    order_slots,
                    limits,
                    limit_slots,
                    sampling,
                    sampling_slots,
                    dimension_slots,
                    self.frame,
                )
            },
        )
    }
    /// Read through a caller field; `call.real(AdvApp2VarRealField::UStart)`.
    pub fn real(&self, field: AdvApp2VarRealField) -> f64 {
        self.frame.real(field)
    }
    /// Write a caller field's shared slot; `call.set_real(AdvApp2VarRealField::ConstantParameter, .75)`.
    pub fn set_real(&mut self, field: AdvApp2VarRealField, value: f64) {
        self.frame.set_real(field, value);
    }
    /// Read an integer argument while retaining its actual identity; `call.integer(AdvApp2VarIntegerField::Dimension)`.
    pub fn integer(&self, field: AdvApp2VarIntegerField) -> i32 {
        argument_value(self.call, field)
    }
    /// Write an integer argument; `call.set_integer(AdvApp2VarIntegerField::UOrder, 1)`.
    pub fn set_integer(&mut self, field: AdvApp2VarIntegerField, value: i32) {
        set_argument_value(self.call, field, value);
    }
    /// Access local U bounds copied before evaluation; `call.u_bounds()[0] = -.5`.
    pub fn u_bounds(&mut self) -> &mut [f64; 2] {
        self.call.u_start_end
    }
    /// Access local V bounds; `call.v_bounds()[1] = .75`.
    pub fn v_bounds(&mut self) -> &mut [f64; 2] {
        self.call.v_start_end
    }
    /// Access generated sample parameters; `let parameters = call.parameters()`.
    pub fn parameters(&mut self) -> &mut [f64] {
        self.call.parameters
    }
}

/// Callback accepting live caller real slots. Matching closures implement it;
/// `let f = |call: &mut AdvApp2VarRealEvaluation<'_, '_>, result: &mut [f64], error: &mut i32| { result.fill(call.real(AdvApp2VarRealField::ConstantParameter)); *error = 0; };`.
pub trait AdvApp2VarRealEvaluator {
    /// Supply an operation-owned live caller dimension array when configured;
    /// `let dimensions = f.caller_subspace_dimensions(&[2]);` borrows the default.
    fn caller_subspace_dimensions<'a>(
        &'a self,
        fallback: &'a [i32],
    ) -> std::borrow::Cow<'a, [i32]> {
        std::borrow::Cow::Borrowed(fallback)
    }
    /// Evaluate through caller and local field identities; `f.evaluate_reals(&mut call, result, error)`.
    fn evaluate_reals(
        &self,
        call: &mut AdvApp2VarRealEvaluation<'_, '_>,
        result: &mut [f64],
        error: &mut i32,
    );
}
impl<F> AdvApp2VarRealEvaluator for F
where
    F: Fn(&mut AdvApp2VarRealEvaluation<'_, '_>, &mut [f64], &mut i32),
{
    fn evaluate_reals(
        &self,
        call: &mut AdvApp2VarRealEvaluation<'_, '_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self(call, result, error);
    }
}
impl AdvApp2VarIsoRealFrame {
    /// Approximate with caller real-slot identity, retaining writes after failure.
    /// retained local piece copies remain disjoint from caller bounds. Roots,
    /// tolerances and output/error buffers are outside this field domain.
    /// `frame.approximate_iso(&mut request, &f, &mut output, &mut error)?`.
    pub fn approximate_iso(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarRealEvaluator,
        output: &mut IsoApproxOutput<'_>,
        error: &mut i32,
    ) -> Result<i32, Failure> {
        self.seed_request(request)?;
        let state = Mutex::new(RealFrameState {
            frame: Some(self),
            failure: None,
        });
        let adapter = IsoRealAdapter {
            frame: &state,
            evaluator,
        };
        let result = mma2fnc_mut_(request, &adapter, output, error);
        let frame = state.into_inner().map_err(|_| poisoned_real_frame())?;
        frame.frame.expect("AdvApp2VarIsoRealFrame: frame=None; expected restored real storage after evaluation").write_request(request);
        frame.failure.map_or(Ok(result), Err)
    }
}
fn poisoned_real_frame() -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        "AdvApp2VarIsoRealFrame: poisoned=true; expected unpoisoned operation-local real storage",
    )
}
