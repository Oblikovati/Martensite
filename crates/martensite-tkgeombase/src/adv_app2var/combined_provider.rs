//! Combined owned integer and caller-real identities for one iso operation.
#[path = "combined_slice_bridge.rs"]
mod slice_bridge;
use slice_bridge::{CombinedBridge, CombinedState};
#[path = "combined_buffers.rs"]
mod buffers;
#[path = "iso_integer_outputs.rs"]
mod outputs;
use super::approx_f2var::{IsoApproxOutput, IsoApproxRequest};
use super::integer_frame::IntegerFrame;
use super::{
    AdvApp2VarIntegerBindings, AdvApp2VarIntegerField, AdvApp2VarIsoRealFrame,
    AdvApp2VarRealBindings, AdvApp2VarRealEvaluation, AdvApp2VarRealEvaluator, AdvApp2VarRealField,
    AdvApp2VarScalarEvaluation,
};
pub use buffers::{AdvApp2VarCombinedBufferEvaluation, AdvApp2VarCombinedBufferEvaluator};
use martensite_tkernel::standard::Failure;
pub use outputs::{AdvApp2VarIsoIntegerOutput, AdvApp2VarOwnedIsoEvaluator};
use std::sync::Mutex;

/// One callback observing both integer and real incidence. Local piece bounds
/// remain distinct from caller fields; `call.set_integer(AdvApp2VarIntegerField::Dimension, 1)`.
pub struct AdvApp2VarCombinedEvaluation<'view, 'call, Parameters = &'call mut [f64]> {
    integers: &'view mut AdvApp2VarScalarEvaluation<'call, Parameters>,
    reals: &'view mut AdvApp2VarIsoRealFrame,
    subspace_count: &'view mut i32,
    subspace_slot: Option<usize>,
    caller_orders: Option<&'view mut [i32; 2]>,
    caller_order_slots: Option<[usize; 2]>,
    caller_limits: Option<&'view mut [i32; 2]>,
    caller_limit_slots: Option<[usize; 2]>,
    caller_sampling: Option<&'view mut [i32; 2]>,
    caller_sampling_slots: Option<[usize; 2]>,
    caller_dimension_slots: Option<&'view [usize]>,
    caller_output_slots: Option<[usize; 2]>,
    coefficient_origin: Option<usize>,
    cut_origin: Option<usize>,
    folded_origins: Option<[usize; 2]>,
    error_origins: Option<[usize; 2]>,
    endpoint_origins: Option<[usize; 2]>,
}
impl<Parameters> AdvApp2VarCombinedEvaluation<'_, '_, Parameters> {
    /// Read an initialized signed curve offset, including its original owner prefix;
    /// `call.caller_curve_value_signed(-3)?`.
    pub fn caller_curve_value_signed(&self, index: i32) -> Result<f64, Failure> {
        self.reals.caller_curve_value_signed(index)
    }
    /// Write the actual initialized signed curve slot without borrowing its owner;
    /// `call.set_caller_curve_value_signed(-3,0.5)?`.
    pub fn set_caller_curve_value_signed(&mut self, index: i32, value: f64) -> Result<(), Failure> {
        self.reals.set_caller_curve_value_signed(index, value)
    }

