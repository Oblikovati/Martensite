//! Operation-local scalar incidence for evaluator calls.
use super::integer_frame::IntegerFrame;
use super::{AdvApp2VarEvaluationSlots, AdvApp2VarEvaluatorFunc2Var};
use martensite_tkernel::standard::Failure;
use std::sync::{Mutex, MutexGuard};

/// An integer argument's logical identity, independent of its storage slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarIntegerField {
    /// Number of reals per sample.
    Dimension,
    /// Preferred iso direction.
    FavoriteIso,
    /// Number of sample parameters.
    ParameterCount,
    /// U derivative order.
    UOrder,
    /// V derivative order.
    VOrder,
}
pub(super) const INTEGER_FIELDS: [AdvApp2VarIntegerField; 5] = [
    AdvApp2VarIntegerField::Dimension,
    AdvApp2VarIntegerField::FavoriteIso,
    AdvApp2VarIntegerField::ParameterCount,
    AdvApp2VarIntegerField::UOrder,
    AdvApp2VarIntegerField::VOrder,
];

/// Explicit integer argument-to-storage incidence; equal indices name one slot.
/// Indices are local to the owned values passed to the adapter, not global IDs.
/// `AdvApp2VarIntegerBindings { dimension: 0, favorite_iso: 0, parameter_count: 1, u_order: 2, v_order: 3 }` shares dimension and iso.
#[derive(Clone, Copy, Debug)]
pub struct AdvApp2VarIntegerBindings {
    /// Dimension slot index.
    pub dimension: usize,
    /// Preferred iso slot index.
    pub favorite_iso: usize,
    /// Parameter count slot index.
    pub parameter_count: usize,
    /// U derivative slot index.
    pub u_order: usize,
    /// V derivative slot index.
    pub v_order: usize,
}
impl AdvApp2VarIntegerBindings {
    pub(super) fn slot(self, field: AdvApp2VarIntegerField) -> usize {
        match field {
            AdvApp2VarIntegerField::Dimension => self.dimension,
            AdvApp2VarIntegerField::FavoriteIso => self.favorite_iso,
            AdvApp2VarIntegerField::ParameterCount => self.parameter_count,
            AdvApp2VarIntegerField::UOrder => self.u_order,
            AdvApp2VarIntegerField::VOrder => self.v_order,
        }
    }
}

/// A callback view into one owned scalar store. Shared integers are accessed by
/// value, so writes remain immediately visible through every incident field.
/// `call.set_integer(AdvApp2VarIntegerField::Dimension, 1)` changes all fields sharing that slot.
pub struct AdvApp2VarScalarEvaluation<'a, Parameters = &'a mut [f64]> {
    pub(super) values: &'a mut [i32],
    pub(super) bindings: AdvApp2VarIntegerBindings,
    /// Mutable local U bounds; these references remain disjoint from V bounds.
    pub u_start_end: &'a mut [f64; 2],
    /// Mutable local V bounds.
    pub v_start_end: &'a mut [f64; 2],
    /// Constant parameter.
    pub const_param: &'a mut f64,
    /// Mutable sample-parameter storage.
    pub parameters: Parameters,
}
impl<Parameters> AdvApp2VarScalarEvaluation<'_, Parameters> {
    pub(super) fn integer_slot(&self, slot: usize) -> i32 {
        self.values[slot]
    }
    pub(super) fn set_integer_slot(&mut self, slot: usize, value: i32) {
        self.values[slot] = value;
    }
    /// Read the current value through a logical field; `call.integer(AdvApp2VarIntegerField::Dimension)`.
    pub fn integer(&self, field: AdvApp2VarIntegerField) -> i32 {
        self.values[self.bindings.slot(field)]
    }
    /// Write one storage slot through its field; `call.set_integer(AdvApp2VarIntegerField::ParameterCount, 2)`.
    pub fn set_integer(&mut self, field: AdvApp2VarIntegerField, value: i32) {
        self.values[self.bindings.slot(field)] = value;
    }
}

impl<'a> AdvApp2VarScalarEvaluation<'a> {
    pub(super) fn from_call(
        call: &'a mut AdvApp2VarEvaluationSlots<'_>,
        values: &'a mut [i32],
        bindings: AdvApp2VarIntegerBindings,
    ) -> Self {
        Self {
            values,
            bindings,
            u_start_end: call.u_start_end,
            v_start_end: call.v_start_end,
            const_param: call.const_param,
            parameters: call.parameters,
        }
    }
}

