//! Reusable operation-local integer storage behind evaluator boundaries.
use super::integer_arguments::IntegerArguments;
#[path = "iso_integer_output_frame.rs"]
mod outputs;
use super::{AdvApp2VarCallerOrderField, AdvApp2VarEvaluationSlots};
const CALLER_ORDER_FIELDS: [AdvApp2VarCallerOrderField; 2] = [
    AdvApp2VarCallerOrderField::ContinuityOrder,
    AdvApp2VarCallerOrderField::CrossDerivativeOrder,
];
use super::scalar_incidence::{ScalarIncidence, scalar_incidence};
use super::scalar_provider::{
    AdvApp2VarIntegerBindings, AdvApp2VarIntegerField, AdvApp2VarScalarEvaluation, INTEGER_FIELDS,
    argument_value, set_argument_value,
};
use martensite_tkernel::standard::Failure;
#[derive(Clone, Copy)]
enum FrameIntegerField {
    Callback(AdvApp2VarIntegerField),
    SubspaceCount,
    CallerDimension(usize),
    CallerOrder(AdvApp2VarCallerOrderField),
    CallerLimit(super::AdvApp2VarCallerLimitField),
    CallerSampling(super::AdvApp2VarCallerSamplingField),
}
impl std::fmt::Debug for FrameIntegerField {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Callback(field) => std::fmt::Debug::fmt(field, formatter),
            Self::SubspaceCount => formatter.write_str("SubspaceCount"),
            Self::CallerDimension(index) => formatter
                .debug_tuple("SubspaceDimension")
                .field(index)
                .finish(),
            Self::CallerOrder(field) => std::fmt::Debug::fmt(field, formatter),
            Self::CallerLimit(field) => std::fmt::Debug::fmt(field, formatter),
            Self::CallerSampling(field) => std::fmt::Debug::fmt(field, formatter),
        }
    }
}
const CALLER_LIMIT_FIELDS: [super::AdvApp2VarCallerLimitField; 2] = [
    super::AdvApp2VarCallerLimitField::MaximumPieces,
    super::AdvApp2VarCallerLimitField::CoefficientLimit,
];
const CALLER_SAMPLING_FIELDS: [super::AdvApp2VarCallerSamplingField; 2] = [
    super::AdvApp2VarCallerSamplingField::RootCount,
    super::AdvApp2VarCallerSamplingField::JacobiDegree,
];
type IntegerIncidence = ScalarIncidence<FrameIntegerField>;
#[derive(Clone, Copy, Default)]
struct CallerIntegerInputs<'a> {
    subspaces: Option<i32>,
    orders: Option<[i32; 2]>,
    limits: Option<[i32; 2]>,
    sampling: Option<[i32; 2]>,
    dimensions: Option<&'a [i32]>,
}

