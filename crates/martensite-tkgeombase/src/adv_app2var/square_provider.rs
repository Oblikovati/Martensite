//! The square callback uses live caller slots and separate source-local arguments.
#[path = "square_owned_status.rs"]
mod owned_status;
#[path = "square_shared_samples.rs"]
mod shared_samples;
use super::approx_f2var::{SampledSquare, SquareTables, mma2ds1_mut_};
use super::integer_arguments::IntegerArguments;
use super::operation_frame_lease::{OperationFrameLease, OperationFrameState};
use super::square_roots::SquareRootBindings;
use super::{
    AdvApp2VarBufferEvaluation, AdvApp2VarEvaluation, AdvApp2VarEvaluationBuffers,
    AdvApp2VarEvaluationSlots, AdvApp2VarEvaluatorFunc2Var, AdvApp2VarIntegerField as Argument,
    AdvApp2VarSquareIntegerField as Field, AdvApp2VarSquareOperationFrame as Frame,
};
use martensite_tkernel::standard::Failure;
pub use owned_status::AdvApp2VarSquareStatusEvaluator;
pub use shared_samples::AdvApp2VarSquareBufferEvaluator;
use std::sync::{Mutex, OnceLock};
/// One square callback with actual caller incidence and source-local arguments;
/// `call.set_argument(AdvApp2VarIntegerField::Dimension,1)` updates incident caller counts.
pub struct AdvApp2VarSquareEvaluation<'view, 'call, Arguments = AdvApp2VarEvaluationSlots<'call>> {
    frame: &'view mut Frame,
    call: &'view mut Arguments,
    varying: Field,
    callback_lifetime: std::marker::PhantomData<&'call mut ()>,
    roots: SquareRootBindings,
    tables: super::square_tables::SquareTableBindings,
    intervals: super::square_intervals::SquareIntervalBindings,
}
/// Square caller identities with indexed parameter/sample storage;
/// `call.buffers().set_result(0,1.)` preserves a shared parameter slot.
pub type AdvApp2VarSquareBufferEvaluation<'view, 'call> =
    AdvApp2VarSquareEvaluation<'view, 'call, AdvApp2VarBufferEvaluation<'call>>;
