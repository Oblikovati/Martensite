//! Owned caller real fields, distinct from copied callback piece bounds.
#[path = "real_curve_output.rs"]
mod curve_output;
#[path = "real_cut_output.rs"]
mod cut_output;
pub(in crate::adv_app2var) use cut_output::{checked_real_range, checked_real_slot};
#[path = "real_folded_output.rs"]
mod folded_output;
#[path = "real_inputs.rs"]
mod inputs;
#[path = "caller_real_storage.rs"]
mod storage;
use super::approx_f2var::IsoApproxRequest;
use super::scalar_incidence::{ScalarIncidence, scalar_incidence};
pub use inputs::AdvApp2VarCallerRealArray;
use martensite_tkernel::standard::Failure;
pub use storage::{
    AdvApp2VarCallerCurveSlots, AdvApp2VarEndpointOutput, AdvApp2VarErrorOutput,
    AdvApp2VarFoldedOutput,
};
pub(crate) use storage::{
    CallerRealTableRole, OwnedCallerTableSlots, OwnedEndpointSlots, OwnedErrorSlots,
    OwnedFoldedSlots,
};

/// A real caller field in an iso approximation; `AdvApp2VarRealField::ConstantParameter`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarRealField {
    /// First U domain endpoint.
    UStart,
    /// Last U domain endpoint.
    UEnd,
    /// First V domain endpoint.
    VStart,
    /// Last V domain endpoint.
    VEnd,
    /// Caller constant parameter.
    ConstantParameter,
}
const REAL_FIELDS: [AdvApp2VarRealField; 5] = [
    AdvApp2VarRealField::UStart,
    AdvApp2VarRealField::UEnd,
    AdvApp2VarRealField::VStart,
    AdvApp2VarRealField::VEnd,
    AdvApp2VarRealField::ConstantParameter,
];

// Logical caller identity is independent of the owned value's physical slot.
#[derive(Clone, Copy)]
enum CallerRealField {
    Scalar(AdvApp2VarRealField),
    Array(AdvApp2VarCallerRealArray, usize),
    Curve(usize),
}
impl std::fmt::Debug for CallerRealField {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(field) => std::fmt::Debug::fmt(field, formatter),
            Self::Array(array, index) => write!(formatter, "{array:?}[{index}]"),
            Self::Curve(index) => write!(formatter, "CallerCurve[{index}]"),
        }
    }
}
impl CallerRealField {
    fn request_value(self, request: &IsoApproxRequest<'_>, entry_value: f64) -> f64 {
        match self {
            Self::Scalar(field) => request_real(request, field),
            Self::Array(array, index) => array.request_values(request)[index],
            Self::Curve(_) => entry_value,
        }
    }
}

/// Caller field-to-real-slot incidence; equal indices share one owned value.
/// `AdvApp2VarRealBindings { domain: [0,1,2,3], constant_parameter: 0 }` shares U start and constant.
#[derive(Clone, Copy, Debug)]
pub struct AdvApp2VarRealBindings {
    /// Slots for U start, U end, V start and V end.
    pub domain: [usize; 4],
    /// Slot for the constant parameter.
    pub constant_parameter: usize,
}
impl AdvApp2VarRealBindings {
    fn slot(self, field: AdvApp2VarRealField) -> usize {
        match field {
            AdvApp2VarRealField::UStart => self.domain[0],
            AdvApp2VarRealField::UEnd => self.domain[1],
            AdvApp2VarRealField::VStart => self.domain[2],
            AdvApp2VarRealField::VEnd => self.domain[3],
            AdvApp2VarRealField::ConstantParameter => self.constant_parameter,
        }
    }
}