pub(super) struct IntegerFrame {
    values: Vec<i32>,
    bindings: AdvApp2VarIntegerBindings,
    incidence: Vec<IntegerIncidence>,
    subspace_slot: Option<usize>,
    caller_order_slots: Option<[usize; 2]>,
    caller_limit_slots: Option<[usize; 2]>,
    caller_sampling_slots: Option<[usize; 2]>,
    caller_dimension_slots: Option<Vec<usize>>,
}
impl IntegerFrame {
    pub(super) fn new(
        values: Vec<i32>,
        bindings: AdvApp2VarIntegerBindings,
    ) -> Result<Self, Failure> {
        let incidence = integer_incidence(values.len(), bindings, None, None, None, None, None)?;
        Ok(Self {
            values,
            bindings,
            incidence,
            subspace_slot: None,
            caller_order_slots: None,
            caller_limit_slots: None,
            caller_sampling_slots: None,
            caller_dimension_slots: None,
        })
    }
    pub(super) fn values(&self) -> &[i32] {
        &self.values
    }
    pub(super) fn bind_subspace_count(&mut self, slot: usize) -> Result<(), Failure> {
        let incidence = integer_incidence(
            self.values.len(),
            self.bindings,
            Some(slot),
            self.caller_order_slots,
            self.caller_limit_slots,
            self.caller_sampling_slots,
            self.caller_dimension_slots.as_deref(),
        )?;
        validate_caller_slot("SubspaceCount", slot, self.bindings)?;
        self.incidence = incidence;
        self.subspace_slot = Some(slot);
        Ok(())
    }
    pub(super) fn bind_caller_orders(&mut self, slots: [usize; 2]) -> Result<(), Failure> {
        let incidence = integer_incidence(
            self.values.len(),
            self.bindings,
            self.subspace_slot,
            Some(slots),
            self.caller_limit_slots,
            self.caller_sampling_slots,
            self.caller_dimension_slots.as_deref(),
        )?;
        for field in CALLER_ORDER_FIELDS {
            validate_caller_slot(&format!("{field:?}"), slots[field.index()], self.bindings)?;
        }
        self.incidence = incidence;
        self.caller_order_slots = Some(slots);
        Ok(())
    }
    pub(super) fn bind_caller_limits(&mut self, slots: [usize; 2]) -> Result<(), Failure> {
        let incidence = integer_incidence(
            self.values.len(),
            self.bindings,
            self.subspace_slot,
            self.caller_order_slots,
            Some(slots),
            self.caller_sampling_slots,
            self.caller_dimension_slots.as_deref(),
        )?;
        for field in CALLER_LIMIT_FIELDS {
            validate_caller_slot(&format!("{field:?}"), slots[field.index()], self.bindings)?;
        }
        self.incidence = incidence;
        self.caller_limit_slots = Some(slots);
        Ok(())
    }
    pub(super) fn bind_caller_sampling(&mut self, slots: [usize; 2]) -> Result<(), Failure> {
        let incidence = integer_incidence(
            self.values.len(),
            self.bindings,
            self.subspace_slot,
            self.caller_order_slots,
            self.caller_limit_slots,
            Some(slots),
            self.caller_dimension_slots.as_deref(),
        )?;
        for field in CALLER_SAMPLING_FIELDS {
            validate_caller_slot(&format!("{field:?}"), slots[field.index()], self.bindings)?;
        }
        self.incidence = incidence;
        self.caller_sampling_slots = Some(slots);
        Ok(())
    }
    pub(super) fn bind_caller_dimensions(&mut self, slots: Vec<usize>) -> Result<(), Failure> {
        validate_dimension_slots(&slots, self.bindings)?;
        let incidence = integer_incidence(
            self.values.len(),
            self.bindings,
            self.subspace_slot,
            self.caller_order_slots,
            self.caller_limit_slots,
            self.caller_sampling_slots,
            Some(&slots),
        )?;
        self.incidence = incidence;
        self.caller_dimension_slots = Some(slots);
        Ok(())
    }
    pub(super) fn seed_caller_dimensions(&mut self, dimensions: &[i32]) -> Result<(), Failure> {
        let Some(slots) = &self.caller_dimension_slots else {
            return Ok(());
        };
        if slots.len() != dimensions.len() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarIsoOperationFrame: caller dimensions len={}; expected {} values for bound dimension slots",
                    dimensions.len(),
                    slots.len()
                ),
            ));
        }
        for (slot, value) in slots.iter().zip(dimensions) {
            self.values[*slot] = *value;
        }
        Ok(())
    }
    pub(super) fn caller_dimensions(&self) -> Option<Vec<i32>> {
        self.caller_dimension_slots
            .as_ref()
            .map(|slots| slots.iter().map(|slot| self.values[*slot]).collect())
    }
    pub(super) fn evaluate(
        &mut self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        evaluate: impl FnOnce(&mut AdvApp2VarScalarEvaluation<'_>),
    ) -> Result<(), Failure> {
        self.evaluate_with_subspaces(call, None, |call, _, _| {
            evaluate(call);
            Ok(())
        })
    }
    pub(super) fn evaluate_with_subspaces(
        &mut self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        subspaces: Option<&mut i32>,
        evaluate: impl FnOnce(
            &mut AdvApp2VarScalarEvaluation<'_>,
            Option<&mut i32>,
            Option<usize>,
        ) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.evaluate_with_callers(
            call,
            subspaces,
            None,
            None,
            None,
            |call, spaces, slot, _, _, _, _, _, _, _| evaluate(call, spaces, slot),
        )
    }
    pub(super) fn evaluate_with_callers(
        &mut self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        subspaces: Option<&mut i32>,
        caller_orders: Option<&mut [i32; 2]>,
        caller_limits: Option<&mut [i32; 2]>,
        caller_sampling: Option<&mut [i32; 2]>,
        evaluate: impl FnOnce(
            &mut AdvApp2VarScalarEvaluation<'_>,
            Option<&mut i32>,
            Option<usize>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&[usize]>,
        ) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.evaluate_integer_arguments(
            call,
            subspaces,
            caller_orders,
            caller_limits,
            caller_sampling,
            |call,
             values,
             bindings,
             spaces,
             space_slot,
             orders,
             order_slots,
             limits,
             limit_slots,
             sampling,
             sampling_slots,
             dimensions| {
                evaluate(
                    &mut AdvApp2VarScalarEvaluation::from_call(call, values, bindings),
                    spaces,
                    space_slot,
                    orders,
                    order_slots,
                    limits,
                    limit_slots,
                    sampling,
                    sampling_slots,
                    dimensions,
                )
            },
        )
    }
    pub(super) fn evaluate_integer_arguments<C: IntegerArguments>(
        &mut self,
        call: &mut C,
        mut subspaces: Option<&mut i32>,
        mut caller_orders: Option<&mut [i32; 2]>,
        mut caller_limits: Option<&mut [i32; 2]>,
        mut caller_sampling: Option<&mut [i32; 2]>,
        evaluate: impl FnOnce(
            &mut C,
            &mut [i32],
            AdvApp2VarIntegerBindings,
            Option<&mut i32>,
            Option<usize>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&mut [i32; 2]>,
            Option<[usize; 2]>,
            Option<&[usize]>,
        ) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        validate_order_identity(self.bindings, call.shared_orders())?;
        let dimensions = self.caller_dimensions();
        seed_integer_incidence(
            &mut self.values,
            &self.incidence,
            call,
            CallerIntegerInputs {
                subspaces: subspaces.as_deref().copied(),
                orders: caller_orders.as_deref().copied(),
                limits: caller_limits.as_deref().copied(),
                sampling: caller_sampling.as_deref().copied(),
                dimensions: dimensions.as_deref(),
            },
        )?;
        let outcome = evaluate(
            call,
            &mut self.values,
            self.bindings,
            subspaces.as_deref_mut(),
            self.subspace_slot,
            caller_orders.as_deref_mut(),
            self.caller_order_slots,
            caller_limits.as_deref_mut(),
            self.caller_limit_slots,
            caller_sampling.as_deref_mut(),
            self.caller_sampling_slots,
            self.caller_dimension_slots.as_deref(),
        );
        write_integer_arguments(&self.values, self.bindings, call);
        if let (Some(slot), Some(subspaces)) = (self.subspace_slot, subspaces) {
            *subspaces = self.values[slot];
        }
        if let (Some(slots), Some(orders)) = (self.caller_order_slots, caller_orders) {
            *orders = slots.map(|slot| self.values[slot]);
        }
        if let (Some(slots), Some(limits)) = (self.caller_limit_slots, caller_limits) {
            *limits = slots.map(|slot| self.values[slot]);
        }
        if let (Some(slots), Some(sampling)) = (self.caller_sampling_slots, caller_sampling) {
            *sampling = slots.map(|slot| self.values[slot]);
        }
        outcome
    }
}
fn integer_incidence(
    length: usize,
    bindings: AdvApp2VarIntegerBindings,
    subspace_slot: Option<usize>,
    caller_order_slots: Option<[usize; 2]>,
    caller_limit_slots: Option<[usize; 2]>,
    caller_sampling_slots: Option<[usize; 2]>,
    caller_dimension_slots: Option<&[usize]>,
) -> Result<Vec<IntegerIncidence>, Failure> {
    scalar_incidence(
        INTEGER_FIELDS
            .map(|field| (FrameIntegerField::Callback(field), bindings.slot(field)))
            .into_iter()
            .chain(subspace_slot.map(|slot| (FrameIntegerField::SubspaceCount, slot)))
            .chain(caller_order_slots.into_iter().flat_map(|slots| {
                CALLER_ORDER_FIELDS
                    .map(|field| (FrameIntegerField::CallerOrder(field), slots[field.index()]))
            }))
            .chain(caller_limit_slots.into_iter().flat_map(|slots| {
                CALLER_LIMIT_FIELDS
                    .map(|field| (FrameIntegerField::CallerLimit(field), slots[field.index()]))
            }))
            .chain(caller_sampling_slots.into_iter().flat_map(|slots| {
                CALLER_SAMPLING_FIELDS.map(|field| {
                    (
                        FrameIntegerField::CallerSampling(field),
                        slots[field.index()],
                    )
                })
            }))
            .chain(caller_dimension_slots.into_iter().flat_map(|slots| {
                slots
                    .iter()
                    .enumerate()
                    .map(|(index, slot)| (FrameIntegerField::CallerDimension(index), *slot))
            })),
        length,
        "AdvApp2VarScalarProvider",
    )
}
fn validate_order_identity(
    bindings: AdvApp2VarIntegerBindings,
    expected_shared: bool,
) -> Result<(), Failure> {
    if (bindings.u_order == bindings.v_order) != expected_shared {
        return Err(Failure::with_message(
            "Standard_DomainError",
            &format!(
                "AdvApp2VarScalarProvider: derivative slots=({},{}); expected shared={expected_shared}",
                bindings.u_order, bindings.v_order
            ),
        ));
    }
    Ok(())
}
fn seed_integer_incidence(
    values: &mut [i32],
    incidence: &[IntegerIncidence],
    call: &impl IntegerArguments,
    inputs: CallerIntegerInputs<'_>,
) -> Result<(), Failure> {
    for group in incidence {
        values[group.slot] = incident_initial_value(group, call, inputs)?;
    }
    Ok(())
}
fn incident_initial_value(
    group: &IntegerIncidence,
    call: &impl IntegerArguments,
    inputs: CallerIntegerInputs<'_>,
) -> Result<i32, Failure> {
    let first = frame_integer_value(call, group.fields[0], inputs)?;
    for field in &group.fields[1..] {
        let value = frame_integer_value(call, *field, inputs)?;
        if value != first {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarScalarProvider: slot={} incident {field:?} value={value}; expected shared input value={first}",
                    group.slot
                ),
            ));
        }
    }
    Ok(first)
}