    /// Read the actual owned caller-curve output during evaluation;
    /// `call.caller_curve_value(0)?` observes preceding root writes to shared slots.
    pub fn caller_curve_value(&self, index: usize) -> Result<f64, Failure> {
        self.reals.caller_curve_value(index)
    }
    /// Write one caller-curve output with immediate mapped-input visibility;
    /// `call.set_caller_curve_value(0,0.5)?`.
    pub fn set_caller_curve_value(&mut self, index: usize, value: f64) -> Result<(), Failure> {
        self.reals.set_caller_curve_value(index, value)
    }
    /// Read an actual caller-array element through its owned identity;
    /// `call.caller_array_value(super::AdvApp2VarCallerRealArray::SubspaceTolerances,0)?`.
    pub fn caller_array_value(
        &self,
        array: super::AdvApp2VarCallerRealArray,
        index: usize,
    ) -> Result<f64, Failure> {
        self.reals.caller_array_value(array, index)
    }
    /// Write a caller-array element with immediate scalar-alias observation;
    /// `call.set_caller_array_value(super::AdvApp2VarCallerRealArray::SubspaceTolerances,0,1e-6)?`.
    pub fn set_caller_array_value(
        &mut self,
        array: super::AdvApp2VarCallerRealArray,
        index: usize,
        value: f64,
    ) -> Result<(), Failure> {
        self.reals.set_caller_array_value(array, index, value)
    }
    /// Read the current distinct caller input array; `call.caller_array(super::AdvApp2VarCallerRealArray::IsoRoots)?`.
    pub fn caller_array(&self, array: super::AdvApp2VarCallerRealArray) -> Result<&[f64], Failure> {
        self.reals.caller_array(array)
    }
    /// Change owned roots or requested limits; `call.caller_array_mut(super::AdvApp2VarCallerRealArray::SubspaceTolerances)?[0]=1e-6`.
    pub fn caller_array_mut(
        &mut self,
        array: super::AdvApp2VarCallerRealArray,
    ) -> Result<&mut [f64], Failure> {
        self.reals.caller_array_mut(array)
    }
    /// Read one actual caller dimension, including writes through a shared scalar;
    /// `assert_eq!(call.caller_dimension(0)?, 2)` after a shared dimension write.
    pub fn caller_dimension(&self, index: usize) -> Result<i32, Failure> {
        let slot = self.caller_dimension_slot(index)?;
        Ok(self.integers.integer_slot(slot))
    }
    /// Write one actual caller dimension through its original owned integer slot;
    /// `call.set_caller_dimension(0, 2)?` is immediately visible through aliases.
    pub fn set_caller_dimension(&mut self, index: usize, value: i32) -> Result<(), Failure> {
        let slot = self.caller_dimension_slot(index)?;
        self.integers.set_integer_slot(slot, value);
        Ok(())
    }
    fn caller_dimension_slot(&self, index: usize) -> Result<usize, Failure> {
        self.caller_dimension_slots
            .and_then(|slots| slots.get(index).copied())
            .ok_or_else(|| super::integer_frame::missing_caller_dimension(index))
    }
    /// Read the caller count, including writes through an incident callback field;
    /// `assert_eq!(call.subspace_count(), 1)` after a shared dimension write.
    pub fn subspace_count(&self) -> i32 {
        self.subspace_slot.map_or(*self.subspace_count, |slot| {
            self.integers.integer_slot(slot)
        })
    }
    /// Write the caller count through its owned slot or distinct caller local;
    /// `call.set_subspace_count(1)` preserves immediate shared-field observation.
    pub fn set_subspace_count(&mut self, value: i32) {
        if let Some(slot) = self.subspace_slot {
            self.integers.set_integer_slot(slot, value);
            return;
        }
        *self.subspace_count = value;
    }
    /// Read an actual caller order, separately from local U/V orders;
    /// `call.caller_order(super::AdvApp2VarCallerOrderField::ContinuityOrder)?`.
    pub fn caller_order(&self, field: super::AdvApp2VarCallerOrderField) -> Result<i32, Failure> {
        if let Some(slots) = self.caller_order_slots {
            return Ok(self.integers.integer_slot(slots[field.index()]));
        }
        self.caller_orders
            .as_deref()
            .map(|orders| orders[field.index()])
            .ok_or_else(super::caller_orders::missing_caller_order_context)
    }
    /// Write a caller order through its actual shared slot;
    /// `call.set_caller_order(super::AdvApp2VarCallerOrderField::ContinuityOrder, 2)?`.
    pub fn set_caller_order(
        &mut self,
        field: super::AdvApp2VarCallerOrderField,
        value: i32,
    ) -> Result<(), Failure> {
        if let Some(slots) = self.caller_order_slots {
            self.integers.set_integer_slot(slots[field.index()], value);
            return Ok(());
        }
        self.caller_orders
            .as_deref_mut()
            .ok_or_else(super::caller_orders::missing_caller_order_context)?[field.index()] = value;
        Ok(())
    }
    /// Read a live caller allocation limit; `call.caller_limit(super::AdvApp2VarCallerLimitField::MaximumPieces)?`.
    pub fn caller_limit(&self, field: super::AdvApp2VarCallerLimitField) -> Result<i32, Failure> {
        if let Some(slots) = self.caller_limit_slots {
            return Ok(self.integers.integer_slot(slots[field.index()]));
        }
        self.caller_limits
            .as_deref()
            .map(|limits| limits[field.index()])
            .ok_or_else(super::caller_limits::missing_caller_limit_context)
    }
    /// Write through the actual owned limit slot; `call.set_caller_limit(super::AdvApp2VarCallerLimitField::MaximumPieces, 1)?`.
    pub fn set_caller_limit(
        &mut self,
        field: super::AdvApp2VarCallerLimitField,
        value: i32,
    ) -> Result<(), Failure> {
        if let Some(slots) = self.caller_limit_slots {
            self.integers.set_integer_slot(slots[field.index()], value);
            return Ok(());
        }
        self.caller_limits
            .as_deref_mut()
            .ok_or_else(super::caller_limits::missing_caller_limit_context)?[field.index()] = value;
        Ok(())
    }
    /// Read a live caller sampling control; `call.caller_sampling(super::AdvApp2VarCallerSamplingField::RootCount)?`.
    pub fn caller_sampling(
        &self,
        field: super::AdvApp2VarCallerSamplingField,
    ) -> Result<i32, Failure> {
        if let Some(slots) = self.caller_sampling_slots {
            return Ok(self.integers.integer_slot(slots[field.index()]));
        }
        self.caller_sampling
            .as_deref()
            .map(|sampling| sampling[field.index()])
            .ok_or_else(super::caller_sampling::missing_caller_sampling_context)
    }
    /// Write through the actual owned sampling slot; `call.set_caller_sampling(super::AdvApp2VarCallerSamplingField::RootCount, 1)?`.
    pub fn set_caller_sampling(
        &mut self,
        field: super::AdvApp2VarCallerSamplingField,
        value: i32,
    ) -> Result<(), Failure> {
        if let Some(slots) = self.caller_sampling_slots {
            self.integers.set_integer_slot(slots[field.index()], value);
            return Ok(());
        }
        self.caller_sampling
            .as_deref_mut()
            .ok_or_else(super::caller_sampling::missing_caller_sampling_context)?[field.index()] =
            value;
        Ok(())
    }
    /// Read a shared integer slot; `call.integer(AdvApp2VarIntegerField::FavoriteIso)`.
    pub fn integer(&self, field: AdvApp2VarIntegerField) -> i32 {
        self.integers.integer(field)
    }
    /// Write a shared integer slot; `call.set_integer(AdvApp2VarIntegerField::Dimension, 1)`.
    pub fn set_integer(&mut self, field: AdvApp2VarIntegerField, value: i32) {
        self.integers.set_integer(field, value);
    }
    /// Read a caller real slot; `call.real(AdvApp2VarRealField::UStart)`.
    pub fn real(&self, field: AdvApp2VarRealField) -> f64 {
        self.reals.real(field)
    }
    /// Write a caller real slot; `call.set_real(AdvApp2VarRealField::ConstantParameter, .75)`.
    pub fn set_real(&mut self, field: AdvApp2VarRealField, value: f64) {
        self.reals.set_real(field, value);
    }
    /// Access the disjoint local U copy; `call.u_bounds()[0] = -.5`.
    pub fn u_bounds(&mut self) -> &mut [f64; 2] {
        self.integers.u_start_end
    }
    /// Access the disjoint local V copy; `call.v_bounds()[1] = .75`.
    pub fn v_bounds(&mut self) -> &mut [f64; 2] {
        self.integers.v_start_end
    }
}
impl AdvApp2VarCombinedEvaluation<'_, '_> {
    /// Access generated sample parameters; `let parameters = call.parameters()`.
    pub fn parameters(&mut self) -> &mut [f64] {
        self.integers.parameters
    }
}

