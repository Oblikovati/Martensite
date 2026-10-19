//! Original caller status and completed-piece identities live in the integer owner.
#[path = "iso_endpoint_outputs.rs"]
mod endpoints;
#[path = "iso_error_outputs.rs"]
mod error_outputs;
#[path = "iso_real_output_mirrors.rs"]
mod real_mirrors;
use super::*;
#[path = "iso_coefficient_counts.rs"]
mod counts;
#[path = "iso_cut_outputs.rs"]
mod cuts;
#[path = "iso_folded_outputs.rs"]
mod folded;
/// One original caller integer output; `AdvApp2VarIsoIntegerOutput::Status`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarIsoIntegerOutput {
    /// Original caller error/status pointer.
    Status,
    /// Original caller completed-piece count pointer.
    CompletedPieces,
}
impl AdvApp2VarIsoIntegerOutput {
    fn index(self) -> usize {
        match self {
            Self::Status => 0,
            Self::CompletedPieces => 1,
        }
    }
}
fn missing_outputs() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: integer outputs configured=false; expected owned status and completed-piece slots",
    )
}
impl AdvApp2VarIsoOperationFrame {
    /// Bind original caller status/piece slots in existing initialized integer storage;
    /// `let frame=frame.with_caller_output_slots([5,6])?`.
    pub fn with_caller_output_slots(mut self, slots: [usize; 2]) -> Result<Self, Failure> {
        self.integers.validate_output_slots(slots)?;
        self.integer_outputs = Some(slots);
        Ok(self)
    }
    /// Inspect actual caller output after normal/error return or unwind;
    /// `frame.caller_output(AdvApp2VarIsoIntegerOutput::Status)?`.
    pub fn caller_output(&self, field: AdvApp2VarIsoIntegerOutput) -> Result<i32, Failure> {
        let slots = self.integer_outputs.ok_or_else(missing_outputs)?;
        Ok(self.integers.output_slot(slots[field.index()]))
    }
    pub(super) fn require_borrowed_integer_outputs(&self) -> Result<(), Failure> {
        if self.endpoint_origins.is_some() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "AdvApp2VarIsoOperationFrame: endpoint origins configured=true; expected owned-output operation instead of borrowed endpoint storage",
            ));
        }
        if self.error_origins.is_some() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "AdvApp2VarIsoOperationFrame: error origins configured=true; expected owned-output operation instead of borrowed error storage",
            ));
        }
        if self.folded_origins.is_some() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "AdvApp2VarIsoOperationFrame: folded origins configured=true; expected owned-output operation instead of borrowed folded storage",
            ));
        }
        if self.cut_origin.is_some() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "AdvApp2VarIsoOperationFrame: cut origin configured=true; expected owned-output operation instead of borrowed cut storage",
            ));
        }
        if self.integer_outputs.is_none() && self.coefficient_origin.is_none() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            "AdvApp2VarIsoOperationFrame: integer outputs configured=true; expected owned-output operation instead of borrowed output scalars",
        ))
    }
}
impl<Parameters> AdvApp2VarCombinedEvaluation<'_, '_, Parameters> {
    /// Read actual original caller output with immediate input alias observation;
    /// `call.caller_output(AdvApp2VarIsoIntegerOutput::Status)?`.
    pub fn caller_output(&self, field: AdvApp2VarIsoIntegerOutput) -> Result<i32, Failure> {
        let slots = self.caller_output_slots.ok_or_else(missing_outputs)?;
        Ok(self.integers.integer_slot(slots[field.index()]))
    }
    /// Write original caller status or completed pieces through one owned slot;
    /// `call.set_caller_output(AdvApp2VarIsoIntegerOutput::Status,1)?`.
    pub fn set_caller_output(
        &mut self,
        field: AdvApp2VarIsoIntegerOutput,
        value: i32,
    ) -> Result<(), Failure> {
        let slots = self.caller_output_slots.ok_or_else(missing_outputs)?;
        self.integers.set_integer_slot(slots[field.index()], value);
        Ok(())
    }
}
/// Evaluator with actual owned caller status, without a second mutable error reference;
/// `let f=|call:&mut AdvApp2VarCombinedBufferEvaluation<'_, '_,'_>|{call.set_caller_output(AdvApp2VarIsoIntegerOutput::Status,0).unwrap();};`.
pub trait AdvApp2VarOwnedIsoEvaluator {
    /// Evaluate full indexed workspace and original output identities; `f.evaluate_owned_iso(&mut call)`.
    fn evaluate_owned_iso(&self, call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>);
}
impl<F: Fn(&mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>)> AdvApp2VarOwnedIsoEvaluator for F {
    fn evaluate_owned_iso(&self, call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>) {
        self(call);
    }
}
impl AdvApp2VarIsoOperationFrame {
    /// Approximate using actual caller status and piece slots, with a synchronized piece-count mirror;
    /// `frame.approximate_iso_owned_outputs(&mut request,&f,&mut output)?`.
    pub fn approximate_iso_owned_outputs(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarOwnedIsoEvaluator,
        output: &mut IsoApproxOutput<'_>,
    ) -> Result<i32, Failure> {
        self.reals.require_slice_curve()?;
        self.run_owned_integer_outputs(request, evaluator, output)
    }
    /// Compose actual integer outputs with a curve in the existing initialized real owner;
    /// `frame.approximate_iso_owned_output_curve(&mut request,&f,&mut output)?`.
    pub fn approximate_iso_owned_output_curve(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarOwnedIsoEvaluator,
        output: &mut crate::adv_app2var::approx_f2var::IsoApproxCallerOutput<
            '_,
            crate::adv_app2var::AdvApp2VarCallerCurveSlots,
        >,
    ) -> Result<i32, Failure> {
        self.reals.validate_curve_owner(&output.courbe)?;
        self.run_owned_integer_outputs(request, evaluator, output)
    }
    fn run_owned_integer_outputs<
        Curve: crate::adv_app2var::approx_f2var::curve_output::CurveCoefficientStorage,
    >(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarOwnedIsoEvaluator,
        output: &mut crate::adv_app2var::approx_f2var::IsoApproxCallerOutput<'_, Curve>,
    ) -> Result<i32, Failure> {
        self.run_owned_integer_outputs_with_workspace(
            request,
            evaluator,
            output,
            &mut crate::adv_app2var::sys_base::numeric_workspace::HeapNumericWorkspace,
        )
    }
    fn run_owned_integer_outputs_with_workspace<
        Curve: crate::adv_app2var::approx_f2var::curve_output::CurveCoefficientStorage,
    >(
        &mut self,
        request: &mut IsoApproxRequest<'_>,
        evaluator: &impl AdvApp2VarOwnedIsoEvaluator,
        output: &mut crate::adv_app2var::approx_f2var::IsoApproxCallerOutput<'_, Curve>,
        workspace: &mut dyn crate::adv_app2var::sys_base::numeric_workspace::NumericWorkspace,
    ) -> Result<i32, Failure> {
        let slots = self.integer_outputs.ok_or_else(missing_outputs)?;
        self.validate_owned_integer_outputs(request, output.nbcrbe)?;
        self.validate_count_mirror(output.ncoeff)?;
        self.validate_cut_mirror(output.tabdec)?;
        self.validate_folded_mirrors(output.somtab, output.diftab)?;
        self.validate_error_mirrors(output.errmax, output.errmoy)?;
        self.validate_endpoint_mirrors(output.contr1, output.contr2)?;
        let owned_cuts = self.cut_origin.is_some();
        let owned_counts = self.coefficient_origin.is_some();
        self.integers.seed_caller_dimensions(request.ndimse)?;
        self.reals.seed_request(request)?;
        let state = Mutex::new(buffers::IndexedCombinedState {
            frame: Some(self),
            failure: None,
        });
        let adapter = OwnedIsoAdapter(evaluator);
        let bridge = buffers::IndexedCombinedBridge {
            state: &state,
            evaluator: &adapter,
        };
        let mut integers = OwnedCurveIntegers {
            state: &state,
            slots,
        };
        let (mut tables, mirror) =
            crate::adv_app2var::approx_f2var::curve_output::borrowed_curve_tables(output);
        let result = crate::adv_app2var::approx_f2var::mma2fnc_with_workspace(
            request,
            &bridge,
            &mut tables,
            &mut integers,
            workspace,
        );
        *mirror = integers.pieces();
        if owned_counts {
            for (index, count) in output.ncoeff.iter_mut().enumerate() {
                *count = integers.count(&[], index as i32);
            }
        }
        if owned_cuts {
            for (index, cut) in output.tabdec.iter_mut().enumerate() {
                *cut=crate::adv_app2var::approx_f2var::curve_cut_storage::CurveCutParameters::cut_parameter(&integers,&[],index as i32);
            }
        }
        let state = state.into_inner().map_err(|_| poisoned_combined_frame())?;
        let frame = state.frame.expect(
            "AdvApp2VarIsoOperationFrame: frame=None; expected restored owned-output operation",
        );
        frame.integers.refresh_iso_request(request);
        frame.reals.write_request(request);
        frame.synchronize_folded_mirrors(output.somtab, output.diftab)?;
        frame.synchronize_error_mirrors(output.errmax, output.errmoy)?;
        frame.synchronize_endpoint_mirrors(output.contr1, output.contr2)?;
        state.failure.map_or(Ok(result), Err)
    }
    fn validate_owned_integer_outputs(
        &self,
        request: &IsoApproxRequest<'_>,
        mirror: i32,
    ) -> Result<(), Failure> {
        self.integers.validate_iso_request(request)?;
        let actual = self.caller_output(AdvApp2VarIsoIntegerOutput::CompletedPieces)?;
        if actual == mirror {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            &format!(
                "AdvApp2VarIsoOperationFrame: completed-piece mirror={mirror}; expected initialized caller value={actual}"
            ),
        ))
    }
}
struct OwnedIsoAdapter<'a, E>(&'a E);
impl<E: AdvApp2VarOwnedIsoEvaluator> AdvApp2VarCombinedBufferEvaluator for OwnedIsoAdapter<'_, E> {
    fn evaluate_combined_buffers(
        &self,
        call: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>,
        error: &mut i32,
    ) {
        self.0.evaluate_owned_iso(call);
        *error=call.caller_output(AdvApp2VarIsoIntegerOutput::Status).expect("AdvApp2VarIsoOperationFrame: missing owned status; expected validated output callback");
    }
}
use crate::adv_app2var::approx_f2var::curve_integers::CurveIntegerOutputs;
struct OwnedCurveIntegers<'state, 'frame> {
    state: &'state Mutex<buffers::IndexedCombinedState<'frame>>,
    slots: [usize; 2],
}
impl CurveIntegerOutputs for OwnedCurveIntegers<'_, '_> {
    fn folded_piece(
        &self,
        offset: i32,
    ) -> Option<crate::adv_app2var::real_slots::OwnedFoldedSlots> {
        self.with_frame(|frame| {
            frame
                .folded_origins
                .map(|origins| frame.reals.folded_slots(origins, offset))
        })
    }