fn write_integer_arguments(
    values: &[i32],
    bindings: AdvApp2VarIntegerBindings,
    call: &mut impl IntegerArguments,
) {
    for field in INTEGER_FIELDS {
        set_argument_value(call, field, values[bindings.slot(field)]);
    }
}

fn frame_integer_value(
    call: &impl IntegerArguments,
    field: FrameIntegerField,
    inputs: CallerIntegerInputs<'_>,
) -> Result<i32, Failure> {
    match field {
        FrameIntegerField::Callback(field) => Ok(argument_value(call, field)),
        FrameIntegerField::CallerDimension(index) => inputs.dimensions.and_then(|dimensions| dimensions.get(index).copied()).ok_or_else(|| missing_caller_dimension(index)),
        FrameIntegerField::CallerSampling(field) => inputs.sampling.map(|sampling|sampling[field.index()]).ok_or_else(super::caller_sampling::missing_caller_sampling_context),
        FrameIntegerField::CallerLimit(field) => inputs.limits.map(|limits|limits[field.index()]).ok_or_else(super::caller_limits::missing_caller_limit_context),
        FrameIntegerField::CallerOrder(field) => inputs.orders.map(|orders|orders[field.index()]).ok_or_else(super::caller_orders::missing_caller_order_context),
        FrameIntegerField::SubspaceCount => inputs.subspaces.ok_or_else(|| Failure::with_message("Standard_DomainError", "AdvApp2VarIsoOperationFrame: subspace context=None; expected caller subspace count during iso evaluation")),
    }
}