/// Evaluator for a combined frame. Matching closures implement it;
/// `let f = |call: &mut AdvApp2VarCombinedEvaluation<'_, '_>, result: &mut [f64], error: &mut i32| { result.fill(call.real(AdvApp2VarRealField::ConstantParameter)); *error=0; };`.
pub trait AdvApp2VarCombinedEvaluator {
    /// Evaluate with both live slot domains; `f.evaluate_combined(&mut call, result, error)`.
    fn evaluate_combined(
        &self,
        call: &mut AdvApp2VarCombinedEvaluation<'_, '_>,
        result: &mut [f64],
        error: &mut i32,
    );
}
impl<F> AdvApp2VarCombinedEvaluator for F
where
    F: Fn(&mut AdvApp2VarCombinedEvaluation<'_, '_>, &mut [f64], &mut i32),
{
    fn evaluate_combined(
        &self,
        call: &mut AdvApp2VarCombinedEvaluation<'_, '_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self(call, result, error);
    }
}

/// Operation-local typed storage for integer arguments and caller real fields.
/// Existing derivative and domain-region identities are validated rather than
/// inferred from values; `AdvApp2VarIsoOperationFrame::new(integers, integer_bindings, reals, real_bindings)?`.
pub struct AdvApp2VarIsoOperationFrame {
    integers: IntegerFrame,
    reals: AdvApp2VarIsoRealFrame,
    integer_outputs: Option<[usize; 2]>,
    coefficient_origin: Option<usize>,
    cut_origin: Option<usize>,
    folded_origins: Option<[usize; 2]>,
    error_origins: Option<[usize; 2]>,
    endpoint_origins: Option<[usize; 2]>,
}
impl AdvApp2VarIsoOperationFrame {
    /// Own both slot domains; `AdvApp2VarIsoOperationFrame::new(vec![0;4], ints, vec![0.;4], reals)?`.
    pub fn new(
        integer_values: Vec<i32>,
        integer_bindings: AdvApp2VarIntegerBindings,
        real_values: Vec<f64>,
        real_bindings: AdvApp2VarRealBindings,
    ) -> Result<Self, Failure> {
        Ok(Self {
            integers: IntegerFrame::new(integer_values, integer_bindings)?,
            reals: AdvApp2VarIsoRealFrame::new(real_values, real_bindings)?,
            integer_outputs: None,
            coefficient_origin: None,
            cut_origin: None,
            folded_origins: None,
            error_origins: None,
            endpoint_origins: None,
        })
    }
    /// Configure an owned caller curve region in the same allocation as real slots;
    /// `let curve=frame.caller_curve_slots(5,64)?` borrows no slice across callbacks.
    pub fn caller_curve_slots(
        &mut self,
        start: usize,
        length: usize,
    ) -> Result<super::AdvApp2VarCallerCurveSlots, Failure> {
        self.reals.configure_curve_region(start, length)
    }
    /// Explicitly permit caller scalar fields to share initialized curve slots;
    /// `let curve=frame.caller_scalar_curve_slots(4,64)?` preserves indexed identity.
    /// Initial shared input bits must agree before the operation clears output.
    pub fn caller_scalar_curve_slots(
        &mut self,
        start: usize,
        length: usize,
    ) -> Result<super::AdvApp2VarCallerCurveSlots, Failure> {
        self.reals.configure_scalar_curve_region(start, length)
    }
    /// Inspect one owned output coefficient after success or failure;
    /// `frame.caller_curve_value(0)?`.
    pub fn caller_curve_value(&self, index: usize) -> Result<f64, Failure> {
        self.reals.caller_curve_value(index)
    }
    /// Inspect initialized padding before the original curve origin or its declared suffix;
    /// `frame.caller_curve_value_signed(-3)?`.
    pub fn caller_curve_value_signed(&self, index: i32) -> Result<f64, Failure> {
        self.reals.caller_curve_value_signed(index)
    }
    /// Bind actual caller-array elements to owned scalar slots;
    /// `let frame=frame.with_caller_array_slots(super::AdvApp2VarCallerRealArray::SubspaceTolerances,vec![4])?`.
    pub fn with_caller_array_slots(
        mut self,
        array: super::AdvApp2VarCallerRealArray,
        slots: Vec<usize>,
    ) -> Result<Self, Failure> {
        self.reals = self.reals.with_caller_array_slots(array, slots)?;
        Ok(self)
    }
    /// Inspect an indexed owned caller element after the operation;
    /// `frame.caller_array_value(super::AdvApp2VarCallerRealArray::SubspaceTolerances,0)?`.
    pub fn caller_array_value(
        &self,
        array: super::AdvApp2VarCallerRealArray,
        index: usize,
    ) -> Result<f64, Failure> {
        self.reals.caller_array_value(array, index)
    }
    /// Own a caller array independently of scalar fields and numerical workspace;
    /// `let frame=frame.with_owned_caller_array(super::AdvApp2VarCallerRealArray::IsoRoots)`.
    pub fn with_owned_caller_array(mut self, array: super::AdvApp2VarCallerRealArray) -> Self {
        self.reals = self.reals.with_owned_caller_array(array);
        self
    }
    /// Inspect current owned caller inputs after success or failure;
    /// `frame.caller_array(super::AdvApp2VarCallerRealArray::SubspaceTolerances)?`.
    pub fn caller_array(&self, array: super::AdvApp2VarCallerRealArray) -> Result<&[f64], Failure> {
        self.reals.caller_array(array)
    }
    /// Bind the actual caller count to a forwarded caller argument or distinct
    /// owned slot. Callback parameter-count and derivative locals stay distinct;
    /// `let frame = frame.with_subspace_count_slot(0)?` shares it with slot zero.
    pub fn with_subspace_count_slot(mut self, slot: usize) -> Result<Self, Failure> {
        self.integers.bind_subspace_count(slot)?;
        Ok(self)
    }
    /// Bind caller continuity/cross-derivative controls to owned slots; callback
    /// U/V derivative locals remain distinct. `let frame=frame.with_caller_order_slots([5,5])?`.
    pub fn with_caller_order_slots(mut self, slots: [usize; 2]) -> Result<Self, Failure> {
        self.integers.bind_caller_orders(slots)?;
        Ok(self)
    }
    /// Bind piece and coefficient limits to owned caller slots;
    /// `let frame=frame.with_caller_limit_slots([5,5])?` retains a shared identity.
    pub fn with_caller_limit_slots(mut self, slots: [usize; 2]) -> Result<Self, Failure> {
        self.integers.bind_caller_limits(slots)?;
        Ok(self)
    }
    /// Bind root count and Jacobi degree to owned caller slots;
    /// `let frame=frame.with_caller_sampling_slots([5,5])?` retains shared identity.
    pub fn with_caller_sampling_slots(mut self, slots: [usize; 2]) -> Result<Self, Failure> {
        self.integers.bind_caller_sampling(slots)?;
        Ok(self)
    }
    /// Bind distinct caller dimension entries to forwarded or distinct owned
    /// integer slots; `let frame = frame.with_caller_dimension_slots(vec![0])?`.
    pub fn with_caller_dimension_slots(mut self, slots: Vec<usize>) -> Result<Self, Failure> {
        self.integers.bind_caller_dimensions(slots)?;
        Ok(self)
    }
    /// Inspect configured caller dimensions after writeback; `frame.caller_dimensions()`.
    pub fn caller_dimensions(&self) -> Option<Vec<i32>> {
        self.integers.caller_dimensions()
    }
    /// Inspect integer storage after an operation; `frame.integer_values()`.
    pub fn integer_values(&self) -> &[i32] {
        self.integers.values()
    }
    /// Inspect a caller real after write-back; `frame.real(AdvApp2VarRealField::UStart)`.
    pub fn real(&self, field: AdvApp2VarRealField) -> f64 {
        self.reals.real(field)
    }
    /// Approximate with combined field identities while retaining the existing
    /// numerical pipeline; `frame.approximate_iso(&mut request, &f, &mut output, &mut error)?`.
    pub fn approximate_iso(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarCombinedEvaluator,
        output: &mut IsoApproxOutput<'_>,
        error: &mut i32,
    ) -> Result<i32, Failure> {
        self.require_borrowed_integer_outputs()?;
        self.reals.require_slice_curve()?;
        self.integers.seed_caller_dimensions(request.ndimse)?;
        let state = Mutex::new(CombinedState {
            frame: Some(&mut self.integers),
            failure: None,
        });
        let bridge = CombinedBridge {
            state: &state,
            evaluator,
        };
        let result = self
            .reals
            .approximate_iso(request, &bridge, output, error)?;
        let state = state.into_inner().map_err(|_| poisoned_combined_frame())?;
        match state.failure {
            Some(failure) => Err(failure),
            None => Ok(result),
        }
    }
}
fn poisoned_combined_frame() -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        "AdvApp2VarIsoOperationFrame: poisoned=true; expected unpoisoned operation-local combined storage",
    )
}