/// Operation-local owned real storage. Each approximation seeds it from caller
/// fields once, rejects inconsistent shared inputs and preserves live incidence.
/// `AdvApp2VarIsoRealFrame::new(vec![0.;4], bindings)?`.
pub struct AdvApp2VarIsoRealFrame {
    inputs: inputs::OwnedCallerRealArrays,
    curve_region: Option<std::ops::Range<usize>>,
    values: storage::CallerRealSlots,
    bindings: AdvApp2VarRealBindings,
    incidence: Vec<ScalarIncidence<CallerRealField>>,
}
impl AdvApp2VarIsoRealFrame {
    /// Validate field indices and own real slots; `AdvApp2VarIsoRealFrame::new(vec![0.;5], bindings)?`.
    pub fn new(values: Vec<f64>, bindings: AdvApp2VarRealBindings) -> Result<Self, Failure> {
        let incidence = real_incidence(values.len(), bindings, &[None, None], None)?;
        validate_domain_identity(&incidence)?;
        Ok(Self {
            inputs: Default::default(),
            curve_region: None,
            values: storage::CallerRealSlots::new(values),
            bindings,
            incidence,
        })
    }
    /// Read a field's current shared value; `frame.real(AdvApp2VarRealField::UStart)`.
    pub fn real(&self, field: AdvApp2VarRealField) -> f64 {
        self.values.value(self.bindings.slot(field))
    }
    /// Write through one field's slot; `frame.set_real(AdvApp2VarRealField::UStart, .25)`.
    pub fn set_real(&mut self, field: AdvApp2VarRealField, value: f64) {
        self.values.set_value(self.bindings.slot(field), value);
    }
    pub(super) fn seed_request(&mut self, request: &IsoApproxRequest<'_>) -> Result<(), Failure> {
        self.inputs.validate_slot_shapes(request)?;
        self.values
            .with_values_mut(|values| -> Result<(), Failure> {
                for group in &self.incidence {
                    values[group.slot] = initial_incident_real(group, request, values[group.slot])?;
                }
                Ok(())
            })?;
        self.inputs.seed_request(request);
        Ok(())
    }
    pub(super) fn caller_domain(&self) -> [f64; 4] {
        self.values
            .with_values(|values| self.bindings.domain.map(|slot| values[slot]))
    }
    pub(super) fn write_request(&self, request: &mut IsoApproxRequest<'_>) {
        request.uvfonc = self.caller_domain();
        request.tconst = self.real(AdvApp2VarRealField::ConstantParameter);
    }
}
fn request_real(request: &IsoApproxRequest<'_>, field: AdvApp2VarRealField) -> f64 {
    match field {
        AdvApp2VarRealField::UStart => request.uvfonc[0],
        AdvApp2VarRealField::UEnd => request.uvfonc[1],
        AdvApp2VarRealField::VStart => request.uvfonc[2],
        AdvApp2VarRealField::VEnd => request.uvfonc[3],
        AdvApp2VarRealField::ConstantParameter => request.tconst,
    }
}
fn initial_incident_real(
    group: &ScalarIncidence<CallerRealField>,
    request: &IsoApproxRequest<'_>,
    entry_value: f64,
) -> Result<f64, Failure> {
    let first = group.fields[0].request_value(request, entry_value);
    for field in &group.fields[1..] {
        let value = field.request_value(request, entry_value);
        if value.to_bits() != first.to_bits() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarIsoRealFrame: slot={} incident {field:?} value={value:?} bits={:016x}; expected shared input value={first:?} bits={:016x}",
                    group.slot,
                    value.to_bits(),
                    first.to_bits()
                ),
            ));
        }
    }
    Ok(first)
}

fn validate_domain_identity(incidence: &[ScalarIncidence<CallerRealField>]) -> Result<(), Failure> {
    for group in incidence {
        let endpoints = group
            .fields
            .iter()
            .filter(|field| matches!(field, CallerRealField::Scalar(value) if *value != AdvApp2VarRealField::ConstantParameter))
            .count();
        if endpoints > 1 {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarIsoRealFrame: slot={} domain endpoint count={endpoints}; expected distinct slots for four contiguous caller endpoints",
                    group.slot
                ),
            ));
        }
    }
    Ok(())
}

fn real_incidence(
    length: usize,
    bindings: AdvApp2VarRealBindings,
    arrays: &[Option<Vec<usize>>; 2],
    curve: Option<std::ops::Range<usize>>,
) -> Result<Vec<ScalarIncidence<CallerRealField>>, Failure> {
    let fields: Vec<_> = REAL_FIELDS
        .map(|field| (CallerRealField::Scalar(field), bindings.slot(field)))
        .into_iter()
        .chain(inputs::mapped_real_fields(arrays))
        .collect();
    let curve_fields = curve_incident_fields(&fields, curve);
    scalar_incidence(
        fields.into_iter().chain(curve_fields),
        length,
        "AdvApp2VarIsoRealFrame",
    )
}
fn curve_incident_fields(
    fields: &[(CallerRealField, usize)],
    region: Option<std::ops::Range<usize>>,
) -> Vec<(CallerRealField, usize)> {
    let Some(region) = region else {
        return Vec::new();
    };
    // Output-only slots retain their supplied values; only shared identities need seed validation.
    let slots: std::collections::BTreeSet<usize> = fields
        .iter()
        .map(|(_, slot)| *slot)
        .filter(|slot| region.contains(slot))
        .collect();
    slots
        .into_iter()
        .map(|slot| (CallerRealField::Curve(slot - region.start), slot))
        .collect()
}