fn validate_caller_slot(
    field: &str,
    slot: usize,
    bindings: AdvApp2VarIntegerBindings,
) -> Result<(), Failure> {
    if ![bindings.parameter_count, bindings.u_order, bindings.v_order].contains(&slot) {
        return Ok(());
    }
    Err(Failure::with_message(
        "Standard_DomainError",
        &format!(
            "AdvApp2VarIsoOperationFrame: caller {field} slot={slot} names a callback local; expected a forwarded caller argument slot or a distinct owned caller slot"
        ),
    ))
}

pub(super) fn missing_caller_dimension(index: usize) -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        &format!(
            "AdvApp2VarIsoOperationFrame: caller dimension index={index} unavailable; expected configured dimension slots containing that index"
        ),
    )
}
fn validate_dimension_slots(
    slots: &[usize],
    bindings: AdvApp2VarIntegerBindings,
) -> Result<(), Failure> {
    let unique: std::collections::BTreeSet<usize> = slots.iter().copied().collect();
    if slots.is_empty() || unique.len() != slots.len() {
        return Err(Failure::with_message(
            "Standard_DomainError",
            &format!(
                "AdvApp2VarIsoOperationFrame: caller dimension slots={slots:?}; expected nonempty distinct array entry slots"
            ),
        ));
    }
    for (index, slot) in slots.iter().enumerate() {
        validate_caller_slot(&format!("SubspaceDimension({index})"), *slot, bindings)?;
    }
    Ok(())
}

#[path = "iso_coefficient_slots.rs"]
mod coefficient_slots;
pub(super) use coefficient_slots::coefficient_slot;