impl<Arguments> AdvApp2VarSquareEvaluation<'_, '_, Arguments> {
    /// Inspect an original caller field; `call.integer(AdvApp2VarSquareIntegerField::VRootCount)`.
    pub fn integer(&self, field: Field) -> i32 {
        self.frame.integer(field)
    }
    /// Write a caller field with immediate incident observation;
    /// `call.set_integer(AdvApp2VarSquareIntegerField::VRootCount,1)`.
    pub fn set_integer(&mut self, field: Field, value: i32) {
        self.frame.set_integer(field, value);
    }
}
macro_rules! implement_square_arguments {
    ($arguments:ident) => {
        impl AdvApp2VarSquareEvaluation<'_, '_, $arguments<'_>> {
            /// Read a callback argument; `call.argument(AdvApp2VarIntegerField::FavoriteIso)`.
            pub fn argument(&self, field: Argument) -> i32 {
                match field {
                    Argument::Dimension => self.integer(Field::Dimension),
                    Argument::ParameterCount => self.integer(self.varying),
                    _ => self.call.argument_value(field),
                }
            }
            /// Change a callback argument while preserving its source identity;
            /// `call.set_argument(AdvApp2VarIntegerField::UOrder,2)` also changes its shared V order.
            pub fn set_argument(&mut self, field: Argument, value: i32) {
                match field {
                    Argument::Dimension => self.set_integer(Field::Dimension, value),
                    Argument::ParameterCount => self.set_integer(self.varying, value),
                    _ => self.call.set_argument_value(field, value),
                }
            }
            /// Access the copied local U interval; `call.u_bounds()[0]=0.`.
            pub fn u_bounds(&mut self) -> &mut [f64; 2] {
                self.call.u_start_end
            }
            /// Access the copied local V interval; `call.v_bounds()[1]=1.`.
            pub fn v_bounds(&mut self) -> &mut [f64; 2] {
                self.call.v_start_end
            }
            /// Access the current local constant; `*call.constant_parameter()=.5`.
            pub fn constant_parameter(&mut self) -> &mut f64 {
                self.call.const_param
            }
        }
    };
}
implement_square_arguments!(AdvApp2VarEvaluationSlots);
implement_square_arguments!(AdvApp2VarBufferEvaluation);
impl AdvApp2VarSquareEvaluation<'_, '_> {
    /// Access distinct caller parameter storage; `call.parameters()[0]=.5`.
    pub fn parameters(&mut self) -> &mut [f64] {
        self.call.parameters
    }
}
impl<'call> AdvApp2VarSquareBufferEvaluation<'_, 'call> {
    /// Read actual caller interval storage, independent of local bounds;
    /// `call.caller_interval(super::AdvApp2VarSquareIntervalAxis::U,0)?`.
    pub fn caller_interval(
        &self,
        axis: super::AdvApp2VarSquareIntervalAxis,
        index: usize,
    ) -> Result<f64, Failure> {
        self.call.buffers.allocation_value(self.intervals.slot(
            axis,
            index,
            self.call.buffers.allocation_capacity()?,
        )?)
    }
    /// Write a caller interval with immediate incident table/root observation;
    /// `call.set_caller_interval(super::AdvApp2VarSquareIntervalAxis::U,0,-3.)?`.
    pub fn set_caller_interval(
        &mut self,
        axis: super::AdvApp2VarSquareIntervalAxis,
        index: usize,
        value: f64,
    ) -> Result<(), Failure> {
        let slot = self
            .intervals
            .slot(axis, index, self.call.buffers.allocation_capacity()?)?;
        self.call.buffers.set_allocation_value(slot, value)
    }
    /// Read an original caller table through its owned origin;
    /// `call.caller_table(super::AdvApp2VarSquareTable::SumSum,0)?`.
    pub fn caller_table(
        &self,
        role: super::AdvApp2VarSquareTable,
        index: usize,
    ) -> Result<f64, Failure> {
        let slot = self
            .tables
            .slot(role, index, self.call.buffers.allocation_capacity()?)?;
        self.call.buffers.allocation_value(slot)
    }
    /// Write a table slot with immediate incident sample/root observation;
    /// `call.set_caller_table(super::AdvApp2VarSquareTable::SumSum,0,1.)?`.
    pub fn set_caller_table(
        &mut self,
        role: super::AdvApp2VarSquareTable,
        index: usize,
        value: f64,
    ) -> Result<(), Failure> {
        let slot = self
            .tables
            .slot(role, index, self.call.buffers.allocation_capacity()?)?;
        self.call.buffers.set_allocation_value(slot, value)
    }
    /// Read a signed caller table offset through its original initialized owner;
    /// `call.caller_table_signed(super::AdvApp2VarSquareTable::SumSum,-1)?`.
    pub fn caller_table_signed(
        &self,
        role: super::AdvApp2VarSquareTable,
        offset: isize,
    ) -> Result<f64, Failure> {
        let slot =
            self.tables
                .signed_slot(role, offset, self.call.buffers.allocation_capacity()?)?;
        self.call.buffers.allocation_value(slot)
    }
    /// Write a signed caller table offset with immediate shared-slot observation;
    /// `call.set_caller_table_signed(super::AdvApp2VarSquareTable::SumSum,-1,2.)?`.
    pub fn set_caller_table_signed(
        &mut self,
        role: super::AdvApp2VarSquareTable,
        offset: isize,
        value: f64,
    ) -> Result<(), Failure> {
        let slot =
            self.tables
                .signed_slot(role, offset, self.call.buffers.allocation_capacity()?)?;
        self.call.buffers.set_allocation_value(slot, value)
    }
    /// Read one actual owned caller root; `call.caller_root(super::AdvApp2VarSquareRootAxis::U,0)?`.
    pub fn caller_root(
        &self,
        axis: super::AdvApp2VarSquareRootAxis,
        index: usize,
    ) -> Result<f64, Failure> {
        let slot = self
            .roots
            .slot(axis, index, self.call.buffers.allocation_capacity()?)?;
        self.call.buffers.allocation_value(slot)
    }
    /// Write an owned root with immediate parameter/sample observation;
    /// `call.set_caller_root(super::AdvApp2VarSquareRootAxis::U,0,0.25)?`.
    pub fn set_caller_root(
        &mut self,
        axis: super::AdvApp2VarSquareRootAxis,
        index: usize,
        value: f64,
    ) -> Result<(), Failure> {
        let slot = self
            .roots
            .slot(axis, index, self.call.buffers.allocation_capacity()?)?;
        self.call.buffers.set_allocation_value(slot, value)
    }
    /// Access original indexed parameter/sample storage; `call.buffers().parameter(0)`.
    pub fn buffers(&mut self) -> &mut AdvApp2VarEvaluationBuffers<'call> {
        &mut self.call.buffers
    }
}
/// A square evaluator with explicit original caller identity. Matching closures
/// implement this trait; `let f=|_: &mut AdvApp2VarSquareEvaluation<'_, '_>, values:&mut[f64], error:&mut i32|{values.fill(1.);*error=0;};`.
pub trait AdvApp2VarSquareEvaluator {
    /// Evaluate using original caller slots; `f.evaluate_square(&mut call,values,error)`.
    fn evaluate_square(
        &self,
        call: &mut AdvApp2VarSquareEvaluation<'_, '_>,
        values: &mut [f64],
        error: &mut i32,
    );
}
impl<F: Fn(&mut AdvApp2VarSquareEvaluation<'_, '_>, &mut [f64], &mut i32)> AdvApp2VarSquareEvaluator
    for F
{
    fn evaluate_square(
        &self,
        call: &mut AdvApp2VarSquareEvaluation<'_, '_>,
        values: &mut [f64],
        error: &mut i32,
    ) {
        self(call, values, error);
    }
}
impl Frame {
    /// Sample through the existing square pipeline with live caller identities;
    /// `frame.sample_square(&mut request,&f,&mut tables,samples,parameters,error)?`.
    pub fn sample_square(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareEvaluator,
        tables: &mut SquareTables<'_>,
        samples: &mut [f64],
        parameters: &mut [f64],
        error: &mut i32,
    ) -> Result<i32, Failure> {
        self.require_separate_status()?;
        self.seed_request(request)?;
        let state = Mutex::new(OperationFrameState {
            frame: Some(self),
            failure: None,
        });
        let bridge = SquareCallerBridge {
            state: &state,
            evaluator,
            varying: OnceLock::new(),
        };
        let result = mma2ds1_mut_(request, &bridge, tables, samples, parameters, error);
        let state = state.into_inner().map_err(|_| poisoned_square())?;
        state
            .frame
            .expect("AdvApp2VarSquareOperationFrame: frame=None; expected restored operation frame")
            .write_request(request);
        state.failure.map_or(Ok(result), Err)
    }
}
struct SquareCallerBridge<'state, 'frame, 'eval, E> {
    state: &'state Mutex<OperationFrameState<'frame, Frame>>,
    evaluator: &'eval E,
    varying: OnceLock<Field>,
}
impl<E: AdvApp2VarSquareEvaluator> AdvApp2VarEvaluatorFunc2Var
    for SquareCallerBridge<'_, '_, '_, E>
{
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], error: &mut i32) {
        *error = 1;
    }
    fn caller_square_root_counts(&self) -> Option<[i32; 2]> {
        let mut state = self.state.lock().expect(
            "AdvApp2VarSquareOperationFrame: poisoned=true; expected unpoisoned stage frame",
        );
        let Some(frame) = state.frame.as_deref() else {
            state.failure = Some(active_square());
            drop(state);
            panic!(
                "AdvApp2VarSquareOperationFrame: callback active=true; expected stage access outside evaluator callback"
            );
        };
        Some(frame.root_counts())
    }
    fn evaluate_slots(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        values: &mut [f64],
        error: &mut i32,
    ) {
        evaluate_square_frame(
            self.state,
            &self.varying,
            call,
            error,
            |frame, call, varying, error| {
                self.evaluator.evaluate_square(
                    &mut AdvApp2VarSquareEvaluation {
                        frame,
                        call,
                        varying,
                        callback_lifetime: std::marker::PhantomData,
                        roots: SquareRootBindings::default(),
                        tables: super::square_tables::SquareTableBindings::default(),
                        intervals: super::square_intervals::SquareIntervalBindings::default(),
                    },
                    values,
                    error,
                );
            },
        );
    }
}
fn evaluate_square_frame<Arguments: IntegerArguments>(
    state: &Mutex<OperationFrameState<'_, Frame>>,
    varying: &OnceLock<Field>,
    call: &mut Arguments,
    error: &mut i32,
    evaluate: impl FnOnce(&mut Frame, &mut Arguments, Field, &mut i32),
) {
    let mut lease = match OperationFrameLease::take(state, poisoned_square, active_square) {
        Ok(lease) => lease,
        Err(failure) => {
            state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .failure = Some(failure);
            *error = 1;
            return;
        }
    };
    let varying = *varying.get_or_init(|| {
        if call.argument_value(Argument::FavoriteIso) == 1 {
            Field::VRootCount
        } else {
            Field::URootCount
        }
    });
    let frame=lease.frame_mut("AdvApp2VarSquareOperationFrame: leased frame=None; expected active exclusive callback frame");
    evaluate(frame, call, varying, error);
    call.set_argument_value(Argument::Dimension, frame.integer(Field::Dimension));
    call.set_argument_value(Argument::ParameterCount, frame.integer(varying));
}
fn poisoned_square() -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        "AdvApp2VarSquareOperationFrame: poisoned=true; expected unpoisoned operation-local frame",
    )
}
fn active_square() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarSquareOperationFrame: callback active=true; expected exclusive non-reentrant evaluator dispatch",
    )
}

#[cfg(test)]
#[path = "square_provider_tests.rs"]
mod tests;