    fn endpoint_piece(
        &self,
        offset: i32,
    ) -> Option<crate::adv_app2var::real_slots::OwnedEndpointSlots> {
        self.with_frame(|frame| {
            frame
                .endpoint_origins
                .map(|origins| frame.reals.endpoint_slots(origins, offset))
        })
    }
    fn error_piece(&self, offset: i32) -> Option<crate::adv_app2var::real_slots::OwnedErrorSlots> {
        self.with_frame(|frame| {
            frame
                .error_origins
                .map(|origins| frame.reals.error_slots(origins, offset))
        })
    }
    fn subspace_tolerance(&self, values: &[f64], index: usize) -> f64 {
        self.with_frame(|frame| {
            frame.reals.caller_array_value_or(
                crate::adv_app2var::AdvApp2VarCallerRealArray::SubspaceTolerances,
                values,
                index,
            )
        })
    }
    fn coefficient_count(&self, values: &[i32], index: i32) -> i32 {
        self.count(values, index)
    }
    fn set_coefficient_count(
        &mut self,
        values: &mut [i32],
        index: i32,
        value: i32,
        request: &mut IsoApproxRequest<'_>,
    ) {
        self.write_count(values, index, value, request);
    }
    fn subspace_dimension(&self, values: &[i32], index: usize) -> i32 {
        self.with_frame(|frame| frame.integers.live_caller_dimension(values, index))
    }

