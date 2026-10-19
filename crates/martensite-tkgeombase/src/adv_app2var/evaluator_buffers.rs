//! Sequential indexed access preserves overlapping callback buffer identities.
use super::evaluator::AdvApp2VarEvaluatorFunc2Var;
use super::{AdvApp2VarDerivativeOrders, AdvApp2VarEvaluationSlots, AdvApp2VarIsoCallerIntegers};

/// Callback storage with one owner even when parameter and result regions overlap.
/// `buffers.set_result(0, 1.); let value = buffers.parameter(0);`.
pub struct AdvApp2VarEvaluationBuffers<'a> {
    storage: BufferRegions<'a>,
}
use super::AdvApp2VarEndpointOutput;
use super::real_slots::OwnedEndpointSlots;
enum BufferRegions<'a> {
    OwnedEndpoint {
        parameters: &'a mut [f64],
        results: &'a mut OwnedEndpointSlots,
        end: AdvApp2VarEndpointOutput,
        offset: i32,
    },
    Separate {
        parameters: &'a mut [f64],
        results: &'a mut [f64],
        legacy_parameters: usize,
    },
    Shared {
        values: &'a mut [f64],
        parameter_start: usize,
        parameter_end: usize,
        result_start: usize,
        legacy_end: usize,
    },
}
impl<'a> AdvApp2VarEvaluationBuffers<'a> {
    pub(super) fn owned_endpoint(
        parameters: &'a mut [f64],
        results: &'a mut OwnedEndpointSlots,
        end: AdvApp2VarEndpointOutput,
        offset: i32,
    ) -> Self {
        Self {
            storage: BufferRegions::OwnedEndpoint {
                parameters,
                results,
                end,
                offset,
            },
        }
    }
    pub(super) fn separate(
        parameters: &'a mut [f64],
        results: &'a mut [f64],
        legacy_parameters: usize,
    ) -> Self {
        Self {
            storage: BufferRegions::Separate {
                parameters,
                results,
                legacy_parameters,
            },
        }
    }
    pub(super) fn shared(
        values: &'a mut [f64],
        parameter_start: usize,
        parameter_end: usize,
        result_start: usize,
        legacy_end: usize,
    ) -> Self {
        Self {
            storage: BufferRegions::Shared {
                values,
                parameter_start,
                parameter_end,
                result_start,
                legacy_end,
            },
        }
    }
    pub(super) fn allocation_value(
        &self,
        index: usize,
    ) -> Result<f64, martensite_tkernel::standard::Failure> {
        let BufferRegions::Shared { values, .. } = &self.storage else {
            return Err(shared_allocation_required());
        };
        values
            .get(index)
            .copied()
            .ok_or_else(|| allocation_index_error(index, values.len()))
    }
    pub(super) fn set_allocation_value(
        &mut self,
        index: usize,
        value: f64,
    ) -> Result<(), martensite_tkernel::standard::Failure> {
        let BufferRegions::Shared { values, .. } = &mut self.storage else {
            return Err(shared_allocation_required());
        };
        let capacity = values.len();
        *values
            .get_mut(index)
            .ok_or_else(|| allocation_index_error(index, capacity))? = value;
        Ok(())
    }
    pub(super) fn allocation_suffix(
        &self,
        start: usize,
    ) -> Result<&[f64], martensite_tkernel::standard::Failure> {
        let BufferRegions::Shared { values, .. } = &self.storage else {
            return Err(shared_allocation_required());
        };
        values
            .get(start..)
            .ok_or_else(|| allocation_index_error(start, values.len()))
    }
    pub(super) fn allocation_suffix_mut(
        &mut self,
        start: usize,
    ) -> Result<&mut [f64], martensite_tkernel::standard::Failure> {
        let BufferRegions::Shared { values, .. } = &mut self.storage else {
            return Err(shared_allocation_required());
        };
        let capacity = values.len();
        values
            .get_mut(start..)
            .ok_or_else(|| allocation_index_error(start, capacity))
    }
    pub(super) fn allocation_capacity(
        &self,
    ) -> Result<usize, martensite_tkernel::standard::Failure> {
        match &self.storage {
            BufferRegions::Shared { values, .. } => Ok(values.len()),
            _ => Err(shared_allocation_required()),
        }
    }
    pub(super) fn reborrow(&mut self) -> AdvApp2VarEvaluationBuffers<'_> {
        match &mut self.storage {
            BufferRegions::OwnedEndpoint {
                parameters,
                results,
                end,
                offset,
            } => AdvApp2VarEvaluationBuffers::owned_endpoint(parameters, results, *end, *offset),
            BufferRegions::Separate {
                parameters,
                results,
                legacy_parameters,
            } => AdvApp2VarEvaluationBuffers::separate(parameters, results, *legacy_parameters),
            BufferRegions::Shared {
                values,
                parameter_start,
                parameter_end,
                result_start,
                legacy_end,
            } => AdvApp2VarEvaluationBuffers::shared(
                values,
                *parameter_start,
                *parameter_end,
                *result_start,
                *legacy_end,
            ),
        }
    }
    /// Read a parameter from its original operation slot; `buffers.parameter(0)`.
    pub fn parameter(&self, index: usize) -> f64 {
        match &self.storage {
            BufferRegions::Separate { parameters, .. }
            | BufferRegions::OwnedEndpoint { parameters, .. } => parameters[index],
            BufferRegions::Shared {
                values,
                parameter_start,
                ..
            } => values[parameter_start + index],
        }
    }
    /// Write one parameter, visible immediately through an overlapping result;
    /// `buffers.set_parameter(0, 0.5)`.
    pub fn set_parameter(&mut self, index: usize, value: f64) {
        match &mut self.storage {
            BufferRegions::Separate { parameters, .. }
            | BufferRegions::OwnedEndpoint { parameters, .. } => parameters[index] = value,
            BufferRegions::Shared {
                values,
                parameter_start,
                ..
            } => values[*parameter_start + index] = value,
        }
    }
    /// Read one result, including preceding parameter writes; `buffers.result(0)`.
    pub fn result(&self, index: usize) -> f64 {
        match &self.storage {
            BufferRegions::OwnedEndpoint {
                results,
                end,
                offset,
                ..
            } => results.value(*end, endpoint_offset(*offset, index)).expect(
                "AdvApp2VarEndpointOutput: invalid result read; expected initialized callback slot",
            ),
            BufferRegions::Separate { results, .. } => results[index],
            BufferRegions::Shared {
                values,
                result_start,
                ..
            } => values[result_start + index],
        }
    }
    // MMA2ds1 can read an initialized slot before its original sample pointer.
    pub(super) fn result_at_signed_offset(
        &self,
        offset: isize,
    ) -> Result<f64, martensite_tkernel::standard::Failure> {
        if let BufferRegions::OwnedEndpoint {
            results,
            end,
            offset: start,
            ..
        } = &self.storage
        {
            let index = i32::try_from(offset)
                .ok()
                .and_then(|offset| start.checked_add(offset))
                .ok_or_else(|| signed_result_offset_error(offset, 0, 0))?;
            return results.value(*end, index);
        }
        let (values, origin): (&[f64], usize) = match &self.storage {
            BufferRegions::OwnedEndpoint { .. } => unreachable!(),
            BufferRegions::Separate { results, .. } => (results, 0),
            BufferRegions::Shared {
                values,
                result_start,
                ..
            } => (values, *result_start),
        };
        origin
            .checked_add_signed(offset)
            .and_then(|slot| values.get(slot))
            .copied()
            .ok_or_else(|| signed_result_offset_error(offset, origin, values.len()))
    }
    /// Write one result without borrowing an overlapping slice; `buffers.set_result(0, 1.)`.
    pub fn set_result(&mut self, index: usize, value: f64) {
        match &mut self.storage {
            BufferRegions::OwnedEndpoint {results,end,offset,..} => results.set_value(*end,endpoint_offset(*offset,index),value).expect("AdvApp2VarEndpointOutput: invalid result write; expected initialized callback slot"),
            BufferRegions::Separate { results, .. } => results[index] = value,
            BufferRegions::Shared {
                values,
                result_start,
                ..
            } => values[*result_start + index] = value,
        }
    }
    fn legacy_views(&mut self) -> Option<(&mut [f64], &mut [f64])> {
        match &mut self.storage {
            BufferRegions::OwnedEndpoint { .. } => None,
            BufferRegions::Separate {
                parameters,
                results,
                legacy_parameters,
            } => Some((&mut parameters[..*legacy_parameters], results)),
            BufferRegions::Shared {
                values,
                parameter_start,
                parameter_end,
                result_start,
                legacy_end,
            } => {
                let (parameters, results) = values[..*legacy_end].split_at_mut(*result_start);
                Some((&mut parameters[*parameter_start..*parameter_end], results))
            }
        }
    }
}