/// Evaluator accepting explicit scalar incidence rather than distinct integer
/// references. Matching closures implement it; `let evaluator = |call: &mut AdvApp2VarScalarEvaluation<'_>, result: &mut [f64], error: &mut i32| { result.fill(f64::from(call.integer(AdvApp2VarIntegerField::Dimension))); *error = 0; };`.
pub trait AdvApp2VarScalarEvaluator {
    /// Evaluate with immediate shared-slot observation; `evaluator.evaluate_scalars(&mut call, &mut values, &mut error)`.
    fn evaluate_scalars(
        &self,
        call: &mut AdvApp2VarScalarEvaluation<'_>,
        result: &mut [f64],
        error: &mut i32,
    );
}
impl<F> AdvApp2VarScalarEvaluator for F
where
    F: Fn(&mut AdvApp2VarScalarEvaluation<'_>, &mut [f64], &mut i32),
{
    fn evaluate_scalars(
        &self,
        call: &mut AdvApp2VarScalarEvaluation<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self(call, result, error);
    }
}

struct ScalarProviderState {
    integers: IntegerFrame,
    failure: Option<Failure>,
}

/// An operation-local evaluator adapter with explicit scalar-slot identity.
/// The generated call seeds each unique slot once; inconsistent incident input
/// values are rejected. The provider sees live aliases, then logical values are
/// copied back to the existing numerical pipeline. Derivative bindings must
/// preserve the pipeline's actual shared/independent local identity.
/// `AdvApp2VarScalarProvider::new(vec![0; 4], bindings, evaluator)?`.
pub struct AdvApp2VarScalarProvider<E> {
    evaluator: E,
    state: Mutex<ScalarProviderState>,
}
impl<E: AdvApp2VarScalarEvaluator> AdvApp2VarScalarProvider<E> {
    /// Validate bindings once and own the operation's scalar storage;
    /// `AdvApp2VarScalarProvider::new(vec![0; 3], bindings, evaluator)?`.
    pub fn new(
        values: Vec<i32>,
        bindings: AdvApp2VarIntegerBindings,
        evaluator: E,
    ) -> Result<Self, Failure> {
        let integers = IntegerFrame::new(values, bindings)?;
        Ok(Self {
            evaluator,
            state: Mutex::new(ScalarProviderState {
                integers,
                failure: None,
            }),
        })
    }
    /// Return the last adapter failure, separately from evaluator error codes;
    /// `assert!(provider.last_failure()?.is_none())` after a valid call.
    pub fn last_failure(&self) -> Result<Option<Failure>, Failure> {
        Ok(self.lock_state()?.failure.clone())
    }
    /// Snapshot owned integer storage after a call; `provider.integer_values()?`.
    pub fn integer_values(&self) -> Result<Vec<i32>, Failure> {
        Ok(self.lock_state()?.integers.values().to_vec())
    }
    fn lock_state(&self) -> Result<MutexGuard<'_, ScalarProviderState>, Failure> {
        self.state.lock().map_err(|_| Failure::with_message("Standard_ProgramError", "AdvApp2VarScalarProvider: poisoned=true; expected unpoisoned operation-local scalar storage"))
    }
    fn record_failure(&self, failure: Failure, error: &mut i32) {
        if let Ok(mut state) = self.lock_state() {
            state.failure = Some(failure);
        }
        *error = 1;
    }
    fn evaluate_call(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) -> Result<(), Failure> {
        let mut state = self.lock_state()?;
        state.failure = None;
        state.integers.evaluate(call, |view| {
            self.evaluator.evaluate_scalars(view, result, error)
        })
    }
}
impl<E: AdvApp2VarScalarEvaluator> AdvApp2VarEvaluatorFunc2Var for AdvApp2VarScalarProvider<E> {
    fn evaluate(&self, _: &super::AdvApp2VarEvaluation<'_>, _: &mut [f64], error: &mut i32) {
        self.record_failure(Failure::with_message(
            "Standard_DomainError",
            "AdvApp2VarScalarProvider: immutable call=true; expected an evaluate_slots call with explicit derivative identity",
        ), error);
    }
    fn evaluate_slots(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        if let Err(failure) = self.evaluate_call(call, result, error) {
            self.record_failure(failure, error);
        }
    }
}

pub(super) fn argument_value(
    call: &impl super::integer_arguments::IntegerArguments,
    field: AdvApp2VarIntegerField,
) -> i32 {
    call.argument_value(field)
}
pub(super) fn set_argument_value(
    call: &mut impl super::integer_arguments::IntegerArguments,
    field: AdvApp2VarIntegerField,
    value: i32,
) {
    call.set_argument_value(field, value);
}