    fn status(&self) -> i32 {
        self.with_frame(|frame| frame.integers.output_slot(self.slots[0]))
    }
    fn pieces(&self) -> i32 {
        self.with_frame(|frame| frame.integers.output_slot(self.slots[1]))
    }
    fn set_status(&mut self, value: i32, request: &mut IsoApproxRequest<'_>) {
        self.write_request_slot(self.slots[0], value, request);
    }
    fn set_pieces(&mut self, value: i32, request: &mut IsoApproxRequest<'_>) {
        self.write_request_slot(self.slots[1], value, request);
    }
    fn prepare_sampling(&mut self, request: &mut IsoApproxRequest<'_>) {
        self.set_status(0, request);
    }
    fn refresh_request(&self, request: &mut IsoApproxRequest<'_>) {
        self.with_frame(|frame| frame.integers.refresh_iso_request(request));
    }
    fn with_status<ResultValue>(
        &mut self,
        operation: impl FnOnce(&mut i32) -> ResultValue,
    ) -> ResultValue {
        let mut code = self.status();
        let result = operation(&mut code);
        self.write_slot(self.slots[0], code);
        result
    }
}
impl OwnedCurveIntegers<'_, '_> {
    fn write_request_slot(&self, slot: usize, value: i32, request: &mut IsoApproxRequest<'_>) {
        self.with_frame(|frame| {
            frame.integers.set_output_slot(slot, value);
            frame.integers.refresh_iso_request(request);
        });
    }
    fn write_slot(&self, slot: usize, value: i32) {
        self.with_frame(|frame| frame.integers.set_output_slot(slot, value));
    }
    fn with_frame<ResultValue>(
        &self,
        operation: impl FnOnce(&mut AdvApp2VarIsoOperationFrame) -> ResultValue,
    ) -> ResultValue {
        let mut state = self
            .state
            .lock()
            .expect("AdvApp2VarIsoOperationFrame: poisoned=true; expected unpoisoned output stage");
        let Some(frame) = state.frame.as_deref_mut() else {
            state.failure = Some(Failure::with_message(
                "Standard_DomainError",
                "AdvApp2VarIsoOperationFrame: callback active=true; expected owned output stage outside evaluator",
            ));
            drop(state);
            panic!(
                "AdvApp2VarIsoOperationFrame: callback active=true; expected resident output stage"
            );
        };
        operation(frame)
    }
}

#[cfg(test)]
#[path = "iso_integer_outputs_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "iso_curve_resource_tests.rs"]
mod resource_tests;