fn endpoint_offset(start: i32, index: usize) -> i32 {
    i32::try_from(index)
        .ok()
        .and_then(|index| start.checked_add(index))
        .unwrap_or_else(|| panic!("AdvApp2VarEndpointOutput: offset={start}, index={index}; expected logical result index inside i32 range"))
}
fn shared_allocation_required() -> martensite_tkernel::standard::Failure {
    martensite_tkernel::standard::Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarEvaluationBuffers: shared allocation=false; expected initialized owned allocation for absolute slot access",
    )
}
fn allocation_index_error(index: usize, capacity: usize) -> martensite_tkernel::standard::Failure {
    martensite_tkernel::standard::Failure::with_message(
        "Standard_OutOfRange",
        &format!(
            "AdvApp2VarEvaluationBuffers: allocation index={index}; expected element in 0..{capacity}"
        ),
    )
}

/// Mutable scalar arguments and indexed buffer regions for one evaluator call.
/// Override `AdvApp2VarEvaluatorFunc2Var::evaluate_iso_buffers` to observe sharing.
pub struct AdvApp2VarBufferEvaluation<'a> {
    /// Number of reals per sample.
    pub dimension: &'a mut i32,
    /// Local U parameter bounds.
    pub u_start_end: &'a mut [f64; 2],
    /// Local V parameter bounds.
    pub v_start_end: &'a mut [f64; 2],
    /// Constant-parameter direction.
    pub favor_iso: &'a mut i32,
    /// Constant parameter value.
    pub const_param: &'a mut f64,
    /// Number of local parameters.
    pub nb_params: &'a mut i32,
    /// Local derivative identities.
    pub orders: AdvApp2VarDerivativeOrders<'a>,
    /// Operation-owned parameter and result regions.
    pub buffers: AdvApp2VarEvaluationBuffers<'a>,
}
impl AdvApp2VarBufferEvaluation<'_> {
    pub(super) fn evaluate_square_legacy(
        &mut self,
        evaluator: &(impl AdvApp2VarEvaluatorFunc2Var + ?Sized),
        error: &mut i32,
    ) {
        let Some((parameters, result)) = self.buffers.legacy_views() else {
            *error = 1;
            return;
        };
        let mut call = AdvApp2VarEvaluationSlots {
            dimension: self.dimension,
            u_start_end: self.u_start_end,
            v_start_end: self.v_start_end,
            favor_iso: self.favor_iso,
            const_param: self.const_param,
            nb_params: self.nb_params,
            parameters,
            orders: self.orders.reborrow(),
        };
        evaluator.evaluate_slots(&mut call, result, error);
    }
    pub(super) fn evaluate_legacy(
        &mut self,
        evaluator: &(impl AdvApp2VarEvaluatorFunc2Var + ?Sized),
        callers: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        let Some((parameters, result)) = self.buffers.legacy_views() else {
            *error = 1;
            return;
        };
        let previous = self.orders.u();
        let (mut u, mut v) = (previous, self.orders.v());
        let mut call = AdvApp2VarEvaluationSlots {
            dimension: self.dimension,
            u_start_end: self.u_start_end,
            v_start_end: self.v_start_end,
            favor_iso: self.favor_iso,
            const_param: self.const_param,
            nb_params: self.nb_params,
            parameters,
            orders: AdvApp2VarDerivativeOrders::Independent {
                u: &mut u,
                v: &mut v,
            },
        };
        evaluator.evaluate_iso_callers(&mut call, callers, result, error);
        match &mut self.orders {
            AdvApp2VarDerivativeOrders::Independent {
                u: original_u,
                v: original_v,
            } => {
                **original_u = u;
                **original_v = v;
            }
            AdvApp2VarDerivativeOrders::Shared(original) => {
                **original = if u != previous { u } else { v }
            }
        }
    }
}
fn signed_result_offset_error(
    offset: isize,
    origin: usize,
    capacity: usize,
) -> martensite_tkernel::standard::Failure {
    martensite_tkernel::standard::Failure::with_message(
        "Standard_OutOfRange",
        &format!(
            "AdvApp2VarEvaluationBuffers: result offset={offset}, origin={origin}; expected initialized physical slot in 0..{capacity}"
        ),
    )
}

#[cfg(test)]
#[path = "evaluator_buffers_tests.rs"]
mod tests;
